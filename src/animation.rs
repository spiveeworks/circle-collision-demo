use maths::units;
use sulphate;

pub struct Body {
    position: units::Position,
    velocity: units::Velocity,
    updated: units::Time,
}

impl Body {
    pub fn position(self: &Self, now: units::Time) -> units::Position {
        self.position + (now - self.updated) * self.velocity
    }
}

pub struct Matter {
    pub body: Body,
    pub manner: Manner,
    pub state: u32,
}

impl Matter {
    pub fn new() -> Self {
        let position = Default::default();
        let velocity = Default::default();
        let updated = Default::default();
        let body = Body { position, velocity, updated };

        let manner = Manner::new();

        let state = 0;

        Matter { body, manner, state }
    }

    pub fn start(game: &mut sulphate::Game) {
        Matter::next_event(game);
    }

    fn step_program(self: &mut Self, now: units::Time) -> units::Duration {
        let program = self.manner.program[self.state as usize];
        let (state, duration, velocity) = program;

        self.state = state;
        self.body = Body {
            position: self.body.position(now),
            velocity,
            updated: now,
        };

        duration
    }

    pub fn next_event(game: &mut sulphate::Game) {
        let now = game.now();
        let duration = game.matter.step_program(now);

        let event = MatterEvent {};
        game.enqueue_event_relative(event, duration);
    }
}

pub struct Manner {
    program: Vec<(u32, units::Duration, units::Velocity)>,
}

impl Manner {
    pub fn new() -> Self {
        let mut program = Vec::new();
        program.push((1, 1.0.into(), units::Velocity::new(100.0,   0.0)));
        program.push((2, 1.0.into(), units::Velocity::new(  0.0, 100.0)));
        program.push((3, 2.0.into(), units::Velocity::new(-50.0,   0.0)));
        program.push((0, 0.5.into(), units::Velocity::new(  0.0, 200.0)));

        Manner { program }
    }
}

pub struct MatterEvent {
}

impl sulphate::Event for MatterEvent {
    fn invoke(self: Self, game: &mut sulphate::Game) {
        Matter::next_event(game);
    }
}

