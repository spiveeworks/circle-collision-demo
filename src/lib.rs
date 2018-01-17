extern crate piston_app;
extern crate piston_window;

mod animation;

use piston_window::*;

pub struct App {
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
        _centre: Context,
        _graphics: &mut G2d,
        _args: RenderArgs,
    ) {
    }

    fn on_update(
        self: &mut Self,
        _args: UpdateArgs,
    ) {
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
    App {}
}

