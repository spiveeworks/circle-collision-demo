#![feature(get_type_id)]

extern crate sulphate_lib;
extern crate lib_math;

//pub mod entities;
pub mod space;
// module to hide `lib_math`
pub mod units;

// module to hide `sulphate_lib`
pub mod sulphate {
    use sulphate_lib::entity_heap;
    use sulphate_lib::event_queue;

    use units;

    pub type EventQueue = event_queue::EventQueue<units::Time>;
    pub type EntityHeap = entity_heap::EntityHeap;
    pub type EntityId = entity_heap::UID;

    //pub mod server;
}
