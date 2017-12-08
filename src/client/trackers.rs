use std::option;

use city_internal::space;
use city_internal::sulphate;

struct Tracker {
    id: sulphate::EntityId,
    image: Option<space::Image>,
}

pub struct Perception {
    player: Tracker
}


impl Perception {
    pub fn apply_update(
        self: &mut Self,
        before: Option<space::Image>,
        after: Option<space::Image>,
    ) {
        if self.player.image == before {
            self.player.image = after;
        } else {
            panic!("Cannot handle more than one entity!");
        }
    }

    pub fn new(id: sulphate::EntityId) -> Self {
        let image = None;
        let player = Tracker { id, image };
        Perception { player }
    }

    pub fn player_id(self: &Self) -> sulphate::EntityId {
        self.player.id
    }
}


impl<'a> IntoIterator for &'a Perception {
    type Item = &'a space::Image;
    type IntoIter = option::Iter<'a, space::Image>;
    fn into_iter(self: &'a Perception) -> Self::IntoIter {
        self.player.image.iter()
    }
}
