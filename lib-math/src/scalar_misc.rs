use std::cmp;
use std::fmt;
use std::time;

use super::{Scalar, Coord, Inner};


impl Scalar {
    pub fn from_bits(bits: i64) -> Scalar {
        Scalar(Inner::new(bits))
    }

    pub fn into_bits(self: Scalar) -> i64 {
        self.0.bits
    }

    pub fn rough_sqrt(self: Scalar) -> Scalar {
        // this is x * 2 ^ 16
        let bits = self.0.bits;
        // debug assert since newton's method will work... strangely
        // although we just integer overflow, which is less strange
        debug_assert!(bits >= 0, "Square root of negative number");
        // this is root_x * 2 ^ 8
        let result = rough_sqrt(bits as u32, 8, 6);
        Scalar::from_bits((result as i64) << 8)
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
fn rough_sqrt(val: u32, magnitude: i8, iterations: u8) -> u32 {
    let mut result = val >> ((magnitude + 16) / 2);
    // this is similar to an epsilon value, but it will only last
    // the specified number of iterations before it becomes zero
    // meaning it also gives sqrt(0) = 0
    result += 1 << (iterations - 1);
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
            test_sqrt_err(i);
        }
    }

    fn test_sqrt_err(num: i32) {
        let val: ::Scalar = num.into();

        let root = val.rough_sqrt();
        let approx = root.squared();

        // very generous test
        assert!(
            val - 1.into() < approx && approx <= val,
            "Scalar::sqrt({}) is horrible, [{}^2 = {}]", val, root, approx
        );
    }

    fn test_small_sqrt(num: i64) {
        let val = ::Scalar::from_bits(num);

        let root = val.rough_sqrt();
        let approx = root.squared();

        // very generous test
        assert!(
             (val - approx).squared() < 1,
            "Scalar::sqrt({}) is horrible, [{}^2 = {}]", val, root, approx
        );
    }

    #[test]
    fn test_small_sqrts_exhaustive() {
        println!("Testing");
        let max_val: ::Scalar = 8.into();
        for val in 0..max_val.into_bits() {
            test_small_sqrt(val);
        }
    }
}


impl From<i32> for Scalar {
    fn from(val: i32) -> Scalar {
        Scalar(Inner::new((val as i64) << 16))
    }
}

impl From<Scalar> for i32 {
    fn from(val: Scalar) -> i32 {
        (val.0.bits >> 16) as i32
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

impl From<time::Duration> for Scalar {
    fn from(duration: time::Duration) -> Scalar {
        let seconds = duration.as_secs();
        let nanos = duration.subsec_nanos();
        let time_s: Scalar = (seconds as i32).into();
        let time_n_num: Scalar = (nanos as i32).into();
        let time_n = time_n_num / 1_000_000_000;
        time_s + time_n
    }
}

impl From<Scalar> for time::Duration {
    fn from(duration: Scalar) -> time::Duration {
        let time_s: i32 = duration.into();
        let time_frac = duration - time_s.into();
        let time_n: i32 = (time_frac * 1_000_000_000).into();
        time::Duration::new(time_s as u64, time_n as u32)
    }
}

impl PartialEq<i32> for Scalar {
    fn eq(self: &Scalar, other: &i32) -> bool {
        let scalar_other: Scalar = (*other).into();
        *self == scalar_other
    }
}

impl PartialEq<Scalar> for i32 {
    fn eq(self: &i32, other: &Scalar) -> bool {
        let scalar_self: Scalar = (*self).into();
        scalar_self == *other
    }
}

impl PartialOrd<i32> for Scalar {
    fn partial_cmp(self: &Scalar, other: &i32) -> Option<cmp::Ordering> {
        let scalar_other: Scalar = (*other).into();
        PartialOrd::partial_cmp(self, &scalar_other)
    }
}

impl PartialOrd<Scalar> for i32 {
    fn partial_cmp(self: &i32, other: &Scalar) -> Option<cmp::Ordering> {
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



