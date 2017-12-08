use std::sync::mpsc;

use city_internal::entities::player;
use city_internal::units;
use city_internal::space;
use city_internal::sulphate;
use city_internal::sulphate::server;

use piston_window as app;

mod draw;
mod trackers;
mod user_input;

struct ClientData {
    recv_upd: mpsc::Receiver<player::Update>,
}

pub struct Client {
    vision: trackers::Perception,
    clock: server::Clock,
    input: user_input::Input,
    send_upd: mpsc::Sender<server::Interruption>,
    recv_upd: mpsc::Receiver<player::Update>,
}

fn server_init(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    matter: &mut sulphate::EntityHeap,
) -> ClientData {
    let (player_send_upd, recv_upd) = mpsc::channel();
    let position = Default::default();
    player::Player::new(space, time, matter, position, player_send_upd);
    ClientData { recv_upd }
}

pub fn start_game() -> Client {
    let (send_upd, clock, client_data) = server::start_server(server_init);
    Client::new(send_upd, clock, client_data)
}

impl Client {
    fn new(
        send_upd: mpsc::Sender<server::Interruption>,
        clock: server::Clock,
        data: ClientData,
    ) -> Client {
        let id = match data.recv_upd.recv() {
            Ok(upd) => match upd.what {
                player::UpdateData::Created { id, .. } => id,
                _ => panic!("Player didn't send Created update first"),
            },
            Err(_) => panic!("Player gave bad Receiver"),
        };
        let vision = trackers::Perception::new(id);
        let input = user_input::Input::new();
        let ClientData { recv_upd } = data;

        Client { vision, clock, input, send_upd, recv_upd }
    }

    pub fn on_update(self: &mut Self, _upd: app::UpdateArgs) {
        for upd in self.recv_upd.try_iter() {
            use city_internal::entities::player::UpdateData::*;
            match upd.what {
                Created { .. } => unreachable!(),
                Vision { before, after } => {
                    self.vision.apply_update(before, after);
                },
            }
        }
    }

    pub fn on_input(self: &mut Self, bin: app::ButtonArgs) {
        let action = self.input.interpret(bin);

        use client::user_input::DeviceUpdate::*;
        match action {
            Nop => (),
            ChangeMovement { dirs } => {
                self.change_movement(dirs);
            },
        }
    }

    fn change_movement(self: &Self, dirs: user_input::DirPad<bool>) {
        let mut velocity: units::Velocity = Default::default();
        let speed: units::Scalar = 150.into();

        if dirs.up    { velocity.y -= speed; }
        if dirs.down  { velocity.y += speed; }
        if dirs.left  { velocity.x -= speed; }
        if dirs.right { velocity.x += speed; }

        if velocity.x != 0 && velocity.y != 0 {
            velocity *= 5;
            velocity /= 7;
        }

        let id = self.vision.player_id();
        let control = player::Control::Move { velocity };
        let interruption = server::Interruption::PlayerUpdate { id, control };
        self.send_upd.send(interruption).expect("Player entity disconnected");
    }

    pub fn on_mouse_move(&mut self, _mouse: [f64; 2]) {
        // self.input.on_mouse_move(mouse);
    }

    pub fn on_draw(
        &mut self,
        context: app::Context,
        graphics: &mut app::G2d,
        ren: app::RenderArgs
    ) {
        let now_rt = ::std::time::Instant::now();
        let clock: &mut server::ClockMethods = &mut self.clock;
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

