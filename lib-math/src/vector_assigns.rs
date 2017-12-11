use std::ops;

use {Vector, Position, Scalar};

impl ops::AddAssign for Vector {
    fn add_assign(self: &mut Vector, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::AddAssign<Vector> for Position {
    fn add_assign(self: &mut Position, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::SubAssign for Vector {
    fn sub_assign(self: &mut Vector, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::SubAssign<Vector> for Position {
    fn sub_assign(self: &mut Position, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::MulAssign<Scalar> for Vector {
    fn mul_assign(self: &mut Vector, other: Scalar) {
        self.x *= other;
        self.y *= other;
    }
}

impl ops::MulAssign<i64> for Vector {
    fn mul_assign(self: &mut Vector, other: i64) {
        self.x *= other;
        self.y *= other;
    }
}

impl ops::DivAssign<Scalar> for Vector {
    fn div_assign(self: &mut Vector, other: Scalar) {
        self.x /= other;
        self.y /= other;
    }
}

impl ops::DivAssign<i64> for Vector {
    fn div_assign(self: &mut Vector, other: i64) {
        self.x /= other;
        self.y /= other;
    }
}

