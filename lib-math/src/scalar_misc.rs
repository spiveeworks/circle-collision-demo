use std::fmt;
use std::cmp;

use super::{Scalar, Coord, NarrowInner};


impl Scalar {
    pub fn from_bits(bits: i32) -> Scalar {
        Scalar(NarrowInner::new(bits))
    }

    pub fn into_bits(self: Scalar) -> i32 {
        self.0.bits
    }

    pub fn rough_sqrt(self: Scalar) -> Scalar {
        // this is x * 2 ^ 16
        let bits = self.0.bits;
        // this is root_x * 2 ^ 8
        let result = rough_sqrt(bits, 8, 6);
        Scalar::from_bits(result << 8)
    }

    pub fn sqrt(self: Scalar) -> Scalar {
        self.rough_sqrt()
    }

    pub fn squared(self: Scalar) -> Scalar {
        self * self
    }
}

// note there is a nice approximation algorithm at
// https://users.rust-lang.org/t/integer-square-root-algorithm/13529/5
// but this is fine for now
fn rough_sqrt(val: i32, magnitude: i8, iterations: u8) -> i32 {
    if val == 0 { return 0; }
    let mut result = val >> ((magnitude + 16) / 2);
    for _ in 0 .. iterations {
        result = val / 2 / result + result / 2;
    }
    result
}

#[cfg(test)]
mod test_rough_sqrt {
    #[test]
    fn test_sqrts() {
        for i in 0..5000 {
            test_sqrt(i);
        }
    }

    fn test_sqrt(num: i16) {
        let val: ::Scalar = num.into();
        let root = val.rough_sqrt();
        let approx = root.squared();
        // very generous test
        assert!(
            num - 1 < approx && approx < num + 1,
            "Scalar::sqrt({}.into()) is horrible", num
        );
    }
}


impl From<i16> for Scalar {
    fn from(val: i16) -> Scalar {
        Scalar(NarrowInner::new((val as i32) << 16))
    }
}

impl From<Scalar> for i16 {
    fn from(val: Scalar) -> i16 {
        (val.0.bits >> 16) as i16
    }
}

/*
// more accurate to do <f64 as From>::from(x) as f32
impl From<Scalar> for f32 {
    fn from(val: Scalar) -> f32 {
        val.0.bits as f32 / (1 << 16) as f32
    }
}
*/

impl From<Scalar> for f64 {
    fn from(val: Scalar) -> f64 {
        val.0.bits as f64 / (1 << 16) as f64
    }
}


impl PartialEq<i16> for Scalar {
    fn eq(self: &Scalar, other: &i16) -> bool {
        let scalar_other: Scalar = (*other).into();
        *self == scalar_other
    }
}

impl PartialEq<Scalar> for i16 {
    fn eq(self: &i16, other: &Scalar) -> bool {
        let scalar_self: Scalar = (*self).into();
        scalar_self == *other
    }
}

impl PartialOrd<i16> for Scalar {
    fn partial_cmp(self: &Scalar, other: &i16) -> Option<cmp::Ordering> {
        let scalar_other: Scalar = (*other).into();
        PartialOrd::partial_cmp(self, &scalar_other)
    }
}

impl PartialOrd<Scalar> for i16 {
    fn partial_cmp(self: &i16, other: &Scalar) -> Option<cmp::Ordering> {
        let scalar_self: Scalar = (*self).into();
        PartialOrd::partial_cmp(&scalar_self, other)
    }
}


impl fmt::Debug for Coord {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl fmt::Debug for Scalar {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Coord {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let x = self.0.bits;
        let hi = x >> 16;
        let lo_mask = (1 << 16) - 1;
        let lo = x & lo_mask;
        write!(f, "{}", hi)?;
        if lo > 0 {
            write!(f, ".")?;
            let mut remaining = lo.abs();
            while remaining > 0 {
                remaining *= 10;
                write!(f, "{}", remaining >> 16)?;
                remaining &= lo_mask;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Scalar {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.0.bits > 0 {
            write!(f, "+")?;
        }
        let as_coord = Coord::default() + self.clone();
        write!(f, "{}", as_coord)
    }
}



