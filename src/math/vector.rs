use std::ops;

use super::{Inner, Scalar, Coord};

// TODO Eq
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Vector {
    pub x: Scalar,
    pub y: Scalar,
}
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Position {
    pub x: Coord,
    pub y: Coord,
}

impl Vector {
/*
    pub fn rough_magnitude(self: Vector) -> Scalar {
        self.squared().rough_sqrt()
    }

    pub fn magnitude(self: Vector) -> Scalar {
        self.squared().sqrt()
    }
*/

    pub fn squared(self: Vector) -> Scalar {
        Vector::inner(self, self)
    }

    pub fn inner(self: Vector, other: Vector) -> Scalar {
        self.x * other.x + self.y * other.y
    }
}


// Operations
// ==========

impl ops::Neg for Vector {
    type Output = Self;
    fn neg(self: Self) -> Self {
        let x = -self.x;
        let y = -self.y;
        Vector { x, y }
    }
}

impl ops::Sub for Position {
    type Output = Vector;
    fn sub(self: Position, other: Position) -> Vector {
        let x = self.x - other.x;
        let y = self.y - other.y;
        Vector { x, y }
    }
}

// the following are all trivially defined in terms of their assignments

impl ops::Add for Vector {
    type Output = Vector;
    fn add(mut self: Vector, other: Vector) -> Vector {
        self += other;
        self
    }
}

impl ops::Add<Vector> for Position {
    type Output = Position;
    fn add(mut self: Position, other: Vector) -> Position {
        self += other;
        self
    }
}

impl ops::Add<Position> for Vector {
    type Output = Position;
    fn add(self: Vector, mut other: Position) -> Position {
        other += self;
        other
    }
}

impl ops::Sub for Vector {
    type Output = Vector;
    fn sub(mut self: Vector, other: Vector) -> Vector {
        self -= other;
        self
    }
}

impl ops::Sub<Vector> for Position {
    type Output = Position;
    fn sub(mut self: Position, other: Vector) -> Position {
        self -= other;
        self
    }
}

impl ops::Mul<Scalar> for Vector {
    type Output = Vector;
    fn mul(mut self: Vector, other: Scalar) -> Vector {
        self *= other;
        self
    }
}

impl ops::Mul<Vector> for Scalar {
    type Output = Vector;
    fn mul(self: Scalar, mut other: Vector) -> Vector {
        other *= self;
        other
    }
}

impl ops::Mul<Inner> for Vector {
    type Output = Vector;
    fn mul(mut self: Vector, other: Inner) -> Vector {
        self *= other;
        self
    }
}

impl ops::Mul<Vector> for Inner {
    type Output = Vector;
    fn mul(self: Inner, mut other: Vector) -> Vector {
        other *= self;
        other
    }
}

impl ops::Div<Scalar> for Vector {
    type Output = Vector;
    fn div(mut self: Vector, other: Scalar) -> Vector {
        self /= other;
        self
    }
}

impl ops::Div<Inner> for Vector {
    type Output = Vector;
    fn div(mut self: Vector, other: Inner) -> Vector {
        self /= other;
        self
    }
}


// Assignment Operations
// =====================

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

impl ops::MulAssign<Inner> for Vector {
    fn mul_assign(self: &mut Vector, other: Inner) {
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

impl ops::DivAssign<Inner> for Vector {
    fn div_assign(self: &mut Vector, other: Inner) {
        self.x /= other;
        self.y /= other;
    }
}

