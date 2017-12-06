use std::any;

use entities;
use sulphate;

mod body;
mod eyes;

pub use self::body::Body;
pub use self::eyes::Entry;
pub use self::eyes::Eyes;
pub use self::eyes::Image;

// a space is a collection of entities with some kind of location-allocation.
// it is the medium through which entities can communicate psedunymously
pub struct CollisionSpace {
    contents: Vec<(sulphate::EntityUId, body::CollisionBody)>
}

impl CollisionSpace {
    pub fn new() -> Self {
        let contents = Vec::new();
        CollisionSpace { contents }
    }

    fn find<T>(self: &Self, id: sulphate::EntityId) -> Option<usize>
        where T: any::Any + entities::Display
    {
        let ty = any::TypeId::of::<T>();
        let uid = sulphate::EntityUId { id, ty };
        for (n, &(ent_uid, _)) in self.contents.iter().enumerate() {
            if uid == ent_uid {
                return Some(n);
            }
        }
        None
    }


    fn get<T>(self: &Self, id: sulphate::EntityId) -> Option<&body::CollisionBody>
        where T: any::Any + entities::Display
    {
        self.find::<T>(id)
            .map(|n| &self.contents[n].1)
    }

    fn get_mut<T>(
        self: &mut Self,
        id: sulphate::EntityId,
    ) -> Option<&mut body::CollisionBody>
        where T: any::Any + entities::Display
    {
        self.find::<T>(id)
            .map(move |n| &mut self.contents[n].1)  // moves self - a reference
    }
}

