use std::any;

use entities;
use sulphate;

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
    in_contact: Vec<(sulphate::EntityUId, sulphate::EntityUId)>,
}

impl CollisionSpace {
    pub fn new() -> Self {
        let contents = Vec::new();
        let in_contact = Vec::new();
        CollisionSpace { contents, in_contact }
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

    fn get_uid_image(
        self: &Self,
        matter: &sulphate::EntityHeap,
        uid: sulphate::EntityUId,
    ) -> Option<Image> {
        let body = self.get_uid(uid).map(|c_body| c_body.body.clone());
        let image = entities::image_of(matter, uid);
        body.and_then(|body|
            image.map(|inner_image|
                Image { inner_image, body }
            )
        )
    }

/*
    fn get_mut<T>(
        self: &mut Self,
        id: sulphate::EntityId,
    ) -> Option<&mut body::CollisionBody>
        where T: any::Any + entities::Display
    {
        self.find::<T>(id)
            .map(move |n| &mut self.contents[n].1)  // moves self - a reference
    }
*/

    fn are_in_contact(
        self: &Self,
        first: sulphate::EntityUId,
        second: sulphate::EntityUId,
    ) -> bool {
        for &(x, y) in &self.in_contact {
            if first == x && second == y
            || first == y && second == x {
                return true;
            }
        }
        false
    }

    fn release_contact(
        self: &mut Self,
        first: sulphate::EntityUId,
        second: sulphate::EntityUId,
    ) {
        self.in_contact.retain(|&(x, y)|
            !(first == x && second == y ||
                  first == y && second == x)
        );
    }

    fn get_contacts(
        self: &Self,
        uid: sulphate::EntityUId,
    ) -> Vec<sulphate::EntityUId> {
        self.in_contact
            .iter()
            .flat_map(|&(first, second)| {
                if uid == first {
                    Some(second)
                } else if uid == second {
                    Some(first)
                } else {
                    None
                }
            })
            .collect()
    }
}

