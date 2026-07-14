//! Engine-neutral, capability-free universe identity reference contract.

use std::collections::BTreeMap;

use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use minicbor::{Decoder, Encoder};
use serde::Serialize;
use sha2::{Digest, Sha256};

pub const IDENTITY_SCHEMA_VERSION: u16 = 1;
pub const MAX_PATH_DEPTH: usize = 6;
pub const MAX_SEGMENT_BYTES: usize = 64;
pub const MAX_LABEL_BYTES: usize = 64;

const LOGICAL_DOMAIN: &[u8] = b"mindwarp/universe-identity/logical/v1\0";
const RECONSTRUCTION_DOMAIN: &[u8] = b"mindwarp/universe-identity/reconstruction/v1\0";
const STREAM_SALT: &[u8] = b"mindwarp/universe-identity/stream-key/v1";
const COUNTER_DOMAIN: &[u8] = b"mindwarp/universe-identity/counter-block/v1\0";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[repr(u16)]
pub enum NodeKind {
    Galaxy = 1,
    StarSystem = 2,
    Body = 3,
    Region = 4,
    Site = 5,
    Entity = 6,
}

impl TryFrom<u16> for NodeKind {
    type Error = IdentityError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Galaxy),
            2 => Ok(Self::StarSystem),
            3 => Ok(Self::Body),
            4 => Ok(Self::Region),
            5 => Ok(Self::Site),
            6 => Ok(Self::Entity),
            _ => Err(IdentityError::UnknownNodeTag(value)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AddressSegment {
    pub kind: NodeKind,
    pub payload: Vec<u8>,
}

impl AddressSegment {
    pub fn new(kind: NodeKind, payload: impl Into<Vec<u8>>) -> Result<Self, IdentityError> {
        let payload = payload.into();
        if payload.is_empty() || payload.len() > MAX_SEGMENT_BYTES {
            return Err(IdentityError::InvalidSegmentLength(payload.len()));
        }
        Ok(Self { kind, payload })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UniverseAddress {
    pub universe_seed: [u8; 32],
    pub path: Vec<AddressSegment>,
}

impl UniverseAddress {
    pub fn new(universe_seed: [u8; 32], path: Vec<AddressSegment>) -> Result<Self, IdentityError> {
        if path.len() > MAX_PATH_DEPTH {
            return Err(IdentityError::PathTooDeep(path.len()));
        }
        let mut previous = 0;
        for segment in &path {
            let tag = segment.kind as u16;
            if segment.payload.is_empty() || segment.payload.len() > MAX_SEGMENT_BYTES {
                return Err(IdentityError::InvalidSegmentLength(segment.payload.len()));
            }
            if tag <= previous {
                return Err(IdentityError::InvalidHierarchyOrder);
            }
            previous = tag;
        }
        Ok(Self {
            universe_seed,
            path,
        })
    }

    pub fn encode_canonical(&self) -> Result<Vec<u8>, IdentityError> {
        let mut encoder = Encoder::new(Vec::new());
        encoder
            .array(3)
            .and_then(|encoder| encoder.u16(IDENTITY_SCHEMA_VERSION))
            .and_then(|encoder| encoder.bytes(&self.universe_seed))
            .and_then(|encoder| encoder.array(self.path.len() as u64))
            .map_err(encode_error)?;
        for segment in &self.path {
            encoder
                .array(2)
                .and_then(|encoder| encoder.u16(segment.kind as u16))
                .and_then(|encoder| encoder.bytes(&segment.payload))
                .map_err(encode_error)?;
        }
        Ok(encoder.into_writer())
    }

    pub fn decode_canonical(bytes: &[u8]) -> Result<Self, IdentityError> {
        let mut decoder = Decoder::new(bytes);
        if decoder.array().map_err(decode_error)? != Some(3) {
            return Err(IdentityError::NonCanonicalEncoding);
        }
        let schema = decoder.u16().map_err(decode_error)?;
        if schema != IDENTITY_SCHEMA_VERSION {
            return Err(IdentityError::UnsupportedSchema(schema));
        }
        let seed_bytes = decoder.bytes().map_err(decode_error)?;
        let universe_seed: [u8; 32] = seed_bytes
            .try_into()
            .map_err(|_| IdentityError::InvalidSeedLength(seed_bytes.len()))?;
        let Some(path_len) = decoder.array().map_err(decode_error)? else {
            return Err(IdentityError::NonCanonicalEncoding);
        };
        let path_len =
            usize::try_from(path_len).map_err(|_| IdentityError::PathTooDeep(usize::MAX))?;
        if path_len > MAX_PATH_DEPTH {
            return Err(IdentityError::PathTooDeep(path_len));
        }
        let mut path = Vec::with_capacity(path_len);
        for _ in 0..path_len {
            if decoder.array().map_err(decode_error)? != Some(2) {
                return Err(IdentityError::NonCanonicalEncoding);
            }
            let kind = NodeKind::try_from(decoder.u16().map_err(decode_error)?)?;
            let payload = decoder.bytes().map_err(decode_error)?.to_vec();
            path.push(AddressSegment::new(kind, payload)?);
        }
        if decoder.position() != bytes.len() {
            return Err(IdentityError::TrailingBytes);
        }
        let address = Self::new(universe_seed, path)?;
        if address.encode_canonical()? != bytes {
            return Err(IdentityError::NonCanonicalEncoding);
        }
        Ok(address)
    }

    pub fn logical_fingerprint(&self) -> Result<[u8; 32], IdentityError> {
        Ok(domain_hash(LOGICAL_DOMAIN, &self.encode_canonical()?))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct GeneratorVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl GeneratorVersion {
    pub fn new(major: u16, minor: u16, patch: u16) -> Result<Self, IdentityError> {
        if (major, minor, patch) == (0, 0, 0) {
            return Err(IdentityError::InvalidGeneratorVersion);
        }
        Ok(Self {
            major,
            minor,
            patch,
        })
    }

    fn bytes(self) -> [u8; 6] {
        let mut bytes = [0; 6];
        bytes[0..2].copy_from_slice(&self.major.to_be_bytes());
        bytes[2..4].copy_from_slice(&self.minor.to_be_bytes());
        bytes[4..6].copy_from_slice(&self.patch.to_be_bytes());
        bytes
    }
}

pub fn reconstruction_fingerprint(
    address: &UniverseAddress,
    version: GeneratorVersion,
    derivation_contract: &str,
) -> Result<[u8; 32], IdentityError> {
    validate_label(derivation_contract)?;
    let mut preimage = Vec::new();
    preimage.extend_from_slice(&address.logical_fingerprint()?);
    preimage.extend_from_slice(&version.bytes());
    append_bounded(&mut preimage, derivation_contract.as_bytes())?;
    Ok(domain_hash(RECONSTRUCTION_DOMAIN, &preimage))
}

pub fn derive_stream_key(
    address: &UniverseAddress,
    version: GeneratorVersion,
    derivation_contract: &str,
    stream_label: &str,
) -> Result<[u8; 32], IdentityError> {
    validate_label(derivation_contract)?;
    validate_label(stream_label)?;
    let canonical = address.encode_canonical()?;
    let mut info = Vec::new();
    append_bounded(&mut info, &canonical)?;
    info.extend_from_slice(&version.bytes());
    append_bounded(&mut info, derivation_contract.as_bytes())?;
    append_bounded(&mut info, stream_label.as_bytes())?;
    let hkdf = Hkdf::<Sha256>::new(Some(STREAM_SALT), &address.universe_seed);
    let mut key = [0; 32];
    hkdf.expand(&info, &mut key)
        .map_err(|_| IdentityError::DerivationFailure)?;
    Ok(key)
}

pub fn counter_block(stream_key: &[u8; 32], counter: u64) -> [u8; 32] {
    let mut mac = Hmac::<Sha256>::new_from_slice(stream_key).expect("HMAC accepts 32-byte keys");
    mac.update(COUNTER_DOMAIN);
    mac.update(&counter.to_be_bytes());
    mac.finalize().into_bytes().into()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MigrationReceipt {
    pub logical_fingerprint: String,
    pub from_version: GeneratorVersion,
    pub to_version: GeneratorVersion,
    pub from_reconstruction: String,
    pub to_reconstruction: String,
}

pub fn migration_receipt(
    address: &UniverseAddress,
    from_version: GeneratorVersion,
    to_version: GeneratorVersion,
    derivation_contract: &str,
) -> Result<MigrationReceipt, IdentityError> {
    if from_version == to_version {
        return Err(IdentityError::NoVersionChange);
    }
    Ok(MigrationReceipt {
        logical_fingerprint: hex(&address.logical_fingerprint()?),
        from_version,
        to_version,
        from_reconstruction: hex(&reconstruction_fingerprint(
            address,
            from_version,
            derivation_contract,
        )?),
        to_reconstruction: hex(&reconstruction_fingerprint(
            address,
            to_version,
            derivation_contract,
        )?),
    })
}

#[derive(Default)]
pub struct IdentityIndex {
    entries: BTreeMap<[u8; 32], Vec<u8>>,
}

impl IdentityIndex {
    pub fn admit(&mut self, address: &UniverseAddress) -> Result<[u8; 32], IdentityError> {
        let canonical = address.encode_canonical()?;
        let fingerprint = address.logical_fingerprint()?;
        self.admit_fingerprint(fingerprint, canonical)?;
        Ok(fingerprint)
    }

    fn admit_fingerprint(
        &mut self,
        fingerprint: [u8; 32],
        canonical: Vec<u8>,
    ) -> Result<(), IdentityError> {
        if let Some(existing) = self.entries.get(&fingerprint) {
            return if existing == &canonical {
                Ok(())
            } else {
                Err(IdentityError::FingerprintCollision(hex(&fingerprint)))
            };
        }
        self.entries.insert(fingerprint, canonical);
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProofVectorEvidence {
    pub schema_version: u16,
    pub system_id: String,
    pub proof_id: String,
    pub fixture_id: String,
    pub contract_version: String,
    pub generator_version: GeneratorVersion,
    pub logical_fingerprint: String,
    pub reconstruction_fingerprint: String,
    pub stream_label: String,
    pub stream_key: String,
    pub counter_blocks: Vec<(u64, String)>,
    pub equivalence_method: String,
    pub measurement_classification: String,
    pub limitations: Vec<String>,
}

pub fn proof_vector_evidence(
    address: &UniverseAddress,
    version: GeneratorVersion,
    derivation_contract: &str,
    stream_label: &str,
    counters: &[u64],
) -> Result<ProofVectorEvidence, IdentityError> {
    let stream_key = derive_stream_key(address, version, derivation_contract, stream_label)?;
    Ok(ProofVectorEvidence {
        schema_version: 1,
        system_id: "universe-identity".into(),
        proof_id: "fixed-vector".into(),
        fixture_id: "universe-identity-v1/core".into(),
        contract_version: "universe-identity-v1".into(),
        generator_version: version,
        logical_fingerprint: hex(&address.logical_fingerprint()?),
        reconstruction_fingerprint: hex(&reconstruction_fingerprint(
            address,
            version,
            derivation_contract,
        )?),
        stream_label: stream_label.into(),
        stream_key: hex(&stream_key),
        counter_blocks: counters
            .iter()
            .map(|counter| (*counter, hex(&counter_block(&stream_key, *counter))))
            .collect(),
        equivalence_method: "byte-exact-sha256".into(),
        measurement_classification: "simulated".into(),
        limitations: vec![
            "Reference determinism fixture; not a bulk-generation benchmark.".into(),
            "Evidence grants no approval, promotion, or runtime authority.".into(),
        ],
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityError {
    UnsupportedSchema(u16),
    InvalidSeedLength(usize),
    UnknownNodeTag(u16),
    InvalidSegmentLength(usize),
    PathTooDeep(usize),
    InvalidHierarchyOrder,
    InvalidGeneratorVersion,
    InvalidLabel,
    NonCanonicalEncoding,
    TrailingBytes,
    DerivationFailure,
    NoVersionChange,
    FingerprintCollision(String),
    Codec(String),
}

fn domain_hash(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(bytes);
    hasher.finalize().into()
}

fn validate_label(value: &str) -> Result<(), IdentityError> {
    if value.is_empty()
        || value.len() > MAX_LABEL_BYTES
        || !value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"-_.".contains(&byte)
        })
    {
        return Err(IdentityError::InvalidLabel);
    }
    Ok(())
}

fn append_bounded(target: &mut Vec<u8>, value: &[u8]) -> Result<(), IdentityError> {
    let length = u16::try_from(value.len()).map_err(|_| IdentityError::InvalidLabel)?;
    target.extend_from_slice(&length.to_be_bytes());
    target.extend_from_slice(value);
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    output
}

fn encode_error<E: std::fmt::Debug>(error: minicbor::encode::Error<E>) -> IdentityError {
    IdentityError::Codec(format!("{error:?}"))
}

fn decode_error(error: minicbor::decode::Error) -> IdentityError {
    IdentityError::Codec(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn address(seed: u8, sibling: u8) -> UniverseAddress {
        UniverseAddress::new(
            [seed; 32],
            vec![
                AddressSegment::new(NodeKind::Galaxy, vec![0]).unwrap(),
                AddressSegment::new(NodeKind::StarSystem, vec![sibling]).unwrap(),
            ],
        )
        .unwrap()
    }

    #[test]
    fn fixed_vectors_are_byte_exact_across_fresh_construction() {
        let root = UniverseAddress::new([0; 32], vec![]).unwrap();
        let version = GeneratorVersion::new(1, 0, 0).unwrap();
        let encoded = root.encode_canonical().unwrap();
        let logical = root.logical_fingerprint().unwrap();
        let reconstruction = reconstruction_fingerprint(&root, version, "identity-v1").unwrap();
        let key = derive_stream_key(&root, version, "identity-v1", "terrain").unwrap();
        assert_eq!(
            hex(&encoded),
            "83015820000000000000000000000000000000000000000000000000000000000000000080"
        );
        assert_eq!(
            hex(&logical),
            "3a24b755bcc0ec0fd99846b322742e5b3f7a783442b63f43459e7a61c12479f1"
        );
        assert_eq!(
            hex(&reconstruction),
            "fc2b37dd23570eb86a63fb4bb3b64f2fa4400a4c2dc88495750dfc3b2c51355a"
        );
        assert_eq!(
            hex(&key),
            "b54559be9921c3fdd6d6fd4b0a51308da69cc1ea1564ff803bad199bfd34fb15"
        );
        assert_eq!(
            hex(&counter_block(&key, 0)),
            "f501db3c4f0bb4039599aadefe3585490da8e8868b7d12ff8dfa39c7250642db"
        );
        assert_eq!(
            hex(&counter_block(&key, 1)),
            "99967823449dcfb0d36761b018e064d98ba3ebd5a11fabf73181aa5f38380bb3"
        );
        assert_eq!(
            hex(&counter_block(&key, u64::MAX)),
            "760e09af8a972717146119f01fb881966960b9d013f1d66862a6fcc577a44852"
        );
        assert_eq!(UniverseAddress::decode_canonical(&encoded).unwrap(), root);
    }

    #[test]
    fn siblings_versions_and_labels_partition_without_identity_drift() {
        let left = address(7, 1);
        let right = address(7, 2);
        assert_ne!(
            left.logical_fingerprint().unwrap(),
            right.logical_fingerprint().unwrap()
        );
        let v1 = GeneratorVersion::new(1, 0, 0).unwrap();
        let v2 = GeneratorVersion::new(2, 0, 0).unwrap();
        assert_ne!(
            reconstruction_fingerprint(&left, v1, "identity-v1").unwrap(),
            reconstruction_fingerprint(&left, v2, "identity-v1").unwrap()
        );
        assert_eq!(
            left.logical_fingerprint().unwrap(),
            left.logical_fingerprint().unwrap()
        );
        let terrain = derive_stream_key(&left, v1, "identity-v1", "terrain").unwrap();
        let ecology = derive_stream_key(&left, v1, "identity-v1", "ecology").unwrap();
        assert_ne!(terrain, ecology);
        assert_ne!(counter_block(&terrain, 0), counter_block(&terrain, 1));
    }

    #[test]
    fn migration_preserves_logical_identity_and_rejects_noop() {
        let address = address(9, 3);
        let v1 = GeneratorVersion::new(1, 0, 0).unwrap();
        let v2 = GeneratorVersion::new(2, 0, 0).unwrap();
        let receipt = migration_receipt(&address, v1, v2, "identity-v1").unwrap();
        assert_eq!(
            receipt.logical_fingerprint,
            hex(&address.logical_fingerprint().unwrap())
        );
        assert_ne!(receipt.from_reconstruction, receipt.to_reconstruction);
        assert_eq!(
            migration_receipt(&address, v1, v1, "identity-v1"),
            Err(IdentityError::NoVersionChange)
        );
    }

    #[test]
    fn strict_codec_rejects_noncanonical_unknown_and_trailing_inputs() {
        let address = address(1, 1);
        let canonical = address.encode_canonical().unwrap();
        let mut trailing = canonical.clone();
        trailing.push(0);
        assert_eq!(
            UniverseAddress::decode_canonical(&trailing),
            Err(IdentityError::TrailingBytes)
        );

        let mut non_minimal = vec![0x83, 0x18, 0x01];
        non_minimal.extend_from_slice(&canonical[2..]);
        assert_eq!(
            UniverseAddress::decode_canonical(&non_minimal),
            Err(IdentityError::NonCanonicalEncoding)
        );

        let mut indefinite = canonical.clone();
        indefinite[0] = 0x9f;
        indefinite.push(0xff);
        assert_eq!(
            UniverseAddress::decode_canonical(&indefinite),
            Err(IdentityError::NonCanonicalEncoding)
        );

        let unsupported = [0x83, 0x02, 0x58, 0x20];
        assert_eq!(
            UniverseAddress::decode_canonical(&unsupported),
            Err(IdentityError::UnsupportedSchema(2))
        );

        for forbidden_shape in [
            vec![0x83, 0x01, 0xa0],
            vec![0x83, 0x01, 0xf9, 0x3c, 0x00],
            vec![0x83, 0x01, 0x61, b'x'],
        ] {
            assert!(UniverseAddress::decode_canonical(&forbidden_shape).is_err());
        }

        let mut encoder = Encoder::new(Vec::new());
        encoder
            .array(3)
            .unwrap()
            .u16(1)
            .unwrap()
            .bytes(&[1; 32])
            .unwrap()
            .array(1)
            .unwrap();
        encoder
            .array(2)
            .unwrap()
            .u16(99)
            .unwrap()
            .bytes(&[1])
            .unwrap();
        assert_eq!(
            UniverseAddress::decode_canonical(&encoder.into_writer()),
            Err(IdentityError::UnknownNodeTag(99))
        );
    }

    #[test]
    fn hierarchy_bounds_and_labels_fail_closed() {
        let duplicate = UniverseAddress::new(
            [0; 32],
            vec![
                AddressSegment::new(NodeKind::Galaxy, vec![1]).unwrap(),
                AddressSegment::new(NodeKind::Galaxy, vec![2]).unwrap(),
            ],
        );
        assert_eq!(duplicate, Err(IdentityError::InvalidHierarchyOrder));
        assert_eq!(
            AddressSegment::new(NodeKind::Galaxy, vec![]),
            Err(IdentityError::InvalidSegmentLength(0))
        );
        assert_eq!(
            derive_stream_key(
                &address(1, 1),
                GeneratorVersion::new(1, 0, 0).unwrap(),
                "identity-v1",
                "Terrain With Spaces"
            ),
            Err(IdentityError::InvalidLabel)
        );

        let maximum = UniverseAddress::new(
            [3; 32],
            vec![
                AddressSegment::new(NodeKind::Galaxy, vec![1]).unwrap(),
                AddressSegment::new(NodeKind::StarSystem, vec![2]).unwrap(),
                AddressSegment::new(NodeKind::Body, vec![3]).unwrap(),
                AddressSegment::new(NodeKind::Region, vec![4]).unwrap(),
                AddressSegment::new(NodeKind::Site, vec![5]).unwrap(),
                AddressSegment::new(NodeKind::Entity, vec![6]).unwrap(),
            ],
        )
        .unwrap();
        assert_eq!(
            UniverseAddress::decode_canonical(&maximum.encode_canonical().unwrap()).unwrap(),
            maximum
        );

        let mut too_deep = Encoder::new(Vec::new());
        too_deep
            .array(3)
            .unwrap()
            .u16(1)
            .unwrap()
            .bytes(&[0; 32])
            .unwrap()
            .array((MAX_PATH_DEPTH + 1) as u64)
            .unwrap();
        assert_eq!(
            UniverseAddress::decode_canonical(&too_deep.into_writer()),
            Err(IdentityError::PathTooDeep(MAX_PATH_DEPTH + 1))
        );
    }

    #[test]
    fn injected_collision_is_diagnostic_and_equal_retry_is_idempotent() {
        let left = address(1, 1);
        let right = address(1, 2);
        let mut index = IdentityIndex::default();
        let fingerprint = index.admit(&left).unwrap();
        index.admit(&left).unwrap();
        assert_eq!(
            index.admit_fingerprint(fingerprint, right.encode_canonical().unwrap()),
            Err(IdentityError::FingerprintCollision(hex(&fingerprint)))
        );
    }

    #[test]
    fn proof_evidence_is_bounded_and_authority_negative() {
        let evidence = proof_vector_evidence(
            &address(4, 1),
            GeneratorVersion::new(1, 0, 0).unwrap(),
            "identity-v1",
            "terrain",
            &[0, 1, u64::MAX],
        )
        .unwrap();
        assert_eq!(evidence.system_id, "universe-identity");
        assert_eq!(evidence.counter_blocks.len(), 3);
        assert_eq!(evidence.measurement_classification, "simulated");
        let json = serde_json::to_string(&evidence).unwrap();
        for forbidden in ["approved", "promoted", "authority_grant", "runtime_engine"] {
            assert!(!json.contains(forbidden));
        }
    }
}
