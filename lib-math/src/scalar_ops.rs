use std::ops;

use fix::aliases::binary::IFix64;
use typenum::N32;

use {Scalar, Coord, wide, narrow};

impl ops::Neg for Scalar {
    type Output = Self;
    fn neg(self: Self) -> Self {
        Scalar(-self.0)
    }
}

impl ops::Add for Scalar {
    type Output = Scalar;
    fn add(self: Scalar, other: Scalar) -> Scalar {
        Scalar(self.0 + other.0)
    }
}

impl ops::Add<Scalar> for Coord {
    type Output = Coord;
    fn add(self: Coord, other: Scalar) -> Coord {
        Coord(self.0 + wide(other.0))
    }
}

impl ops::Add<Coord> for Scalar {
    type Output = Coord;
    fn add(self: Scalar, other: Coord) -> Coord {
        Coord(wide(self.0) + other.0)
    }
}

impl ops::Sub for Scalar {
    type Output = Scalar;
    fn sub(self: Scalar, other: Scalar) -> Scalar {
        Scalar(self.0 - other.0)
    }
}

impl ops::Sub<Scalar> for Coord {
    type Output = Coord;
    fn sub(self: Coord, other: Scalar) -> Coord {
        Coord(self.0 - wide(other.0))
    }
}

impl ops::Mul for Scalar {
    type Output = Scalar;
    fn mul(self: Scalar, other: Scalar) -> Scalar {
        let prod = wide(self.0) * wide(other.0);
        Scalar(narrow(prod.convert()))
    }
}

impl ops::Div for Scalar {
    type Output = Scalar;
    fn div(self: Scalar, other: Scalar) -> Scalar {
        let nume: IFix64<N32> = wide(self.0).convert();
        let quot = nume / wide(other.0);
        Scalar(narrow(quot))
    }
}

impl ops::Rem for Scalar {
    type Output = Scalar;
    fn rem(self: Scalar, other: Scalar) -> Scalar {
        Scalar(self.0 % other.0)
    }
}

