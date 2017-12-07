use std::any;

use entities;
use sulphate;
use units;

mod body;
mod eyes;

pub use self::body::Collide;
pub use self::body::Body;
pub use self::eyes::Entry;
pub use self::eyes::Eyes;
pub use self::eyes::Image;

// a space is a collection of entities with some kind of location-allocation.
// it is the medium through which entities can communicate psedunymously
pub struct CollisionSpace {
    contents: Vec<(sulphate::EntityUId, body::CollisionBody)>,
    last_collision_time: units::Time,
    collisions: Vec<(sulphate::EntityUId, sulphate::EntityUId)>,
}

impl CollisionSpace {
    pub fn new(initial_time: units::Time) -> Self {
        let contents = Vec::new();
        let last_collision_time = initial_time;
        let collisions = Vec::new();
        CollisionSpace { contents, last_collision_time, collisions }
    }

    fn find<T>(self: &Self, id: sulphate::EntityId) -> Option<usize>
        where T: any::Any + entities::Display
    {
        let ty = any::TypeId::of::<T>();
        let uid = sulphate::EntityUId { id, ty };
        self.find_uid(uid)
    }

    fn find_uid(self: &Self, uid: sulphate::EntityUId) -> Option<usize> {
        for (n, &(ent_uid, _)) in self.contents.iter().enumerate() {
            if uid == ent_uid {
                return Some(n);
            }
        }
        None
    }

    fn get<T>(
        self: &Self,
        id: sulphate::EntityId,
    ) -> Option<&body::CollisionBody>
        where T: any::Any + entities::Display
    {
        self.find::<T>(id)
            .map(|n| &self.contents[n].1)
    }

    fn get_uid(
        self: &Self,
        uid: sulphate::EntityUId,
    ) -> Option<&body::CollisionBody> {
        self.find_uid(uid)
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

    // will prevent things from colliding more than once per instant
    // at the moment this works even if they collide in different ways each
    // time
    fn has_collided(
        self: &Self,
        now: units::Time,
        first: sulphate::EntityUId,
        second: sulphate::EntityUId,
    ) -> bool {
        if now != self.last_collision_time {
            return false;
        }
        for &(x, y) in &self.collisions {
            if first == x && second == y
            || first == y && second == x {
                return true;
            }
        }
        false
    }

    fn note_collided(
        self: &mut Self,
        now: units::Time,
        first: sulphate::EntityUId,
        second: sulphate::EntityUId,
    ) {
        if now != self.last_collision_time {
            self.collisions.clear();
            self.last_collision_time = now;
        }
        self.collisions.push((first, second));
    }
}

