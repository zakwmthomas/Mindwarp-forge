//! Capability-free deterministic field reference. Canonical math is integer-only.

use minicbor::{Decoder, Encoder};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

pub const CONTRACT_VERSION: u16 = 1;
pub const VALUE_FRAC: u32 = 48;
pub const COORD_FRAC: u32 = 32;
pub const ONE: i64 = 1_i64 << VALUE_FRAC;
const MAX_TERMS: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldError {
    Overflow,
    InvalidRecipe(&'static str),
    Codec(String),
}
impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for FieldError {}

fn round_shift_even(value: i128, shift: u32) -> Result<i64, FieldError> {
    let negative = value < 0;
    let magnitude = value.unsigned_abs();
    let base = magnitude >> shift;
    let remainder = magnitude & ((1_u128 << shift) - 1);
    let half = 1_u128 << (shift - 1);
    let rounded = base + u128::from(remainder > half || (remainder == half && base & 1 == 1));
    let signed = if negative {
        -(rounded as i128)
    } else {
        rounded as i128
    };
    i64::try_from(signed).map_err(|_| FieldError::Overflow)
}

pub fn mul_value(a: i64, b: i64) -> Result<i64, FieldError> {
    round_shift_even(i128::from(a) * i128::from(b), VALUE_FRAC)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Constant(i64),
    ValueLattice2 {
        frequency: u32,
        amplitude: i64,
        component: u32,
    },
    Add {
        left: u16,
        right: u16,
    },
    Multiply {
        left: u16,
        right: u16,
    },
    Ridged {
        input: u16,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldRecipe {
    pub terms: Vec<Term>,
    pub output: u16,
}

impl FieldRecipe {
    pub fn new(terms: Vec<Term>, output: u16) -> Result<Self, FieldError> {
        if terms.is_empty() || terms.len() > MAX_TERMS || usize::from(output) >= terms.len() {
            return Err(FieldError::InvalidRecipe("term count/output"));
        }
        for (index, term) in terms.iter().enumerate() {
            let refs: &[u16] = match term {
                Term::Add { left, right } | Term::Multiply { left, right } => &[*left, *right],
                Term::Ridged { input } => &[*input],
                Term::ValueLattice2 {
                    frequency,
                    amplitude,
                    ..
                } => {
                    if *frequency == 0 || amplitude.unsigned_abs() > ONE as u64 {
                        return Err(FieldError::InvalidRecipe("basis range"));
                    }
                    &[]
                }
                Term::Constant(value) => {
                    if value.unsigned_abs() > (ONE as u64) * 16 {
                        return Err(FieldError::InvalidRecipe("constant range"));
                    }
                    &[]
                }
            };
            if refs.iter().any(|r| usize::from(*r) >= index) {
                return Err(FieldError::InvalidRecipe("forward/self reference"));
            }
        }
        Ok(Self { terms, output })
    }

    pub fn encode_canonical(&self) -> Result<Vec<u8>, FieldError> {
        let mut out = Vec::new();
        let mut e = Encoder::new(&mut out);
        e.array(3)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.array(self.terms.len() as u64))
            .map_err(codec)?;
        for term in &self.terms {
            match term {
                Term::Constant(v) => {
                    e.array(2)
                        .and_then(|e| e.u8(0))
                        .and_then(|e| e.i64(*v))
                        .map_err(codec)?;
                }
                Term::ValueLattice2 {
                    frequency,
                    amplitude,
                    component,
                } => {
                    e.array(4)
                        .and_then(|e| e.u8(1))
                        .and_then(|e| e.u32(*frequency))
                        .and_then(|e| e.i64(*amplitude))
                        .and_then(|e| e.u32(*component))
                        .map_err(codec)?;
                }
                Term::Add { left, right } => {
                    e.array(3)
                        .and_then(|e| e.u8(2))
                        .and_then(|e| e.u16(*left))
                        .and_then(|e| e.u16(*right))
                        .map_err(codec)?;
                }
                Term::Multiply { left, right } => {
                    e.array(3)
                        .and_then(|e| e.u8(3))
                        .and_then(|e| e.u16(*left))
                        .and_then(|e| e.u16(*right))
                        .map_err(codec)?;
                }
                Term::Ridged { input } => {
                    e.array(2)
                        .and_then(|e| e.u8(4))
                        .and_then(|e| e.u16(*input))
                        .map_err(codec)?;
                }
            }
        }
        e.u16(self.output).map_err(codec)?;
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, FieldError> {
        let mut d = Decoder::new(bytes);
        if d.array().map_err(codec)? != Some(3) || d.u16().map_err(codec)? != CONTRACT_VERSION {
            return Err(FieldError::InvalidRecipe("envelope"));
        }
        let count = d
            .array()
            .map_err(codec)?
            .ok_or(FieldError::InvalidRecipe("indefinite"))? as usize;
        if count == 0 || count > MAX_TERMS {
            return Err(FieldError::InvalidRecipe("term count"));
        }
        let mut terms = Vec::with_capacity(count);
        for _ in 0..count {
            let len = d
                .array()
                .map_err(codec)?
                .ok_or(FieldError::InvalidRecipe("indefinite"))?;
            let tag = d.u8().map_err(codec)?;
            terms.push(match (tag, len) {
                (0, 2) => Term::Constant(d.i64().map_err(codec)?),
                (1, 4) => Term::ValueLattice2 {
                    frequency: d.u32().map_err(codec)?,
                    amplitude: d.i64().map_err(codec)?,
                    component: d.u32().map_err(codec)?,
                },
                (2, 3) => Term::Add {
                    left: d.u16().map_err(codec)?,
                    right: d.u16().map_err(codec)?,
                },
                (3, 3) => Term::Multiply {
                    left: d.u16().map_err(codec)?,
                    right: d.u16().map_err(codec)?,
                },
                (4, 2) => Term::Ridged {
                    input: d.u16().map_err(codec)?,
                },
                _ => return Err(FieldError::InvalidRecipe("term tag/length")),
            });
        }
        let recipe = Self::new(terms, d.u16().map_err(codec)?)?;
        if d.position() != bytes.len() || recipe.encode_canonical()? != bytes {
            return Err(FieldError::InvalidRecipe("noncanonical/trailing"));
        }
        Ok(recipe)
    }
}

fn codec<E: fmt::Display>(e: E) -> FieldError {
    FieldError::Codec(e.to_string())
}

pub fn philox4x32_10(mut c: [u32; 4], mut k: [u32; 2]) -> [u32; 4] {
    const M0: u64 = 0xD2511F53;
    const M1: u64 = 0xCD9E8D57;
    for _ in 0..10 {
        let p0 = M0 * u64::from(c[0]);
        let p1 = M1 * u64::from(c[2]);
        c = [
            ((p1 >> 32) as u32) ^ c[1] ^ k[0],
            p1 as u32,
            ((p0 >> 32) as u32) ^ c[3] ^ k[1],
            p0 as u32,
        ];
        k[0] = k[0].wrapping_add(0x9E3779B9);
        k[1] = k[1].wrapping_add(0xBB67AE85);
    }
    c
}

fn zigzag(value: i32) -> u32 {
    ((value << 1) ^ (value >> 31)) as u32
}
fn lattice(key: [u32; 2], x: i32, y: i32, component: u32) -> i64 {
    let word = philox4x32_10([zigzag(x), zigzag(y), component, 0], key)[0];
    (i64::from(word) - (1_i64 << 31)) << 17
}
fn floor_coord(value: i64) -> Result<(i32, u32), FieldError> {
    let cell = value.div_euclid(1_i64 << COORD_FRAC);
    Ok((
        i32::try_from(cell).map_err(|_| FieldError::Overflow)?,
        value.rem_euclid(1_i64 << COORD_FRAC) as u32,
    ))
}
fn fade(frac: u32) -> Result<i64, FieldError> {
    let t = i64::from(frac) << 16;
    let t2 = mul_value(t, t)?;
    let t3 = mul_value(t2, t)?;
    let inner = mul_value(
        t,
        mul_value(t, 6 * ONE)?
            .checked_sub(15 * ONE)
            .ok_or(FieldError::Overflow)?,
    )?
    .checked_add(10 * ONE)
    .ok_or(FieldError::Overflow)?;
    mul_value(t3, inner)
}
fn lerp(a: i64, b: i64, t: i64) -> Result<i64, FieldError> {
    a.checked_add(mul_value(b.checked_sub(a).ok_or(FieldError::Overflow)?, t)?)
        .ok_or(FieldError::Overflow)
}

pub fn sample(
    recipe: &FieldRecipe,
    stream_key: [u8; 32],
    x: i64,
    y: i64,
) -> Result<i64, FieldError> {
    let key = [
        u32::from_le_bytes(stream_key[0..4].try_into().unwrap()),
        u32::from_le_bytes(stream_key[4..8].try_into().unwrap()),
    ];
    let mut values: Vec<i64> = Vec::with_capacity(recipe.terms.len());
    for term in &recipe.terms {
        let value = match *term {
            Term::Constant(v) => v,
            Term::ValueLattice2 {
                frequency,
                amplitude,
                component,
            } => {
                let sx = i128::from(x) * i128::from(frequency);
                let sy = i128::from(y) * i128::from(frequency);
                let sx = i64::try_from(sx).map_err(|_| FieldError::Overflow)?;
                let sy = i64::try_from(sy).map_err(|_| FieldError::Overflow)?;
                let (cx, fx) = floor_coord(sx)?;
                let (cy, fy) = floor_coord(sy)?;
                let tx = fade(fx)?;
                let ty = fade(fy)?;
                let a = lerp(
                    lattice(key, cx, cy, component),
                    lattice(
                        key,
                        cx.checked_add(1).ok_or(FieldError::Overflow)?,
                        cy,
                        component,
                    ),
                    tx,
                )?;
                let b = lerp(
                    lattice(
                        key,
                        cx,
                        cy.checked_add(1).ok_or(FieldError::Overflow)?,
                        component,
                    ),
                    lattice(
                        key,
                        cx.checked_add(1).ok_or(FieldError::Overflow)?,
                        cy.checked_add(1).ok_or(FieldError::Overflow)?,
                        component,
                    ),
                    tx,
                )?;
                mul_value(lerp(a, b, ty)?, amplitude)?
            }
            Term::Add { left, right } => values[usize::from(left)]
                .checked_add(values[usize::from(right)])
                .ok_or(FieldError::Overflow)?,
            Term::Multiply { left, right } => {
                mul_value(values[usize::from(left)], values[usize::from(right)])?
            }
            Term::Ridged { input } => ONE
                .checked_sub(
                    values[usize::from(input)]
                        .checked_abs()
                        .ok_or(FieldError::Overflow)?,
                )
                .ok_or(FieldError::Overflow)?,
        };
        values.push(value);
    }
    Ok(values[usize::from(recipe.output)])
}

pub fn recipe_fingerprint(recipe: &FieldRecipe) -> Result<[u8; 32], FieldError> {
    Ok(Sha256::digest(
        [
            b"mw-field-recipe-v1".as_slice(),
            &recipe.encode_canonical()?,
        ]
        .concat(),
    )
    .into())
}
pub fn cache_key(
    recipe: &FieldRecipe,
    reconstruction: [u8; 32],
    domain: &[u8],
) -> Result<[u8; 32], FieldError> {
    Ok(Sha256::digest(
        [
            b"mw-field-cache-v1".as_slice(),
            &reconstruction,
            &recipe.encode_canonical()?,
            domain,
        ]
        .concat(),
    )
    .into())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldProofEvidence {
    pub fixture_id: String,
    pub exact: bool,
    pub canonical: bool,
    pub limitations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn recipe() -> FieldRecipe {
        FieldRecipe::new(
            vec![
                Term::ValueLattice2 {
                    frequency: 2,
                    amplitude: ONE,
                    component: 7,
                },
                Term::Ridged { input: 0 },
            ],
            1,
        )
        .unwrap()
    }
    #[test]
    fn philox_known_answer_is_fixed() {
        assert_eq!(
            philox4x32_10([0; 4], [0; 2]),
            [0x6627e8d5, 0xe169c58d, 0xbc57ac4c, 0x9b00dbd8]
        );
    }
    #[test]
    fn recipe_bytes_are_strict_and_stable() {
        let r = recipe();
        let b = r.encode_canonical().unwrap();
        assert_eq!(FieldRecipe::decode_strict(&b).unwrap(), r);
        let mut trailing = b;
        trailing.push(0);
        assert!(FieldRecipe::decode_strict(&trailing).is_err());
    }
    #[test]
    fn same_inputs_repeat_and_components_partition() {
        let r = recipe();
        let a = sample(&r, [3; 32], 123456789, -987654321).unwrap();
        assert_eq!(a, sample(&r, [3; 32], 123456789, -987654321).unwrap());
        let r2 = FieldRecipe::new(
            vec![Term::ValueLattice2 {
                frequency: 2,
                amplitude: ONE,
                component: 8,
            }],
            0,
        )
        .unwrap();
        assert_ne!(a, sample(&r2, [3; 32], 123456789, -987654321).unwrap());
    }
    #[test]
    fn exact_cell_boundaries_are_continuous() {
        let r = FieldRecipe::new(
            vec![Term::ValueLattice2 {
                frequency: 1,
                amplitude: ONE,
                component: 0,
            }],
            0,
        )
        .unwrap();
        let at = sample(&r, [0; 32], 1_i64 << 32, 0).unwrap();
        assert_eq!(at, lattice([0, 0], 1, 0, 0));
    }
    #[test]
    fn composition_order_is_explicit() {
        let r = FieldRecipe::new(
            vec![
                Term::Constant(ONE / 2),
                Term::Constant(ONE / 4),
                Term::Add { left: 0, right: 1 },
                Term::Multiply { left: 0, right: 1 },
            ],
            2,
        )
        .unwrap();
        assert_eq!(sample(&r, [0; 32], 0, 0).unwrap(), 3 * ONE / 4);
    }
    #[test]
    fn poison_and_overflow_fail_closed() {
        assert!(
            FieldRecipe::new(
                vec![Term::ValueLattice2 {
                    frequency: 0,
                    amplitude: ONE,
                    component: 0
                }],
                0
            )
            .is_err()
        );
        assert!(FieldRecipe::new(vec![Term::Add { left: 0, right: 0 }], 0).is_err());
        assert_eq!(mul_value(i64::MAX, i64::MAX), Err(FieldError::Overflow));
    }
    #[test]
    fn cache_key_binds_recipe_identity_and_domain() {
        let r = recipe();
        assert_ne!(
            cache_key(&r, [1; 32], b"a").unwrap(),
            cache_key(&r, [1; 32], b"b").unwrap()
        );
        assert_ne!(
            cache_key(&r, [1; 32], b"a").unwrap(),
            cache_key(&r, [2; 32], b"a").unwrap()
        );
    }
    #[test]
    fn proof_evidence_is_authority_negative() {
        let e = FieldProofEvidence {
            fixture_id: "fixed-v1".into(),
            exact: true,
            canonical: true,
            limitations: vec!["CPU reference; not runtime performance".into()],
        };
        let j = serde_json::to_value(e).unwrap();
        assert!(
            j.get("approve").is_none() && j.get("promote").is_none() && j.get("execute").is_none()
        );
    }
}
