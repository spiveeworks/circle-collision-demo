use lib_math;


pub type Time = lib_math::Coord;
pub type Duration = lib_math::Scalar;

// meant to be a minimal unit of time for rendering
// things that ought to be seen, will last at least this long
// so by keeping the framerate above 16, these things will be seen!

pub const MOMENT_RATE: u16 = 16;

pub fn moments(num: i16) -> Duration {
    let as_scalar: lib_math::Scalar = num.into();
    as_scalar / MOMENT_RATE as i32
}

pub type Displacement = lib_math::Vector;
pub type Velocity = lib_math::Vector;
pub type Position = lib_math::Position;


