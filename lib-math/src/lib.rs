extern crate fix;
extern crate typenum;

// bunch of trait implementations
mod scalar_assigns;
mod scalar_ops;
mod scalar_misc;
mod vector_assigns;
mod vector_ops;

use fix::aliases::binary::{IFix32, IFix64};
use typenum::N16;

type NarrowInner = IFix32<N16>;
type WideInner = IFix64<N16>;

#[derive(Clone, Copy, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar(NarrowInner);
#[derive(Clone, Copy, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord(WideInner);

#[derive(Clone, Copy, Hash, Default, PartialEq, Eq, Debug)]
pub struct Vector {
    pub x: Scalar,
    pub y: Scalar,
}
#[derive(Clone, Copy, Hash, Default, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: Coord,
    pub y: Coord,
}

fn narrow(val: WideInner) -> NarrowInner {
    NarrowInner::new(val.bits as i32)
}

fn wide(val: NarrowInner) -> WideInner {
    WideInner::new(val.bits as i64)
}

