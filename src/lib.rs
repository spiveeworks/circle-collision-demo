extern crate piston_app;
extern crate piston_window;
extern crate sulphate_lib;

mod animation;
pub mod maths;
mod sulphate;

use std::time;

use piston_window::*;

use maths::units;

pub struct App {
    priv_game: sulphate::PrivateGame,
    clock: Clock,
    camera_position: units::Position,
}

type Clock = sulphate_lib::clock::Simple<units::Time>;

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
        args: RenderArgs,
    ) {
        let trans = centre.transform;
        let trans = trans.trans(
            args.draw_width as f64 / 2.0,
            args.draw_height as f64 / 2.0,
        );

        let now_rt = time::Instant::now();
        let now = self.clock.time(now_rt);
        let position = self.game().matter.body.position(now);
        let pos = position - self.camera_position;
        let trans = trans.trans(pos.x.into(), pos.y.into());

        let radius = 50.0;
        let rect = [-radius, -radius, 2.0 * radius, 2.0 * radius];

        ellipse([1.0; 4], rect, trans, graphics);
    }

    fn on_update(
        self: &mut Self,
        _args: UpdateArgs,
    ) {
        use sulphate_lib::event_queue::Simulation;
        // TODO a .now() method would be useful upstream
        let now_rt = time::Instant::now();
        let now = self.clock.time(now_rt);
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

    let clock = Clock::new(now);

    let camera_position = Default::default();

    App { priv_game, clock, camera_position }
}

