use {Vector, Scalar};

impl Vector {
    pub fn rough_magnitude(self: Vector) -> Scalar {
        self.squared().rough_sqrt()
    }

    pub fn magnitude(self: Vector) -> Scalar {
        self.squared().sqrt()
    }

    pub fn squared(self: Vector) -> Scalar {
        Vector::inner(self, self)
    }

    pub fn inner(self: Vector, other: Vector) -> Scalar {
        self.x * other.x + self.y * other.y
    }
}
