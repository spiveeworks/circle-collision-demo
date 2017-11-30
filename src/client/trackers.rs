use std::iter;

use city_internal::sulphate;
use city_internal::entities;

struct Tracker {
    id: sulphate::EntityId,
    image: entities::Image,
}

pub struct Perception {
    player: Tracker
}


impl Perception {
    pub fn apply_update(
        self: &mut Self,
        id: sulphate::EntityId,
        before: entities::Image,
        after: entities::Image
    ) {
        if self.player.id == id {
            debug_assert!(
                self.player.image == before,
                "Inconsistent game world"
            );
            self.player.image = after;
        }
    }

    pub fn new(id: sulphate::EntityId) -> Self {
        let image = entities::Image::Nothing;
        let player = Tracker { id, image };
        Perception { player }
    }

    pub fn player_id(self: &Self) -> sulphate::EntityId {
        self.player.id
    }
}


impl<'a> IntoIterator for &'a Perception {
    type Item = &'a entities::Image;
    type IntoIter = iter::Once<&'a entities::Image>;
    fn into_iter(self: &'a Perception) -> Self::IntoIter {
        iter::once(&self.player.image)
    }
}
