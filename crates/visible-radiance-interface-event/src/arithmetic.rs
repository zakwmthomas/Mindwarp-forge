use core::cmp::Ordering;
use crypto_bigint::{CheckedAdd, NonZero, U512};

use crate::VisibleRadianceInterfaceError;

pub(crate) const PRECISIONS: [u16; 3] = [96, 128, 160];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Signed512 {
    negative: bool,
    magnitude: U512,
}

impl Signed512 {
    pub(crate) fn zero() -> Self {
        Self::new(false, U512::ZERO)
    }
    pub(crate) fn one() -> Self {
        Self::new(false, U512::ONE)
    }
    pub(crate) fn from_i128(value: i128) -> Self {
        Self::new(value.is_negative(), U512::from(value.unsigned_abs()))
    }
    pub(crate) fn new(negative: bool, magnitude: U512) -> Self {
        Self {
            negative: negative && magnitude != U512::ZERO,
            magnitude,
        }
    }
    pub(crate) fn is_negative(&self) -> bool {
        self.negative
    }
    pub(crate) fn magnitude(&self) -> &U512 {
        &self.magnitude
    }
    pub(crate) fn bits(&self) -> u16 {
        self.magnitude.bits_vartime() as u16
    }
    pub(crate) fn canonical_decimal(&self) -> String {
        if self.magnitude == U512::ZERO {
            return "0".to_owned();
        }
        let ten = NonZero::new(U512::from(10_u8)).expect("ten is nonzero");
        let mut remaining = self.magnitude;
        let mut reversed = Vec::new();
        while remaining != U512::ZERO {
            let (quotient, remainder) = remaining.div_rem_vartime(&ten);
            let digit = (0_u8..=9)
                .find(|value| remainder == U512::from(*value))
                .expect("remainder below ten");
            reversed.push(char::from(b'0' + digit));
            remaining = quotient;
        }
        if self.negative {
            reversed.push('-');
        }
        reversed.into_iter().rev().collect()
    }
    pub(crate) fn checked_neg(&self) -> Self {
        Self::new(!self.negative, self.magnitude)
    }
    pub(crate) fn checked_add(&self, other: &Self) -> Result<Self, VisibleRadianceInterfaceError> {
        if self.negative == other.negative {
            let magnitude = Option::<U512>::from(self.magnitude.checked_add(&other.magnitude))
                .ok_or(VisibleRadianceInterfaceError::ArithmeticDefect(
                    "signed-magnitude addition overflow",
                ))?;
            Ok(Self::new(self.negative, magnitude))
        } else {
            match self.magnitude.cmp(&other.magnitude) {
                Ordering::Greater | Ordering::Equal => Ok(Self::new(
                    self.negative,
                    self.magnitude.wrapping_sub(&other.magnitude),
                )),
                Ordering::Less => Ok(Self::new(
                    other.negative,
                    other.magnitude.wrapping_sub(&self.magnitude),
                )),
            }
        }
    }
    pub(crate) fn checked_sub(&self, other: &Self) -> Result<Self, VisibleRadianceInterfaceError> {
        self.checked_add(&other.checked_neg())
    }
    pub(crate) fn checked_mul(&self, other: &Self) -> Result<Self, VisibleRadianceInterfaceError> {
        let magnitude = Option::<U512>::from(self.magnitude.checked_mul(&other.magnitude)).ok_or(
            VisibleRadianceInterfaceError::ArithmeticDefect(
                "signed-magnitude multiplication overflow",
            ),
        )?;
        Ok(Self::new(self.negative ^ other.negative, magnitude))
    }
    pub(crate) fn checked_shl(&self, shift: u16) -> Result<Self, VisibleRadianceInterfaceError> {
        let magnitude = self
            .magnitude
            .overflowing_shl_vartime(u32::from(shift))
            .ok_or(VisibleRadianceInterfaceError::ArithmeticDefect(
                "signed-magnitude shift overflow",
            ))?;
        Ok(Self::new(self.negative, magnitude))
    }
    fn div_parts(
        &self,
        denominator: &Self,
    ) -> Result<(Self, bool, bool), VisibleRadianceInterfaceError> {
        let divisor = Option::<NonZero<U512>>::from(NonZero::new(denominator.magnitude)).ok_or(
            VisibleRadianceInterfaceError::ArithmeticDefect("directed division by zero"),
        )?;
        let (quotient, remainder) = self.magnitude.div_rem_vartime(&divisor);
        let negative = self.negative ^ denominator.negative;
        Ok((
            Self::new(negative, quotient),
            remainder != U512::ZERO,
            negative,
        ))
    }
    pub(crate) fn div_floor(
        &self,
        denominator: &Self,
    ) -> Result<Self, VisibleRadianceInterfaceError> {
        let (quotient, remainder, negative) = self.div_parts(denominator)?;
        if negative && remainder {
            quotient.checked_sub(&Self::one())
        } else {
            Ok(quotient)
        }
    }
    pub(crate) fn div_ceil(
        &self,
        denominator: &Self,
    ) -> Result<Self, VisibleRadianceInterfaceError> {
        let (quotient, remainder, negative) = self.div_parts(denominator)?;
        if !negative && remainder {
            quotient.checked_add(&Self::one())
        } else {
            Ok(quotient)
        }
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
pub(crate) struct FixedInterval {
    pub(crate) lower: Signed512,
    pub(crate) upper: Signed512,
    pub(crate) bits: u16,
}

impl FixedInterval {
    pub(crate) fn new(
        lower: Signed512,
        upper: Signed512,
        bits: u16,
    ) -> Result<Self, VisibleRadianceInterfaceError> {
        if lower > upper || !PRECISIONS.contains(&bits) {
            return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                "reversed or unsupported fixed interval",
            ));
        }
        Ok(Self { lower, upper, bits })
    }
    pub(crate) fn integer(value: i128, bits: u16) -> Result<Self, VisibleRadianceInterfaceError> {
        let raw = Signed512::from_i128(value).checked_shl(bits)?;
        Self::new(raw.clone(), raw, bits)
    }
    pub(crate) fn unsigned_ratio(
        numerator: U512,
        denominator: U512,
        bits: u16,
    ) -> Result<Self, VisibleRadianceInterfaceError> {
        let shifted = Signed512::new(false, numerator).checked_shl(bits)?;
        let divisor = Signed512::new(false, denominator);
        Self::new(
            shifted.div_floor(&divisor)?,
            shifted.div_ceil(&divisor)?,
            bits,
        )
    }
    fn compatible(&self, other: &Self) -> Result<(), VisibleRadianceInterfaceError> {
        if self.bits != other.bits {
            return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                "incompatible fixed intervals",
            ));
        }
        Ok(())
    }
    pub(crate) fn add(&self, other: &Self) -> Result<Self, VisibleRadianceInterfaceError> {
        self.compatible(other)?;
        Self::new(
            self.lower.checked_add(&other.lower)?,
            self.upper.checked_add(&other.upper)?,
            self.bits,
        )
    }
    pub(crate) fn sub(&self, other: &Self) -> Result<Self, VisibleRadianceInterfaceError> {
        self.compatible(other)?;
        Self::new(
            self.lower.checked_sub(&other.upper)?,
            self.upper.checked_sub(&other.lower)?,
            self.bits,
        )
    }
    pub(crate) fn mul(&self, other: &Self) -> Result<Self, VisibleRadianceInterfaceError> {
        self.compatible(other)?;
        let products = [
            self.lower.checked_mul(&other.lower)?,
            self.lower.checked_mul(&other.upper)?,
            self.upper.checked_mul(&other.lower)?,
            self.upper.checked_mul(&other.upper)?,
        ];
        let scale = Signed512::one().checked_shl(self.bits)?;
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
            self.bits,
        )
    }
    pub(crate) fn div(&self, other: &Self) -> Result<Self, VisibleRadianceInterfaceError> {
        self.compatible(other)?;
        if other.lower <= Signed512::zero() && other.upper >= Signed512::zero() {
            return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                "fixed denominator interval contains zero",
            ));
        }
        let mut floors = Vec::with_capacity(4);
        let mut ceils = Vec::with_capacity(4);
        for numerator in [&self.lower, &self.upper] {
            for denominator in [&other.lower, &other.upper] {
                let scaled = numerator.checked_shl(self.bits)?;
                floors.push(scaled.div_floor(denominator)?);
                ceils.push(scaled.div_ceil(denominator)?);
            }
        }
        Self::new(
            floors.into_iter().min().expect("four quotients"),
            ceils.into_iter().max().expect("four quotients"),
            self.bits,
        )
    }
    pub(crate) fn square(&self) -> Result<Self, VisibleRadianceInterfaceError> {
        if self.lower <= Signed512::zero() && self.upper >= Signed512::zero() {
            let a = self.lower.checked_mul(&self.lower)?;
            let b = self.upper.checked_mul(&self.upper)?;
            let maximum = if a >= b { a } else { b };
            let scale = Signed512::one().checked_shl(self.bits)?;
            Self::new(Signed512::zero(), maximum.div_ceil(&scale)?, self.bits)
        } else {
            self.mul(self)
        }
    }
    pub(crate) fn sqrt(&self) -> Result<Self, VisibleRadianceInterfaceError> {
        if self.lower.is_negative() {
            return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                "negative fixed square root",
            ));
        }
        let lower_radicand = self
            .lower
            .magnitude()
            .overflowing_shl_vartime(u32::from(self.bits))
            .ok_or(VisibleRadianceInterfaceError::ArithmeticDefect(
                "square-root radicand overflow",
            ))?;
        let upper_radicand = self
            .upper
            .magnitude()
            .overflowing_shl_vartime(u32::from(self.bits))
            .ok_or(VisibleRadianceInterfaceError::ArithmeticDefect(
                "square-root radicand overflow",
            ))?;
        let lower = lower_radicand.floor_sqrt_vartime();
        let upper_floor = upper_radicand.floor_sqrt_vartime();
        let exact = Option::<U512>::from(upper_floor.checked_mul(&upper_floor))
            .is_some_and(|square| square == upper_radicand);
        let upper = if exact {
            upper_floor
        } else {
            Option::<U512>::from(upper_floor.checked_add(&U512::ONE)).ok_or(
                VisibleRadianceInterfaceError::ArithmeticDefect("square-root ceiling overflow"),
            )?
        };
        Self::new(
            Signed512::new(false, lower),
            Signed512::new(false, upper),
            self.bits,
        )
    }
    pub(crate) fn intersect_unit(&self) -> Result<Self, VisibleRadianceInterfaceError> {
        let one = Signed512::one().checked_shl(self.bits)?;
        Self::new(
            self.lower.clone().max(Signed512::zero()),
            self.upper.clone().min(one),
            self.bits,
        )
    }
    pub(crate) fn project(
        &self,
        target_bits: u16,
    ) -> Result<ProjectedInterval, VisibleRadianceInterfaceError> {
        if target_bits > self.bits {
            return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                "projection increases precision",
            ));
        }
        let divisor = Signed512::one().checked_shl(self.bits - target_bits)?;
        ProjectedInterval::new(
            self.lower.div_floor(&divisor)?,
            self.upper.div_ceil(&divisor)?,
            target_bits,
        )
    }
    pub(crate) fn max_bits(&self) -> u16 {
        self.lower.bits().max(self.upper.bits())
    }
    pub(crate) fn contains_integer(
        &self,
        value: i128,
    ) -> Result<bool, VisibleRadianceInterfaceError> {
        let raw = Signed512::from_i128(value).checked_shl(self.bits)?;
        Ok(self.lower <= raw && raw <= self.upper)
    }
    pub(crate) fn overlaps(&self, other: &Self) -> Result<bool, VisibleRadianceInterfaceError> {
        self.compatible(other)?;
        Ok(self.lower <= other.upper && other.lower <= self.upper)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProjectedInterval {
    pub(crate) lower: Signed512,
    pub(crate) upper: Signed512,
    pub(crate) bits: u16,
}
impl ProjectedInterval {
    pub(crate) fn new(
        lower: Signed512,
        upper: Signed512,
        bits: u16,
    ) -> Result<Self, VisibleRadianceInterfaceError> {
        if lower > upper {
            return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                "empty projected interval",
            ));
        }
        Ok(Self { lower, upper, bits })
    }
    pub(crate) fn intersect(&self, other: &Self) -> Result<Self, VisibleRadianceInterfaceError> {
        if self.bits != other.bits {
            return Err(VisibleRadianceInterfaceError::ArithmeticDefect(
                "projected scale mismatch",
            ));
        }
        Self::new(
            self.lower.clone().max(other.lower.clone()),
            self.upper.clone().min(other.upper.clone()),
            self.bits,
        )
    }
    pub(crate) fn certified_one_unit(&self) -> Result<bool, VisibleRadianceInterfaceError> {
        Ok(self.upper.checked_sub(&self.lower)? <= Signed512::one())
    }
}

pub(crate) fn checked_u512_product(values: &[U512]) -> Result<U512, VisibleRadianceInterfaceError> {
    values.iter().try_fold(U512::ONE, |accumulator, value| {
        Option::<U512>::from(accumulator.checked_mul(value)).ok_or(
            VisibleRadianceInterfaceError::ArithmeticDefect("unsigned 512-bit product overflow"),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn directed_signed_magnitude_division_has_mathematical_floor_and_ceiling() {
        let minus_five = Signed512::from_i128(-5);
        let two = Signed512::from_i128(2);
        assert_eq!(
            minus_five.div_floor(&two).unwrap(),
            Signed512::from_i128(-3)
        );
        assert_eq!(minus_five.div_ceil(&two).unwrap(), Signed512::from_i128(-2));
        let minus_two = Signed512::from_i128(-2);
        assert_eq!(
            minus_five.div_floor(&minus_two).unwrap(),
            Signed512::from_i128(2)
        );
        assert_eq!(
            minus_five.div_ceil(&minus_two).unwrap(),
            Signed512::from_i128(3)
        );
        let minus_one = Signed512::from_i128(-1);
        assert_eq!(minus_one.div_floor(&two).unwrap(), Signed512::from_i128(-1));
        assert_eq!(minus_one.div_ceil(&two).unwrap(), Signed512::zero());
    }
    #[test]
    fn native_limbs_are_absent_from_target_neutral_decimal_codec() {
        assert_eq!(
            Signed512::from_i128(-123456789).canonical_decimal(),
            "-123456789"
        );
        let forbidden = ["as_", "words"].concat();
        assert!(!include_str!("arithmetic.rs").contains(&forbidden));
    }
}
