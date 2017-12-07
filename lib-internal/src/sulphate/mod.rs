use sulphate_lib::event_queue;

use space;
use units;

pub use sulphate::entity_heap::EntityHeap;

//pub mod server;

mod entity_heap;

pub type EventQueue = event_queue::EventQueue<units::Time, World>;
pub type EntityId = entity_heap::ID;
pub type EntityUId = entity_heap::UID;

pub struct Game {
    time: EventQueue,
    world: World,
}

pub struct World {
    space: space::CollisionSpace,
    matter: EntityHeap,
}

impl Game {
    pub fn new(initial_time: units::Time) -> Game {
        let space = space::CollisionSpace::new(initial_time);
        let time = EventQueue::new(initial_time);
        let matter = EntityHeap::new();

        let world = World { space, matter };
        Game { time, world }
    }

    pub fn enqueue_absolute<E>(
        self: &mut Self,
        event: E,
        execute_time: units::Time,
    ) where E: Event {
        self.time.enqueue_absolute(AdaptEvent(event), execute_time);
    }

    pub fn enqueue_relative<E>(
        self: &mut Self,
        event: E,
        execute_delay: units::Duration,
    ) where E: Event {
        self.time.enqueue_relative(AdaptEvent(event), execute_delay);
    }
}

struct AdaptEvent<E>(E) where E: Event;

// TODO hide EventQueue::invoke_next() methods from Event implementors
pub trait Event: 'static {
    fn invoke(
        self: Self,
        space: &mut space::CollisionSpace,
        time: &mut EventQueue,
        matter: &mut EntityHeap,
    );
}

impl<E> event_queue::Event<units::Time, World> for AdaptEvent<E>
    where E: Event
{
    fn invoke(
        self: Self,
        time: &mut EventQueue,
        world: &mut World,
    ) {
        self.0.invoke(&mut world.space, time, &mut world.matter);
    }
}

