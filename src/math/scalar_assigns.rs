use std::ops;

use {Scalar, Coord};

impl ops::AddAssign for Scalar {
    fn add_assign(self: &mut Scalar, other: Scalar) {
        self.0 += other.0;
    }
}

impl ops::AddAssign<Scalar> for Coord {
    fn add_assign(self: &mut Coord, other: Scalar) {
        self.0 += other.0;
    }
}

impl ops::SubAssign for Scalar {
    fn sub_assign(self: &mut Scalar, other: Scalar) {
        self.0 -= other.0;
    }
}

impl ops::SubAssign<Scalar> for Coord {
    fn sub_assign(self: &mut Coord, other: Scalar) {
        self.0 -= other.0;
    }
}

impl ops::MulAssign for Scalar {
    fn mul_assign(self: &mut Scalar, other: Scalar) {
        *self = *self * other;
    }
}

impl ops::DivAssign for Scalar {
    fn div_assign(self: &mut Scalar, other: Scalar) {
        *self = *self / other;
    }
}

impl ops::MulAssign<i64> for Scalar {
    fn mul_assign(self: &mut Scalar, other: i64) {
        self.0.bits *= other;
    }
}

impl ops::DivAssign<i64> for Scalar {
    fn div_assign(self: &mut Scalar, other: i64) {
        self.0.bits /= other;
    }
}

impl ops::RemAssign for Scalar {
    fn rem_assign(self: &mut Scalar, other: Scalar) {
        self.0 %= other.0;
    }
}

