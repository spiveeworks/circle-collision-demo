use maths::units;

pub struct Body {
    position: units::Position,
    _velocity: units::Velocity,
    _updated: units::Time,
}

impl Body {
    pub fn position(self: &Self, _now: units::Time) -> units::Position {
        self.position
    }
}

pub struct Matter {
    pub body: Body,
}

impl Matter {
    pub fn new() -> Self {
        let position = Default::default();
        let _velocity = Default::default();
        let _updated = Default::default();
        let body = Body { position, _velocity, _updated };

        Matter { body }
    }
}

