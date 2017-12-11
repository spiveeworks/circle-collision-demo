extern crate fix;
extern crate typenum;

// bunch of trait implementations
mod scalar_assigns;
mod scalar_ops;
mod scalar_misc;
mod vector_assigns;
mod vector_ops;
mod vector_misc;

use fix::aliases::binary::IFix64;
use typenum::N16;

type Inner = IFix64<N16>;

#[derive(Clone, Copy, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar(Inner);
#[derive(Clone, Copy, Hash, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord(Inner);

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

