extern crate fix;
extern crate typenum;

use std::fmt;
use std::ops;

use fix::aliases::binary::{IFix32, IFix64};
use typenum::{N16, N32};

// useful for Coord division operations (dividing a space into sections)
// pub type Int = i32;

type NarrowInner = IFix32<N16>;
type WideInner = IFix64<N16>;

#[derive(Clone, Copy, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar(NarrowInner);
#[derive(Clone, Copy, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord(WideInner);

#[derive(Clone, Copy, Hash, Default, Debug, PartialEq, Eq)]
pub struct Vector(Scalar, Scalar, Scalar);
#[derive(Clone, Copy, Hash, Default, Debug, PartialEq, Eq)]
pub struct Position(Coord, Coord, Coord);
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
