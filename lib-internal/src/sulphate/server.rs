use std::sync::mpsc;
use std::time;
use std::thread;

use sulphate_lib::server;

use entities::player;
use space;
use sulphate;
use sulphate::clock;
use units;

pub enum Interruption {
    PlayerUpdate {
        id: sulphate::EntityId,
        control: player::Control
    },
    KillServer,
}

impl server::Interruption<units::Time, sulphate::World> for Interruption {
    fn update(
        self: Self,
        time: &mut sulphate::EventQueue,
        world: &mut sulphate::World,
    ) -> bool {
        use self::Interruption::*;
        match self {
            PlayerUpdate { id, control } => {
                player::Control::apply(
                    &mut world.space,
                    time,
                    &mut world.matter,
                    id,
                    control,
                );
            },
            KillServer => return true,
        }
        false
    }
}

type Server = server::Server<
    Clock,
    Interruption,
    units::Time,
    sulphate::World
>;
type Clock = clock::Simple<units::Time>;

fn create_server_local<F, R>(
    f: F,
    upd: mpsc::Receiver<Interruption>,
) -> (Server, Clock, R)
    where F: FnOnce(
                 &mut space::CollisionSpace,
                 &mut sulphate::EventQueue,
                 &mut sulphate::EntityHeap,
             ) -> R,
          R: Send + 'static,
{
    let initial_time = Default::default();
    let mut clock = clock::Simple::new(initial_time);
    clock.start(time::Instant::now());

    let mut space = space::CollisionSpace::new();
    let mut time = sulphate::EventQueue::new(initial_time);
    let mut matter = sulphate::EntityHeap::new();

    let r = f(&mut space, &mut time, &mut matter);

    let world = sulphate::World { space, matter };

    let server = Server::new(time, world, upd, clock.clone());

    (server, clock, r)
}

struct ServerWatcher {
    natural: bool
}

impl Drop for ServerWatcher {
    fn drop(self: &mut Self) {
        if self.natural {
            println!("Server closed without panicking");
        } else {
            println!("Server panicked!");
        }
    }
}


pub fn start_server<F, R>(f: F) -> (
    mpsc::Sender<Interruption>,
    Clock,
    R,
)
    where F: Send + 'static
           + FnOnce(
                 &mut space::CollisionSpace,
                 &mut sulphate::EventQueue,
                 &mut sulphate::EntityHeap,
             ) -> R,
          R: Send + 'static,
{
    let (upd, upd_recv) = mpsc::channel();
    let (send, recv) = mpsc::channel();

    thread::spawn(move || {
        let mut announce_shutdown = ServerWatcher { natural: false };

        let (mut server, clock, r) =
            create_server_local(f, upd_recv);
        send.send((clock, r)).expect("failed to send server result");
        server.run();

        announce_shutdown.natural = true;
    });

    let (clock, r) = recv.recv().expect("failed to receive server result");

    (upd, clock, r)
}
