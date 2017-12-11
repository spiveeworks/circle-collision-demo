use std::any;

use sulphate;
use units;

pub use self::player::Player;

pub mod player;

#[derive(PartialEq, Clone)]
pub enum Image {
    Player(player::Image),
}

impl Image {
    pub fn radius(self: &Self) -> units::Distance {
        match *self {
            Image::Player(_) => 10.into(),
        }
    }
}

pub trait Display {
    fn image(self: &Self) -> Option<Image>;
}

pub fn image_of(
    matter: &sulphate::EntityHeap,
    uid: sulphate::EntityUId,
) -> Option<Image> {
    if uid.ty == any::TypeId::of::<Player>() {
        matter.get::<Player>(uid.id).and_then(Display::image)
    } else {
        panic!("Tried to get image of unknown entity");
    }
}


