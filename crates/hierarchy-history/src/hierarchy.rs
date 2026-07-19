use minicbor::{Decoder, Encoder};
use serde::Serialize;

use crate::{CONTRACT_VERSION, HierarchyHistoryError, codec, hash};

const DESCRIPTOR_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/descriptor/v1\0";
const CHILD_LOGICAL_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/fixture-child/v1\0";
const CHILD_DESCRIPTOR_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/fixture-child-descriptor/v1\0";
const DYNAMIC_INSTANCE_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/dynamic-instance/v1\0";
const ADDRESS_PRESENCE_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/address-presence/v1\0";
const MAX_RECIPE_BYTES: usize = 4096;
const MAX_PRESENCE_BYTES: usize = 256;
pub const MAX_CHILD_WINDOW: u16 = 256;

pub fn dynamic_instance_logical_id(
    parent_logical_id: [u8; 32],
    stable_instance_id: [u8; 32],
) -> Result<[u8; 32], HierarchyHistoryError> {
    if parent_logical_id == [0; 32] || stable_instance_id == [0; 32] {
        return Err(HierarchyHistoryError::Invalid("dynamic instance identity"));
    }
    let mut preimage = [0_u8; 64];
    preimage[..32].copy_from_slice(&parent_logical_id);
    preimage[32..].copy_from_slice(&stable_instance_id);
    Ok(hash(DYNAMIC_INSTANCE_DOMAIN, &preimage))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum AddressPresence {
    NeverObserved,
    Absent {
        address_fingerprint: [u8; 32],
    },
    Present {
        descriptor_fingerprint: [u8; 32],
    },
    Tombstoned {
        prior_descriptor_fingerprint: [u8; 32],
        tombstone_delta: [u8; 32],
    },
}

impl AddressPresence {
    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        match self {
            Self::NeverObserved => encoder
                .array(2)
                .and_then(|e| e.u16(CONTRACT_VERSION))
                .and_then(|e| e.u8(0))
                .map_err(codec)?,
            Self::Absent {
                address_fingerprint,
            } => {
                nonzero(address_fingerprint, "absent address")?;
                encoder
                    .array(3)
                    .and_then(|e| e.u16(CONTRACT_VERSION))
                    .and_then(|e| e.u8(1))
                    .and_then(|e| e.bytes(address_fingerprint))
                    .map_err(codec)?
            }
            Self::Present {
                descriptor_fingerprint,
            } => {
                nonzero(descriptor_fingerprint, "present descriptor")?;
                encoder
                    .array(3)
                    .and_then(|e| e.u16(CONTRACT_VERSION))
                    .and_then(|e| e.u8(2))
                    .and_then(|e| e.bytes(descriptor_fingerprint))
                    .map_err(codec)?
            }
            Self::Tombstoned {
                prior_descriptor_fingerprint,
                tombstone_delta,
            } => {
                nonzero(prior_descriptor_fingerprint, "tombstone descriptor")?;
                nonzero(tombstone_delta, "tombstone delta")?;
                encoder
                    .array(4)
                    .and_then(|e| e.u16(CONTRACT_VERSION))
                    .and_then(|e| e.u8(3))
                    .and_then(|e| e.bytes(prior_descriptor_fingerprint))
                    .and_then(|e| e.bytes(tombstone_delta))
                    .map_err(codec)?
            }
        };
        if out.len() > MAX_PRESENCE_BYTES {
            return Err(HierarchyHistoryError::Invalid("presence length"));
        }
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, HierarchyHistoryError> {
        if bytes.len() > MAX_PRESENCE_BYTES {
            return Err(HierarchyHistoryError::Invalid("presence length"));
        }
        let mut decoder = Decoder::new(bytes);
        let len = decoder
            .array()
            .map_err(codec)?
            .ok_or(HierarchyHistoryError::Invalid("presence envelope"))?;
        if decoder.u16().map_err(codec)? != CONTRACT_VERSION {
            return Err(HierarchyHistoryError::Invalid("presence version"));
        }
        let value = match (decoder.u8().map_err(codec)?, len) {
            (0, 2) => Self::NeverObserved,
            (1, 3) => Self::Absent {
                address_fingerprint: bytes32(decoder.bytes().map_err(codec)?)?,
            },
            (2, 3) => Self::Present {
                descriptor_fingerprint: bytes32(decoder.bytes().map_err(codec)?)?,
            },
            (3, 4) => Self::Tombstoned {
                prior_descriptor_fingerprint: bytes32(decoder.bytes().map_err(codec)?)?,
                tombstone_delta: bytes32(decoder.bytes().map_err(codec)?)?,
            },
            _ => return Err(HierarchyHistoryError::Invalid("presence tag/length")),
        };
        if decoder.position() != bytes.len() || value.encode_canonical()? != bytes {
            return Err(HierarchyHistoryError::Invalid("noncanonical presence"));
        }
        Ok(value)
    }

    pub fn fingerprint(&self) -> Result<[u8; 32], HierarchyHistoryError> {
        Ok(hash(ADDRESS_PRESENCE_DOMAIN, &self.encode_canonical()?))
    }
}

fn nonzero(value: &[u8; 32], label: &'static str) -> Result<(), HierarchyHistoryError> {
    if *value == [0; 32] {
        Err(HierarchyHistoryError::Invalid(label))
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum DescriptorOrigin {
    Procedural = 0,
    Dynamic = 1,
}

impl TryFrom<u8> for DescriptorOrigin {
    type Error = HierarchyHistoryError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Procedural),
            1 => Ok(Self::Dynamic),
            _ => Err(HierarchyHistoryError::Invalid("descriptor origin")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HierarchyDescriptor {
    pub logical_id: [u8; 32],
    pub parent_logical_id: Option<[u8; 32]>,
    pub reconstruction_fingerprint: [u8; 32],
    pub world_conditions_contract: [u8; 32],
    pub world_conditions_fingerprint: [u8; 32],
    pub origin: DescriptorOrigin,
    pub recipe: Vec<u8>,
}

impl HierarchyDescriptor {
    pub fn new(
        logical_id: [u8; 32],
        parent_logical_id: Option<[u8; 32]>,
        reconstruction_fingerprint: [u8; 32],
        world_conditions_contract: [u8; 32],
        world_conditions_fingerprint: [u8; 32],
        origin: DescriptorOrigin,
        recipe: Vec<u8>,
    ) -> Result<Self, HierarchyHistoryError> {
        if recipe.is_empty() || recipe.len() > MAX_RECIPE_BYTES {
            return Err(HierarchyHistoryError::Invalid("descriptor recipe length"));
        }
        Ok(Self {
            logical_id,
            parent_logical_id,
            reconstruction_fingerprint,
            world_conditions_contract,
            world_conditions_fingerprint,
            origin,
            recipe,
        })
    }

    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        encoder
            .array(8)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&self.logical_id))
            .map_err(codec)?;
        match self.parent_logical_id {
            Some(parent) => encoder.bytes(&parent).map_err(codec)?,
            None => encoder.null().map_err(codec)?,
        };
        encoder
            .bytes(&self.reconstruction_fingerprint)
            .and_then(|e| e.bytes(&self.world_conditions_contract))
            .and_then(|e| e.bytes(&self.world_conditions_fingerprint))
            .and_then(|e| e.u8(self.origin as u8))
            .and_then(|e| e.bytes(&self.recipe))
            .map_err(codec)?;
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, HierarchyHistoryError> {
        let mut decoder = Decoder::new(bytes);
        if decoder.array().map_err(codec)? != Some(8)
            || decoder.u16().map_err(codec)? != CONTRACT_VERSION
        {
            return Err(HierarchyHistoryError::Invalid("descriptor envelope"));
        }
        let logical_id = bytes32(decoder.bytes().map_err(codec)?)?;
        let parent_logical_id = match decoder.datatype().map_err(codec)? {
            minicbor::data::Type::Null => {
                decoder.null().map_err(codec)?;
                None
            }
            minicbor::data::Type::Bytes => Some(bytes32(decoder.bytes().map_err(codec)?)?),
            _ => return Err(HierarchyHistoryError::Invalid("descriptor parent")),
        };
        let descriptor = Self::new(
            logical_id,
            parent_logical_id,
            bytes32(decoder.bytes().map_err(codec)?)?,
            bytes32(decoder.bytes().map_err(codec)?)?,
            bytes32(decoder.bytes().map_err(codec)?)?,
            DescriptorOrigin::try_from(decoder.u8().map_err(codec)?)?,
            decoder.bytes().map_err(codec)?.to_vec(),
        )?;
        if decoder.position() != bytes.len() || descriptor.encode_canonical()? != bytes {
            return Err(HierarchyHistoryError::Invalid("noncanonical descriptor"));
        }
        Ok(descriptor)
    }

    pub fn fingerprint(&self) -> Result<[u8; 32], HierarchyHistoryError> {
        Ok(hash(DESCRIPTOR_DOMAIN, &self.encode_canonical()?))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct ChildCursor {
    pub parent_descriptor: [u8; 32],
    pub child_kind: u16,
    pub next_index: u64,
}

impl ChildCursor {
    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let mut out = Vec::new();
        Encoder::new(&mut out)
            .array(4)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&self.parent_descriptor))
            .and_then(|e| e.u16(self.child_kind))
            .and_then(|e| e.u64(self.next_index))
            .map_err(codec)?;
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, HierarchyHistoryError> {
        let mut decoder = Decoder::new(bytes);
        if decoder.array().map_err(codec)? != Some(4)
            || decoder.u16().map_err(codec)? != CONTRACT_VERSION
        {
            return Err(HierarchyHistoryError::Invalid("cursor envelope"));
        }
        let cursor = Self {
            parent_descriptor: bytes32(decoder.bytes().map_err(codec)?)?,
            child_kind: decoder.u16().map_err(codec)?,
            next_index: decoder.u64().map_err(codec)?,
        };
        if cursor.child_kind == 0
            || decoder.position() != bytes.len()
            || cursor.encode_canonical()? != bytes
        {
            return Err(HierarchyHistoryError::Invalid("noncanonical cursor"));
        }
        Ok(cursor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct ChildDescriptorRef {
    pub index: u64,
    pub logical_id: [u8; 32],
    pub descriptor_fingerprint: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ObservationWindow {
    pub parent_descriptor: [u8; 32],
    pub child_kind: u16,
    pub start_index: u64,
    pub requested_limit: u16,
    pub children: Vec<ChildDescriptorRef>,
    pub next_cursor: Option<ChildCursor>,
    pub has_more: bool,
    pub examined: u16,
}

impl ObservationWindow {
    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        encoder
            .array(9)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&self.parent_descriptor))
            .and_then(|e| e.u16(self.child_kind))
            .and_then(|e| e.u64(self.start_index))
            .and_then(|e| e.u16(self.requested_limit))
            .and_then(|e| e.array(self.children.len() as u64))
            .map_err(codec)?;
        for child in &self.children {
            encoder
                .array(3)
                .and_then(|e| e.u64(child.index))
                .and_then(|e| e.bytes(&child.logical_id))
                .and_then(|e| e.bytes(&child.descriptor_fingerprint))
                .map_err(codec)?;
        }
        match self.next_cursor {
            Some(cursor) => encoder.bytes(&cursor.encode_canonical()?).map_err(codec)?,
            None => encoder.null().map_err(codec)?,
        };
        encoder
            .bool(self.has_more)
            .and_then(|e| e.u16(self.examined))
            .map_err(codec)?;
        Ok(out)
    }
}

pub fn observe_fixture_window(
    parent: &HierarchyDescriptor,
    child_kind: u16,
    cursor: Option<ChildCursor>,
    limit: u16,
    logical_count: u64,
    work_budget: u16,
) -> Result<ObservationWindow, HierarchyHistoryError> {
    if child_kind == 0 || limit > MAX_CHILD_WINDOW {
        return Err(HierarchyHistoryError::Invalid("window kind/limit"));
    }
    let parent_descriptor = parent.fingerprint()?;
    let start = if let Some(cursor) = cursor {
        if cursor.parent_descriptor != parent_descriptor || cursor.child_kind != child_kind {
            return Err(HierarchyHistoryError::StaleCursor);
        }
        cursor.next_index
    } else {
        0
    };
    if start > logical_count {
        return Err(HierarchyHistoryError::StaleCursor);
    }
    if limit == 0 {
        if start != logical_count || work_budget != 0 {
            return Err(HierarchyHistoryError::Invalid("zero window"));
        }
        return Ok(ObservationWindow {
            parent_descriptor,
            child_kind,
            start_index: start,
            requested_limit: 0,
            children: Vec::new(),
            next_cursor: None,
            has_more: false,
            examined: 0,
        });
    }
    let count = logical_count.saturating_sub(start).min(u64::from(limit)) as u16;
    if work_budget < count {
        return Err(HierarchyHistoryError::Cancelled);
    }
    let mut children = Vec::with_capacity(usize::from(count));
    for offset in 0..u64::from(count) {
        let index = start
            .checked_add(offset)
            .ok_or(HierarchyHistoryError::Invalid("index"))?;
        let mut preimage = Vec::with_capacity(42);
        preimage.extend_from_slice(&parent.logical_id);
        preimage.extend_from_slice(&child_kind.to_be_bytes());
        preimage.extend_from_slice(&index.to_be_bytes());
        let logical_id = hash(CHILD_LOGICAL_DOMAIN, &preimage);
        preimage.extend_from_slice(&parent_descriptor);
        children.push(ChildDescriptorRef {
            index,
            logical_id,
            descriptor_fingerprint: hash(CHILD_DESCRIPTOR_DOMAIN, &preimage),
        });
    }
    let end = start
        .checked_add(u64::from(count))
        .ok_or(HierarchyHistoryError::Invalid("window end"))?;
    let has_more = end < logical_count;
    Ok(ObservationWindow {
        parent_descriptor,
        child_kind,
        start_index: start,
        requested_limit: limit,
        children,
        next_cursor: has_more.then_some(ChildCursor {
            parent_descriptor,
            child_kind,
            next_index: end,
        }),
        has_more,
        examined: count,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum ResidencyState {
    Cold,
    Warm,
    Evicted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct MaterializationReceipt {
    pub descriptor_fingerprint: [u8; 32],
    pub state: ResidencyState,
    pub measured_cost_units: u64,
}

impl MaterializationReceipt {
    pub fn for_descriptor(
        descriptor: &HierarchyDescriptor,
        state: ResidencyState,
        measured_cost_units: u64,
    ) -> Result<Self, HierarchyHistoryError> {
        Ok(Self {
            descriptor_fingerprint: descriptor.fingerprint()?,
            state,
            measured_cost_units,
        })
    }
}

fn bytes32(bytes: &[u8]) -> Result<[u8; 32], HierarchyHistoryError> {
    bytes
        .try_into()
        .map_err(|_| HierarchyHistoryError::Invalid("expected 32 bytes"))
}
