use std::cmp;
use std::fmt;
use std::ops;

use super::Inner;

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Scalar {
    pub value: Inner
}
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Coord {
    pub value: Inner
}

// Misc Traits
// ===========

impl From<Inner> for Scalar {
    fn from(value: Inner) -> Scalar {
        debug_assert!(value.is_finite());
        Scalar { value }
    }
}

impl From<Scalar> for Inner {
    fn from(val: Scalar) -> Inner {
        val.value
    }
}

impl Eq for Scalar {
}

impl Eq for Coord {
}

impl Ord for Scalar {
    fn cmp(self: &Scalar, other: &Scalar) -> cmp::Ordering {
        PartialOrd::partial_cmp(self, other).expect("NaN comparison")
    }
}

impl Ord for Coord {
    fn cmp(self: &Coord, other: &Coord) -> cmp::Ordering {
        PartialOrd::partial_cmp(self, other).expect("NaN comparison")
    }
}

impl PartialEq<Inner> for Scalar {
    fn eq(self: &Scalar, other: &Inner) -> bool {
        self.value == *other
    }
}

impl PartialEq<Scalar> for Inner {
    fn eq(self: &Inner, other: &Scalar) -> bool {
        *self == other.value
    }
}

impl PartialOrd<Inner> for Scalar {
    fn partial_cmp(self: &Scalar, other: &Inner) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&self.value, other)
    }
}

impl PartialOrd<Scalar> for Inner {
    fn partial_cmp(self: &Inner, other: &Scalar) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(self, &other.value)
    }
}


impl fmt::Debug for Coord {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}

impl fmt::Debug for Scalar {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}

// Operations
// ==========

impl ops::Neg for Scalar {
    type Output = Self;
    fn neg(self: Self) -> Self {
        let value = -self.value;
        Scalar { value }
    }
}

impl ops::Sub for Coord {
    type Output = Scalar;
    fn sub(self: Coord, other: Coord) -> Scalar {
        let value = self.value - other.value;
        debug_assert!(value.is_finite());
        Scalar { value }
    }
}

// the following are all trivially defined in terms of their assignments

impl ops::Add for Scalar {
    type Output = Scalar;
    fn add(mut self: Scalar, other: Scalar) -> Scalar {
        self += other;
        self
    }
}

impl ops::Add<Scalar> for Coord {
    type Output = Coord;
    fn add(mut self: Coord, other: Scalar) -> Coord {
        self += other;
        self
    }
}

impl ops::Add<Coord> for Scalar {
    type Output = Coord;
    fn add(self: Scalar, mut other: Coord) -> Coord {
        other += self;
        other
    }
}

impl ops::Sub for Scalar {
    type Output = Scalar;
    fn sub(mut self: Scalar, other: Scalar) -> Scalar {
        self -= other;
        self
    }
}

impl ops::Sub<Scalar> for Coord {
    type Output = Coord;
    fn sub(mut self: Coord, other: Scalar) -> Coord {
        self -= other;
        self
    }
}

impl ops::Mul for Scalar {
    type Output = Scalar;
    fn mul(mut self: Scalar, other: Scalar) -> Scalar {
        self *= other;
        self
    }
}

impl ops::Div for Scalar {
    type Output = Scalar;
    fn div(mut self: Scalar, other: Scalar) -> Scalar {
        self /= other;
        self
    }
}

impl ops::Rem for Scalar {
    type Output = Scalar;
    fn rem(mut self: Scalar, other: Scalar) -> Scalar {
        self %= other;
        self
    }
}

impl ops::Mul<Inner> for Scalar {
    type Output = Scalar;
    fn mul(mut self: Scalar, other: Inner) -> Scalar {
        self *= other;
        self
    }
}

impl ops::Mul<Scalar> for Inner {
    type Output = Scalar;
    fn mul(self: Inner, mut other: Scalar) -> Scalar {
        other *= self;
        other
    }
}

impl ops::Div<Inner> for Scalar {
    type Output = Scalar;
    fn div(mut self: Scalar, other: Inner) -> Scalar {
        self /= other;
        self
    }
}

// Assignment Operations
// =====================

impl ops::AddAssign for Scalar {
    fn add_assign(self: &mut Scalar, other: Scalar) {
        self.value += other.value;
        debug_assert!(self.value.is_finite());
    }
}

impl ops::AddAssign<Scalar> for Coord {
    fn add_assign(self: &mut Coord, other: Scalar) {
        self.value += other.value;
        debug_assert!(self.value.is_finite());
    }
}

impl ops::SubAssign for Scalar {
    fn sub_assign(self: &mut Scalar, other: Scalar) {
        self.value -= other.value;
        debug_assert!(self.value.is_finite());
    }
}

impl ops::SubAssign<Scalar> for Coord {
    fn sub_assign(self: &mut Coord, other: Scalar) {
        self.value -= other.value;
        debug_assert!(self.value.is_finite());
    }
}

impl ops::MulAssign for Scalar {
    fn mul_assign(self: &mut Scalar, other: Scalar) {
        self.value *= other.value;
        debug_assert!(self.value.is_finite());
    }
}

impl ops::DivAssign for Scalar {
    fn div_assign(self: &mut Scalar, other: Scalar) {
        self.value /= other.value;
        debug_assert!(self.value.is_finite());
    }
}

impl ops::MulAssign<Inner> for Scalar {
    fn mul_assign(self: &mut Scalar, other: Inner) {
        self.value *= other;
        debug_assert!(self.value.is_finite());
    }
}

impl ops::DivAssign<Inner> for Scalar {
    fn div_assign(self: &mut Scalar, other: Inner) {
        self.value /= other;
        debug_assert!(self.value.is_finite());
    }
}

impl ops::RemAssign for Scalar {
    fn rem_assign(self: &mut Scalar, other: Scalar) {
        self.value %= other.value;
        debug_assert!(self.value.is_finite());
    }
}

