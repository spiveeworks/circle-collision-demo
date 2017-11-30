extern crate piston_window;

use piston_window::*;

extern crate city_internal;

mod client;


// as seen in https://github.com/PistonDevelopers/piston-examples/issues/336
fn build_window() -> PistonWindow {
    let title = "lil-city";
    let resolution = [600, 600];
    let opengl = OpenGL::V3_2;
    let mut window_result = WindowSettings::new(title, resolution)
        .exit_on_esc(true)
        .srgb(true)                      // try to init windowbuilder with srgb enabled
        .opengl(opengl)
        .build();
    if window_result.is_err() {   //if srgb=true fails, retry as srgb=false
        window_result = WindowSettings::new(title, resolution)
            .exit_on_esc(true)
            .srgb(false)                 // !!!
            .opengl(opengl)
            .build();
    }
    let window_inner = window_result
        .unwrap_or_else(|e| {
            panic!("Failed to build PistonWindow: {}", e);
        });

    PistonWindow::new(opengl, 0, window_inner)
}


fn main() {
    let mut client = client::start_game();

    let mut window: PistonWindow = build_window();

    while let Some(e) = window.next() {
        if let Some(ren) = e.render_args() {
            window.draw_2d(&e, |c, g| client.on_draw(c, g, ren));
        }
        if let Some(upd) = e.update_args() {
            client.on_update(upd);
        }
        if let Some(bin) = e.button_args() {
            client.on_input(bin);
        }
        if let Some(mouse) = e.mouse_cursor_args() {
            client.on_mouse_move(mouse);
        }
    }
}
