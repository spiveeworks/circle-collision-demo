extern crate sulphate_lib;
extern crate lib_math;

pub mod physics;
pub mod player;

pub mod sulphate {
    use sulphate_lib::entity_heap;
    use sulphate_lib::event_queue;

    use physics;

    pub type EventQueue = event_queue::EventQueue<physics::Time>;
    pub type EntityHeap = entity_heap::EntityHeap;
    pub type EntityId = entity_heap::UID;

    pub mod server;
}
