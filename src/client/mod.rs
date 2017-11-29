use std::sync::mpsc;

use city_internal::sulphate;
use city_internal::sulphate::server;

use piston_window as app;

mod control;
mod draw;
mod trackers;
mod user_input;

type ClientData = ();

pub struct Client {
    vision: trackers::Perception,
    clock: server::Clock,
    input: user_input::Input,
    control: control::Controller,
}

fn server_init(
    space: &mut sulphate::EntityHeap,
    time: &mut sulphate::EventQueue
) -> ClientData {
    ()
}

pub fn start_game() -> Client {
    let (upd, clock, client_data) = server::start_server(server_init);
    Client::new(upd, clock, client_data)
}

impl Client {
    fn new(
        upd: mpsc::Sender<server::Interruption>,
        clock: server::Clock,
        data: ClientData,
    ) -> Client {
        let vision = trackers::Perception::new();
        let input = user_input::Input::new();
        let control = control::Controller::new(upd);

        Client { vision, clock, input, control }
    }

    pub fn on_update(self: &mut Self, _upd: app::UpdateArgs) {
        // do nothing ^-^
    }

    pub fn on_input(self: &mut Self, bin: app::ButtonArgs) {
        let action = self.input.interpret(bin);

        use client::user_input::DeviceUpdate::*;
        match action {
            Nop => (),
            ChangeMovement { dirs } => {
                self.control.ChangeMovement(dirs);
            },
        }
    }

    pub fn on_mouse_move(&mut self, mouse: [f64; 2]) {
        // self.input.on_mouse_move(mouse);
    }

    pub fn on_draw(
        &mut self,
        context: app::Context,
        graphics: &mut app::G2d,
        ren: app::RenderArgs
    ) {
        let now_rt = ::std::time::Instant::now();
        let clock: &server::ClockMethods = &mut self.clock;
        let now = clock.in_game(now_rt);
        clock.finished_cycle(now_rt, now);


        app::clear([0.0, 0.0, 0.0, 1.0], graphics);

        let corner = context.transform;
        let center_x = (ren.width / 2) as f64;
        let center_y = (ren.height / 2) as f64;
        let center = app::Transformed::trans(corner, center_x, center_y);

        for image in &self.vision {
            draw::draw(image, now, center, graphics);
        }
    }
}

