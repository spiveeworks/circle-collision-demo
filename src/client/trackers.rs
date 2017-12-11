use std::slice;

use city_internal::space;
use city_internal::sulphate;

pub struct Perception {
    player: sulphate::EntityId,
    others: Vec<space::Image>,
}

impl Perception {
    pub fn apply_update(
        self: &mut Self,
        before: Option<space::Image>,
        after: Option<space::Image>,
    ) {
        if let Some(before) = before {
            let mut it = None;
            for (i, each) in self.others.iter().enumerate() {
                if *each == before {
                    it = Some(i);
                    break;
                }
            }
            if it.is_none() {
                println!("Unknown entity updated");
                return;
            }
            let it = it.unwrap();
            if let Some(after) = after {
                self.others[it] = after;
            } else {
                self.others.remove(it);
            }
        } else {
            if let Some(after) = after {
                self.others.push(after);
            }
        }
    }

    pub fn new(player: sulphate::EntityId) -> Self {
        let others = Vec::new();
        Perception { player, others }
    }

    pub fn player_id(self: &Self) -> sulphate::EntityId {
        self.player
    }
}


impl<'a> IntoIterator for &'a Perception {
    type Item = &'a space::Image;
    type IntoIter = slice::Iter<'a, space::Image>;
    fn into_iter(self: &'a Perception) -> Self::IntoIter {
        self.others.iter()
    }
}
