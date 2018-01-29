use std::ops;

use {Vector, Position, Scalar};

impl ops::Neg for Vector {
    type Output = Self;
    fn neg(self: Self) -> Self {
        let x = -self.x;
        let y = -self.y;
        Vector { x, y }
    }
}

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

impl ops::Sub for Position {
    type Output = Vector;
    fn sub(self: Position, other: Position) -> Vector {
        let x = self.x - other.x;
        let y = self.y - other.y;
        Vector { x, y }
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

impl ops::Mul<i64> for Vector {
    type Output = Vector;
    fn mul(mut self: Vector, other: i64) -> Vector {
        self *= other;
        self
    }
}

impl ops::Mul<Vector> for i64 {
    type Output = Vector;
    fn mul(self: i64, mut other: Vector) -> Vector {
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

impl ops::Div<i64> for Vector {
    type Output = Vector;
    fn div(mut self: Vector, other: i64) -> Vector {
        self /= other;
        self
    }
}

