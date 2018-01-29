// bunch of trait implementations
mod scalar;
mod vector;

type Inner = f64;

pub use self::scalar::Scalar;
pub use self::scalar::Coord;

pub use self::vector::Vector;
pub use self::vector::Position;

