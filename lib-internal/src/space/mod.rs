use std::any;
use std::collections;

use sulphate;

mod body;
mod eyes;

pub use self::body::Body;
pub use self::eyes::Entry;
pub use self::eyes::Eyes;
pub use self::eyes::Image;

// TODO switch to macro-generated enums please?
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct EntityUId {
    instance: sulphate::EntityId,
    ty: any::TypeId,
}

// a space is a collection of entities with some kind of location-allocation.
// it is the medium through which entities can communicate psedunymously
pub struct CollisionSpace {
    contents: collections::HashMap<EntityUId, body::CollisionBody>
}

impl CollisionSpace {
    pub fn new() -> Self {
        let contents = collections::HashMap::new();
        CollisionSpace { contents }
    }
}

