use sulphate_lib::event_queue;

use space;
use units;

pub use sulphate::entity_heap::EntityHeap;

pub mod server;

mod entity_heap;

pub type EventQueue = event_queue::EventQueue<units::Time, World>;
pub type EntityId = entity_heap::ID;
pub type EntityUId = entity_heap::UID;

pub struct World {
    space: space::CollisionSpace,
    matter: EntityHeap,
}

pub fn enqueue_absolute<E>(
    time: &mut EventQueue,
    event: E,
    execute_time: units::Time,
) where E: Event {
    time.enqueue_absolute(AdaptEvent(event), execute_time);
}

pub fn enqueue_relative<E>(
    time: &mut EventQueue,
    event: E,
    execute_delay: units::Duration,
) where E: Event {
    time.enqueue_relative(AdaptEvent(event), execute_delay);
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

