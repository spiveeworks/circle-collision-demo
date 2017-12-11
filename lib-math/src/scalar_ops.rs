use std::ops;

use fix::aliases::binary::IFix64;
use typenum::N32;

use {Scalar, Coord};

impl ops::Neg for Scalar {
    type Output = Self;
    fn neg(self: Self) -> Self {
        Scalar(-self.0)
    }
}

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

impl ops::Sub for Coord {
    type Output = Scalar;
    fn sub(self: Coord, other: Coord) -> Scalar {
        Scalar(self.0 - other.0)
    }
}

impl ops::Mul for Scalar {
    type Output = Scalar;
    fn mul(self: Scalar, other: Scalar) -> Scalar {
        let prod = self.0 * other.0;
        Scalar(prod.convert())
    }
}

impl ops::Div for Scalar {
    type Output = Scalar;
    fn div(self: Scalar, other: Scalar) -> Scalar {
        let nume: IFix64<N32> = self.0.convert();
        let quot = nume / other.0;
        Scalar(quot)
    }
}

impl ops::Rem for Scalar {
    type Output = Scalar;
    fn rem(mut self: Scalar, other: Scalar) -> Scalar {
        self %= other;
        self
    }
}

impl ops::Mul<i64> for Scalar {
    type Output = Scalar;
    fn mul(mut self: Scalar, other: i64) -> Scalar {
        self *= other;
        self
    }
}

impl ops::Mul<Scalar> for i64 {
    type Output = Scalar;
    fn mul(self: i64, mut other: Scalar) -> Scalar {
        other *= self;
        other
    }
}

impl ops::Div<i64> for Scalar {
    type Output = Scalar;
    fn div(mut self: Scalar, other: i64) -> Scalar {
        self /= other;
        self
    }
}

