use std::sync::mpsc;
use std::time;
use std::thread;

use sulphate_lib::server;

use entities::player;
use sulphate;
use units;

pub enum Interruption {
    PlayerUpdate {
        id: sulphate::EntityId,
        control: player::Control
    },
    KillServer,
}

impl server::Interruption<units::Time> for Interruption {
    fn update(
        self: Self,
        space: &mut sulphate::EntityHeap,
        time: &mut sulphate::EventQueue,
    ) -> bool {
        use self::Interruption::*;
        match self {
            PlayerUpdate { id, control } => {
                player::Control::apply(space, time, id, control);
            },
            KillServer => return true,
        }
        false
    }
}

fn duration_in_game(duration: time::Duration) -> units::Duration {
    let seconds = duration.as_secs();
    let nanos = duration.subsec_nanos();
    let time_s: units::Duration = (seconds as i16).into();
    let time_n_bits = ((nanos as u64) << 16) / 1_000_000_000;
    let time_n = units::Duration::from_bits(time_n_bits as i32);
    time_s + time_n
}

fn duration_real_time(duration: units::Duration) -> time::Duration {
    let time_s: i16 = duration.into();
    let time_bits = duration.into_bits();
    let time_n_bits = time_bits & ((1 << 16) - 1);
    let time_n = (time_n_bits as u64 * 1_000_000_000) >> 16;
    time::Duration::new(time_s as u64, time_n as u32)
}

#[derive(Clone)]
struct Simple {
    start_instant: Option<time::Instant>,
    last_time: units::Time,
}

impl Simple {
    fn new(start_time: units::Time) -> Simple {
        Simple {
            start_instant: None,
            last_time: start_time,
        }
    }

    fn elapsed_as_of(&self, now: time::Instant) -> time::Duration {
        if let Some(start) = self.start_instant {
            now.duration_since(start)
        } else {
            // time only passes if the clock has started
            time::Duration::new(0,0)
        }
    }

    fn time(&self, now: time::Instant) -> units::Time {
        let elapsed = self.elapsed_as_of(now);
        self.last_time + duration_in_game(elapsed)
    }

    fn stop(&mut self, now: time::Instant) {
        self.last_time = self.time(now);
        self.start_instant = None;
    }

    fn start(&mut self, now: time::Instant) {
        self.stop(now);
        self.start_instant = Some(now);
    }
}

#[derive(Clone)]
pub struct Clock(Simple);

pub trait ClockMethods {
    fn in_game(self: &mut Self, now: time::Instant) -> units::Time;
    fn minimum_wait(
        self: &mut Self,
        now: units::Time,
        until: units::Time,
    ) -> time::Duration;
    fn finished_cycle(
        self: &mut Self,
        now: time::Instant,
        in_game: units::Time
    );
    fn end_cycles(self: &mut Self);
}

impl<C> ClockMethods for C where C: server::Clock<units::Time> {
    fn in_game(self: &mut Self, now: time::Instant) -> units::Time {
        server::Clock::in_game(self, now)
    }

    fn minimum_wait(
        self: &mut Self,
        now: units::Time,
        until: units::Time,
    ) -> time::Duration {
        server::Clock::minimum_wait(self, now, until)
    }

    fn finished_cycle(
        self: &mut Self,
        now: time::Instant,
        in_game: units::Time
    ) {
        server::Clock::finished_cycle(self, now, in_game)
    }

    fn end_cycles(self: &mut Self) {
        server::Clock::end_cycles(self);
    }
}

impl server::Clock<units::Time> for Clock {
    fn in_game(self: &mut Self, now: time::Instant) -> units::Time {
        self.0.time(now)
    }
    fn minimum_wait(
        self: &mut Self,
        now: units::Time,
        until: units::Time,
    ) -> time::Duration {
        duration_real_time(until - now)
    }
    fn finished_cycle(
        self: &mut Self,
        _now: time::Instant,
        _in_game: units::Time
    ) {}
    fn end_cycles(self: &mut Self) {}
}

type Server = server::Server<Clock, Interruption, units::Time>;

fn create_server_local<F, R>(
    f: F,
    upd: mpsc::Receiver<Interruption>,
) -> (Server, Clock, R)
    where F: FnOnce(
                 &mut sulphate::EntityHeap,
                 &mut sulphate::EventQueue,
             ) -> R,
          R: Send + 'static,
{
    let initial_time = Default::default();
    let mut clock = Clock(Simple::new(initial_time));
    clock.0.start(time::Instant::now());

    let mut space = sulphate::EntityHeap::new();
    let mut time = sulphate::EventQueue::new(initial_time);

    let r = f(&mut space, &mut time);

    let server = Server::new(space, time, upd, clock.clone());

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
                 &mut sulphate::EntityHeap,
                 &mut sulphate::EventQueue,
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
