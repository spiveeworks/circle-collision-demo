use std::fmt;
use std::cmp;

use super::{Scalar, Coord, NarrowInner};

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



