use lib_math;

pub type Time = lib_math::Coord;
pub type Duration = lib_math::Scalar;

pub type Speed = lib_math::Scalar;
pub type Distance = lib_math::Scalar;

pub type Scalar = lib_math::Scalar;
pub type Coord = lib_math::Coord;

// meant to be a minimal unit of time for rendering
// things that ought to be seen, will last at least this long
// so by keeping the framerate above 16, these things will be seen!

pub const MOMENT_RATE: u16 = 16;

pub fn moments(num: i32) -> Duration {
    let as_scalar: Scalar = num.into();
    as_scalar / MOMENT_RATE as i64
}

pub type Displacement = lib_math::Vector;
pub type Velocity = lib_math::Vector;
pub type Position = lib_math::Position;

// this is useful, for example, when you want the inner of a displacement and a
// velocity, and don't want something misleading such as
// units::Displacement::inner(velocity, displacement)
pub type Vector = lib_math::Vector;
