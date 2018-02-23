extern crate piston_app;
extern crate piston_window;
extern crate sulphate_lib;

mod animation;
pub mod maths;
mod sulphate;

use piston_window::*;

use maths::units;

pub struct App {
    priv_game: sulphate::PrivateGame,
    clock: Clock,
    camera_position: units::Position,
}

struct Clock {
}

impl Clock {
    pub fn now(self: &Self) -> units::Time {
        Default::default()
    }
}

impl App {
    fn game(self: &mut Self) -> &mut sulphate::Game {
        &mut self.priv_game.inner
    }
}

impl piston_app::App for App {
    fn window_name() -> &'static str {
        "inductive programming"
    }

    fn window_starting_size() -> [u32; 2] {
        [600, 600]
    }

    fn on_draw(
        self: &mut Self,
        centre: Context,
        graphics: &mut G2d,
        _args: RenderArgs,
    ) {
        let now = self.clock.now();
        let position = self.game().matter.body.position(now);
        let pos = position - self.camera_position;
        let trans = centre.transform.trans(pos.x.into(), pos.y.into());

        let radius = 50.0;
        let rect = [-radius, -radius, 2.0 * radius, 2.0 * radius];

        ellipse([1.0; 4], rect, trans, graphics);
    }

    fn on_update(
        self: &mut Self,
        _args: UpdateArgs,
    ) {
        use sulphate_lib::event_queue::Simulation;
        let now = self.clock.now();
        self.priv_game.simulate(now);
    }

    fn on_input(
        self: &mut Self,
        _args: ButtonArgs,
    ) {
    }

    fn on_mouse_move(
        self: &mut Self,
        _mouse: [f64; 2],
    ) {
    }
}

pub fn start_app() -> App {
    let now = Default::default();
    let matter = animation::Matter::new();
    let inner = sulphate::Game::new(matter, now);
    let priv_game = sulphate::PrivateGame { inner };

    let clock = Clock {};

    let camera_position = Default::default();

    App { priv_game, clock, camera_position }
}

