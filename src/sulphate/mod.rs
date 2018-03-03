use animation;
use maths::units;
use sulphate_lib::event_queue;

type EventQueue = event_queue::EventQueue<GeneralEvent, units::Time>;

pub struct Game {
    time: EventQueue,
    pub matter: animation::Matter,
}

impl Game {
    pub fn new(matter: animation::Matter, now: units::Time) -> Self {
        let time = EventQueue::new(now);

        let mut game = Game { time, matter };

        animation::Matter::start(&mut game);

        game
    }

    pub fn now(self: &Self) -> units::Time {
        self.time.now()
    }

    pub fn enqueue_event_relative<E>(
        self: &mut Self,
        event: E,
        duration: units::Duration,
    )
        where E: Into<GeneralEvent>,
    {
        self.time.enqueue_relative(event, duration);
    }

    pub fn enqueue_event_absolute<E>(
        self: &mut Self,
        event: E,
        time: units::Time,
    )
        where E: Into<GeneralEvent>,
    {
        self.time.enqueue_absolute(event, time);
    }
}

pub trait Event {
    fn invoke(self: Self, game: &mut Game);
}

pub enum GeneralEvent {
    Matter(animation::MatterEvent),
}

impl From<animation::MatterEvent> for GeneralEvent {
    fn from(event: animation::MatterEvent) -> GeneralEvent {
        GeneralEvent::Matter(event)
    }
}

impl event_queue::GeneralEvent<PrivateGame> for GeneralEvent {
    fn invoke(self: Self, game: &mut PrivateGame) {
        let game = &mut game.inner;

        use self::GeneralEvent::*;
        match self {
            Matter(event) => event.invoke(game),
        }
    }
}

// this struct is never revealed to any entities, so as to hide the simulate()
// method from them
pub struct PrivateGame {
    pub inner: Game,
}

impl AsMut<EventQueue> for PrivateGame {
    fn as_mut(self: &mut Self) -> &mut EventQueue {
        &mut self.inner.time
    }
}
