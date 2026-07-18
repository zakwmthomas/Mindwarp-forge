//! Semantic-neutral checked fixed-width interval arithmetic.
//!
//! This crate owns representation and directed arithmetic only. It has no
//! physical, optical, spectral, identity, authority, persistence, or runtime
//! semantics. Native limbs are intentionally inaccessible.

use core::{cmp::Ordering, fmt};

use crypto_bigint::{CheckedAdd, NonZero, U512};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixedArithmeticError {
    InvalidDecimal,
    StorageOverflow,
    DivisionByZero,
    ReversedInterval,
    IncompatibleScale,
    NegativeSquareRoot,
    InvalidProjection,
}

impl fmt::Display for FixedArithmeticError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidDecimal => "invalid canonical decimal",
            Self::StorageOverflow => "fixed arithmetic storage overflow",
            Self::DivisionByZero => "fixed arithmetic division by zero",
            Self::ReversedInterval => "reversed fixed interval",
            Self::IncompatibleScale => "incompatible fixed interval scale",
            Self::NegativeSquareRoot => "negative fixed square root",
            Self::InvalidProjection => "invalid fixed precision projection",
        })
    }
}

impl std::error::Error for FixedArithmeticError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signed512 {
    negative: bool,
    magnitude: U512,
}

impl Signed512 {
    pub fn zero() -> Self {
        Self::new(false, U512::ZERO)
    }
    pub fn one() -> Self {
        Self::new(false, U512::ONE)
    }
    pub fn from_i64(value: i64) -> Self {
        Self::new(value.is_negative(), U512::from(value.unsigned_abs()))
    }
    pub fn from_i128(value: i128) -> Self {
        Self::new(value.is_negative(), U512::from(value.unsigned_abs()))
    }
    fn new(negative: bool, magnitude: U512) -> Self {
        Self {
            negative: negative && magnitude != U512::ZERO,
            magnitude,
        }
    }
    pub fn is_negative(&self) -> bool {
        self.negative
    }
    pub fn maximum_magnitude_bits(&self) -> u16 {
        self.magnitude.bits_vartime() as u16
    }
    pub fn checked_neg(&self) -> Self {
        Self::new(!self.negative, self.magnitude)
    }
    pub fn checked_add(&self, other: &Self) -> Result<Self, FixedArithmeticError> {
        if self.negative == other.negative {
            Ok(Self::new(
                self.negative,
                Option::<U512>::from(self.magnitude.checked_add(&other.magnitude))
                    .ok_or(FixedArithmeticError::StorageOverflow)?,
            ))
        } else {
            Ok(match self.magnitude.cmp(&other.magnitude) {
                Ordering::Greater | Ordering::Equal => {
                    Self::new(self.negative, self.magnitude.wrapping_sub(&other.magnitude))
                }
                Ordering::Less => Self::new(
                    other.negative,
                    other.magnitude.wrapping_sub(&self.magnitude),
                ),
            })
        }
    }
    pub fn checked_sub(&self, other: &Self) -> Result<Self, FixedArithmeticError> {
        self.checked_add(&other.checked_neg())
    }
    pub fn checked_mul(&self, other: &Self) -> Result<Self, FixedArithmeticError> {
        Ok(Self::new(
            self.negative ^ other.negative,
            Option::<U512>::from(self.magnitude.checked_mul(&other.magnitude))
                .ok_or(FixedArithmeticError::StorageOverflow)?,
        ))
    }
    pub fn checked_shl(&self, shift: u16) -> Result<Self, FixedArithmeticError> {
        Ok(Self::new(
            self.negative,
            self.magnitude
                .overflowing_shl_vartime(u32::from(shift))
                .ok_or(FixedArithmeticError::StorageOverflow)?,
        ))
    }
    fn div_parts(&self, denominator: &Self) -> Result<(Self, bool, bool), FixedArithmeticError> {
        let divisor = Option::<NonZero<U512>>::from(NonZero::new(denominator.magnitude))
            .ok_or(FixedArithmeticError::DivisionByZero)?;
        let (quotient, remainder) = self.magnitude.div_rem_vartime(&divisor);
        let negative = self.negative ^ denominator.negative;
        Ok((
            Self::new(negative, quotient),
            remainder != U512::ZERO,
            negative,
        ))
    }
    pub fn div_floor(&self, denominator: &Self) -> Result<Self, FixedArithmeticError> {
        let (value, remainder, negative) = self.div_parts(denominator)?;
        if remainder && negative {
            value.checked_sub(&Self::one())
        } else {
            Ok(value)
        }
    }
    pub fn div_ceil(&self, denominator: &Self) -> Result<Self, FixedArithmeticError> {
        let (value, remainder, negative) = self.div_parts(denominator)?;
        if remainder && !negative {
            value.checked_add(&Self::one())
        } else {
            Ok(value)
        }
    }
    pub fn from_canonical_decimal(value: &str) -> Result<Self, FixedArithmeticError> {
        if value.is_empty() || value.starts_with('+') || value.chars().any(char::is_whitespace) {
            return Err(FixedArithmeticError::InvalidDecimal);
        }
        let (negative, digits) = value
            .strip_prefix('-')
            .map_or((false, value), |digits| (true, digits));
        if digits.is_empty()
            || (digits.len() > 1 && digits.starts_with('0'))
            || (negative && digits == "0")
        {
            return Err(FixedArithmeticError::InvalidDecimal);
        }
        let mut magnitude = U512::ZERO;
        for byte in digits.bytes() {
            if !byte.is_ascii_digit() {
                return Err(FixedArithmeticError::InvalidDecimal);
            }
            magnitude = Option::<U512>::from(magnitude.checked_mul(&U512::from(10_u8)))
                .and_then(|current| {
                    Option::<U512>::from(current.checked_add(&U512::from(byte - b'0')))
                })
                .ok_or(FixedArithmeticError::StorageOverflow)?;
        }
        let parsed = Self::new(negative, magnitude);
        if parsed.canonical_decimal() != value {
            return Err(FixedArithmeticError::InvalidDecimal);
        }
        Ok(parsed)
    }
    pub fn canonical_decimal(&self) -> String {
        if self.magnitude == U512::ZERO {
            return "0".into();
        }
        let ten = NonZero::new(U512::from(10_u8)).expect("ten is nonzero");
        let mut remaining = self.magnitude;
        let mut reversed = Vec::new();
        while remaining != U512::ZERO {
            let (quotient, remainder) = remaining.div_rem_vartime(&ten);
            let digit = (0_u8..=9)
                .find(|digit| remainder == U512::from(*digit))
                .expect("decimal remainder below ten");
            reversed.push(char::from(b'0' + digit));
            remaining = quotient;
        }
        if self.negative {
            reversed.push('-');
        }
        reversed.into_iter().rev().collect()
    }
}

impl Ord for Signed512 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.negative, other.negative) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.magnitude.cmp(&other.magnitude),
            (true, true) => other.magnitude.cmp(&self.magnitude),
        }
    }
}
impl PartialOrd for Signed512 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedInterval {
    lower: Signed512,
    upper: Signed512,
    fractional_bits: u16,
}

impl FixedInterval {
    pub fn new(
        lower: Signed512,
        upper: Signed512,
        fractional_bits: u16,
    ) -> Result<Self, FixedArithmeticError> {
        if lower > upper {
            return Err(FixedArithmeticError::ReversedInterval);
        }
        Ok(Self {
            lower,
            upper,
            fractional_bits,
        })
    }
    pub fn lower(&self) -> &Signed512 {
        &self.lower
    }
    pub fn upper(&self) -> &Signed512 {
        &self.upper
    }
    pub fn fractional_bits(&self) -> u16 {
        self.fractional_bits
    }
    fn compatible(&self, other: &Self) -> Result<(), FixedArithmeticError> {
        if self.fractional_bits != other.fractional_bits {
            Err(FixedArithmeticError::IncompatibleScale)
        } else {
            Ok(())
        }
    }
    pub fn checked_add(&self, other: &Self) -> Result<Self, FixedArithmeticError> {
        self.compatible(other)?;
        Self::new(
            self.lower.checked_add(&other.lower)?,
            self.upper.checked_add(&other.upper)?,
            self.fractional_bits,
        )
    }
    pub fn checked_sub(&self, other: &Self) -> Result<Self, FixedArithmeticError> {
        self.compatible(other)?;
        Self::new(
            self.lower.checked_sub(&other.upper)?,
            self.upper.checked_sub(&other.lower)?,
            self.fractional_bits,
        )
    }
    pub fn checked_mul(&self, other: &Self) -> Result<Self, FixedArithmeticError> {
        self.compatible(other)?;
        let products = [
            self.lower.checked_mul(&other.lower)?,
            self.lower.checked_mul(&other.upper)?,
            self.upper.checked_mul(&other.lower)?,
            self.upper.checked_mul(&other.upper)?,
        ];
        let scale = Signed512::one().checked_shl(self.fractional_bits)?;
        Self::new(
            products
                .iter()
                .min()
                .expect("four products")
                .div_floor(&scale)?,
            products
                .iter()
                .max()
                .expect("four products")
                .div_ceil(&scale)?,
            self.fractional_bits,
        )
    }
    pub fn intersect(&self, other: &Self) -> Result<Self, FixedArithmeticError> {
        self.compatible(other)?;
        Self::new(
            self.lower.clone().max(other.lower.clone()),
            self.upper.clone().min(other.upper.clone()),
            self.fractional_bits,
        )
    }
    pub fn sqrt(&self) -> Result<Self, FixedArithmeticError> {
        if self.lower.is_negative() {
            return Err(FixedArithmeticError::NegativeSquareRoot);
        }
        let lower_radicand = self
            .lower
            .magnitude
            .overflowing_shl_vartime(u32::from(self.fractional_bits))
            .ok_or(FixedArithmeticError::StorageOverflow)?;
        let upper_radicand = self
            .upper
            .magnitude
            .overflowing_shl_vartime(u32::from(self.fractional_bits))
            .ok_or(FixedArithmeticError::StorageOverflow)?;
        let lower = lower_radicand.floor_sqrt_vartime();
        let upper_floor = upper_radicand.floor_sqrt_vartime();
        let exact = Option::<U512>::from(upper_floor.checked_mul(&upper_floor))
            .is_some_and(|square| square == upper_radicand);
        let upper = if exact {
            upper_floor
        } else {
            Option::<U512>::from(upper_floor.checked_add(&U512::ONE))
                .ok_or(FixedArithmeticError::StorageOverflow)?
        };
        Self::new(
            Signed512::new(false, lower),
            Signed512::new(false, upper),
            self.fractional_bits,
        )
    }
    pub fn project(&self, target_bits: u16) -> Result<Self, FixedArithmeticError> {
        if target_bits > self.fractional_bits {
            return Err(FixedArithmeticError::InvalidProjection);
        }
        let divisor = Signed512::one().checked_shl(self.fractional_bits - target_bits)?;
        Self::new(
            self.lower.div_floor(&divisor)?,
            self.upper.div_ceil(&divisor)?,
            target_bits,
        )
    }
    pub fn maximum_magnitude_bits(&self) -> u16 {
        self.lower
            .maximum_magnitude_bits()
            .max(self.upper.maximum_magnitude_bits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_decimal_and_signed_division_are_mathematical() {
        for poison in ["", "+1", " 1", "1 ", "01", "-0", "--1", "a"] {
            assert!(
                Signed512::from_canonical_decimal(poison).is_err(),
                "{poison:?}"
            );
        }
        for value in ["0", "1", "-1", "123456789", "-123456789"] {
            assert_eq!(
                Signed512::from_canonical_decimal(value)
                    .unwrap()
                    .canonical_decimal(),
                value
            );
        }
        for (left, right, floor, ceil) in [
            (-5, 2, -3, -2),
            (5, -2, -3, -2),
            (-5, -2, 2, 3),
            (5, 2, 2, 3),
        ] {
            let left = Signed512::from_i128(left);
            let right = Signed512::from_i128(right);
            assert_eq!(left.div_floor(&right).unwrap(), Signed512::from_i128(floor));
            assert_eq!(left.div_ceil(&right).unwrap(), Signed512::from_i128(ceil));
        }
        assert_eq!(
            Signed512::from_i128(-1)
                .div_floor(&Signed512::from_i128(2))
                .unwrap(),
            Signed512::from_i128(-1)
        );
        assert_eq!(
            Signed512::from_i128(-1)
                .div_ceil(&Signed512::from_i128(2))
                .unwrap(),
            Signed512::zero()
        );
        assert!(Signed512::one().div_floor(&Signed512::zero()).is_err());
    }

    #[test]
    fn checked_values_and_intervals_fail_closed() {
        let maximum = Signed512::from_canonical_decimal(&"9".repeat(154)).unwrap();
        assert!(maximum.checked_mul(&maximum).is_err());
        assert!(Signed512::one().checked_shl(511).is_ok());
        assert!(Signed512::one().checked_shl(512).is_err());
        assert!(FixedInterval::new(Signed512::one(), Signed512::zero(), 160).is_err());
        let first = FixedInterval::new(Signed512::zero(), Signed512::one(), 160).unwrap();
        let other = FixedInterval::new(Signed512::zero(), Signed512::one(), 96).unwrap();
        assert!(first.checked_add(&other).is_err());
    }

    #[test]
    fn interval_products_roots_intersections_and_projection_are_outward() {
        let scale = Signed512::one().checked_shl(160).unwrap();
        let mixed = FixedInterval::new(scale.checked_neg(), scale.clone(), 160).unwrap();
        let square = mixed.checked_mul(&mixed).unwrap();
        assert_eq!(square.lower(), &scale.checked_neg());
        assert_eq!(square.upper(), &scale);
        let four = Signed512::from_i128(4).checked_shl(160).unwrap();
        let exact = FixedInterval::new(four.clone(), four, 160)
            .unwrap()
            .sqrt()
            .unwrap();
        assert_eq!(
            exact.lower().canonical_decimal(),
            Signed512::from_i128(2)
                .checked_shl(160)
                .unwrap()
                .canonical_decimal()
        );
        let two = Signed512::from_i128(2).checked_shl(160).unwrap();
        let nonsquare = FixedInterval::new(two.clone(), two, 160)
            .unwrap()
            .sqrt()
            .unwrap();
        assert!(nonsquare.lower() < nonsquare.upper());
        let projected = nonsquare.project(96).unwrap();
        assert_eq!(projected.fractional_bits(), 96);
        assert!(projected.project(160).is_err());
        assert!(nonsquare.intersect(&exact).is_err());
    }

    #[test]
    fn magnitude_accounting_and_source_surface_remain_target_neutral() {
        assert_eq!(
            Signed512::one()
                .checked_shl(413)
                .unwrap()
                .maximum_magnitude_bits(),
            414
        );
        let source = include_str!("lib.rs");
        let forbidden = [
            ["to", "words"].join("_"),
            ["as", "words"].join("_"),
            ["std", "fs"].join("::"),
            ["std", "net"].join("::"),
            ["std", "process"].join("::"),
            ["ser", "de"].concat(),
            ["sha", "2"].concat(),
        ];
        for forbidden in forbidden {
            assert!(!source.contains(&forbidden), "{forbidden}");
        }
    }
}
