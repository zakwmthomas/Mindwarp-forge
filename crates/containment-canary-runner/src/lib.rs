//! Pure helpers for the owner-gated LPAC denial-canary runner.

use std::{collections::BTreeMap, fs, path::Path};

pub const REPORT_LIMIT: usize = 256;
pub const EXPECTED_SENTINEL_ERROR: i32 = 5;
pub const EXPECTED_CHILD_ERROR: i32 = 5;
pub const EXPECTED_LOOPBACK_ERROR: i32 = 10013;
pub const ERROR_INVALID_PARAMETER_CODE: i32 = 87;
pub const LPAC_ACCESS_MASK: u32 = 0x2;
pub const REQUIRED_IMAGE_LOAD_FLAGS: u32 = 0x7;

/// Reconciles the direct token query with an independent access-check
/// discriminator. The access check is always mandatory; error 87 is tolerated
/// only for the class-46 query and never weakens the required LPAC behavior.
pub fn validate_lpac_observation(
    class_46: Result<u32, i32>,
    granted_access: u32,
) -> Result<&'static str, String> {
    if granted_access != LPAC_ACCESS_MASK {
        return Err(format!(
            "LPAC access discriminator granted 0x{granted_access:x}, expected 0x{LPAC_ACCESS_MASK:x}"
        ));
    }
    match class_46 {
        Ok(1) => Ok("class46-and-access-check"),
        Err(ERROR_INVALID_PARAMETER_CODE) => Ok("access-check-after-class46-error87"),
        Ok(value) => Err(format!("class-46 LPAC query returned {value}, expected 1")),
        Err(code) => Err(format!(
            "class-46 LPAC query failed with Windows error {code}; only error 87 has a reviewed compatibility path"
        )),
    }
}

pub fn validate_image_load_flags(flags: u32) -> Result<(), String> {
    if flags & REQUIRED_IMAGE_LOAD_FLAGS != REQUIRED_IMAGE_LOAD_FLAGS {
        return Err(format!(
            "image-load mitigation flags are 0x{flags:x}, required mask is 0x{REQUIRED_IMAGE_LOAD_FLAGS:x}"
        ));
    }
    Ok(())
}

pub fn format_post_resume_exit(exit: u32, lpac_verification: &str) -> String {
    format!("post-resume canary exit 0x{exit:08X} ({exit}) after {lpac_verification}")
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanaryReport {
    pub token_is_appcontainer: u32,
    pub capability_count: u32,
    pub sentinel_error: i32,
    pub child_error: i32,
    pub loopback_error: i32,
}

pub fn expected_report_bytes() -> Vec<u8> {
    format!(
        "{{\"schema\":1,\"token_is_appcontainer\":1,\"capability_count\":0,\"sentinel_error\":{},\"child_error\":{},\"loopback_error\":{}}}",
        EXPECTED_SENTINEL_ERROR, EXPECTED_CHILD_ERROR, EXPECTED_LOOPBACK_ERROR
    )
    .into_bytes()
}

pub fn parse_report(bytes: &[u8]) -> Result<CanaryReport, String> {
    if bytes.len() > REPORT_LIMIT {
        return Err("report exceeds byte limit".into());
    }
    std::str::from_utf8(bytes).map_err(|_| "report is not strict UTF-8")?;
    if bytes != expected_report_bytes() {
        return Err(
            "report is noncanonical, contains an unknown field, or records a failed denial".into(),
        );
    }
    Ok(CanaryReport {
        token_is_appcontainer: 1,
        capability_count: 0,
        sentinel_error: EXPECTED_SENTINEL_ERROR,
        child_error: EXPECTED_CHILD_ERROR,
        loopback_error: EXPECTED_LOOPBACK_ERROR,
    })
}

pub fn appcontainer_pe_flag(bytes: &[u8]) -> Result<bool, String> {
    if bytes.len() < 0x40 || &bytes[..2] != b"MZ" {
        return Err("not a bounded PE image".into());
    }
    let pe_offset = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
    if pe_offset
        .checked_add(24)
        .is_none_or(|end| end > bytes.len())
        || &bytes[pe_offset..pe_offset + 4] != b"PE\0\0"
    {
        return Err("invalid PE header".into());
    }
    let optional = pe_offset + 24;
    if optional + 72 > bytes.len() {
        return Err("truncated PE optional header".into());
    }
    let magic = u16::from_le_bytes(bytes[optional..optional + 2].try_into().unwrap());
    if magic != 0x20b && magic != 0x10b {
        return Err("unsupported PE optional header".into());
    }
    let characteristics =
        u16::from_le_bytes(bytes[optional + 70..optional + 72].try_into().unwrap());
    Ok(characteristics & 0x1000 != 0)
}

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    const H0: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let bit_len = (bytes.len() as u64) * 8;
    let mut padded = bytes.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    let mut h = H0;
    for chunk in padded.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, word) in w[..16].iter_mut().enumerate() {
            *word = u32::from_be_bytes(chunk[i * 4..i * 4 + 4].try_into().unwrap());
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (slot, value) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *slot = slot.wrapping_add(value);
        }
    }
    let mut out = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn bounded_inventory(root: &Path) -> Result<BTreeMap<String, [u8; 32]>, String> {
    fn visit(
        root: &Path,
        current: &Path,
        out: &mut BTreeMap<String, [u8; 32]>,
    ) -> Result<(), String> {
        let mut entries = fs::read_dir(current)
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let rel = path.strip_prefix(root).map_err(|e| e.to_string())?;
            if rel.components().any(|component| {
                matches!(
                    component.as_os_str().to_str(),
                    Some("target" | "context" | "evidence" | ".local" | "node_modules" | "dist")
                )
            }) {
                continue;
            }
            let ty = entry.file_type().map_err(|e| e.to_string())?;
            if ty.is_symlink() {
                return Err(format!("inventory encountered symlink: {}", rel.display()));
            }
            if ty.is_dir() {
                visit(root, &path, out)?;
            } else if ty.is_file() {
                let bytes = fs::read(&path).map_err(|e| e.to_string())?;
                out.insert(rel.to_string_lossy().replace('\\', "/"), sha256(&bytes));
            }
        }
        Ok(())
    }
    let mut out = BTreeMap::new();
    visit(root, root, &mut out)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_known_answer() {
        assert_eq!(
            hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn strict_report_rejects_unknown_or_failed_fields() {
        assert!(parse_report(&expected_report_bytes()).is_ok());
        let mut unknown = expected_report_bytes();
        unknown.pop();
        unknown.extend_from_slice(b",\"x\":1}");
        assert!(parse_report(&unknown).is_err());
        assert!(parse_report(&vec![b'x'; REPORT_LIMIT + 1]).is_err());
        assert!(parse_report(b"{\"schema\":1,\"token_is_appcontainer\":0}").is_err());
    }

    #[test]
    fn pe_parser_requires_appcontainer_flag() {
        let mut pe = vec![0u8; 0x200];
        pe[..2].copy_from_slice(b"MZ");
        pe[0x3c..0x40].copy_from_slice(&(0x80u32).to_le_bytes());
        pe[0x80..0x84].copy_from_slice(b"PE\0\0");
        let optional = 0x80 + 24;
        pe[optional..optional + 2].copy_from_slice(&0x20bu16.to_le_bytes());
        assert_eq!(appcontainer_pe_flag(&pe).unwrap(), false);
        pe[optional + 70..optional + 72].copy_from_slice(&0x1000u16.to_le_bytes());
        assert_eq!(appcontainer_pe_flag(&pe).unwrap(), true);
    }

    #[test]
    fn lpac_compatibility_is_fail_closed_and_requires_independent_access_proof() {
        assert_eq!(
            validate_lpac_observation(Ok(1), LPAC_ACCESS_MASK).unwrap(),
            "class46-and-access-check"
        );
        assert_eq!(
            validate_lpac_observation(Err(87), LPAC_ACCESS_MASK).unwrap(),
            "access-check-after-class46-error87"
        );
        for rejected in [
            validate_lpac_observation(Ok(0), LPAC_ACCESS_MASK),
            validate_lpac_observation(Ok(1), 0x1),
            validate_lpac_observation(Ok(1), 0x3),
            validate_lpac_observation(Err(5), LPAC_ACCESS_MASK),
            validate_lpac_observation(Err(87), 0),
        ] {
            assert!(rejected.is_err());
        }
    }

    #[test]
    fn image_load_mask_matches_the_three_documented_leading_bits() {
        assert!(validate_image_load_flags(0x7).is_ok());
        assert!(validate_image_load_flags(0xf).is_ok());
        assert!(validate_image_load_flags(0x3).is_err());
        assert!(validate_image_load_flags(0xb).is_err());
    }

    #[test]
    fn post_resume_failure_retains_hex_status_and_lpac_mode() {
        assert_eq!(
            format_post_resume_exit(0xC000_0142, "access-check-after-class46-error87"),
            "post-resume canary exit 0xC0000142 (3221225794) after access-check-after-class46-error87"
        );
    }
}
