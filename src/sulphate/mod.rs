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

        Game { time, matter }
    }
}

pub enum GeneralEvent {
}

impl event_queue::GeneralEvent<PrivateGame> for GeneralEvent {
    fn invoke(self: Self, _game: &mut PrivateGame) {
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
