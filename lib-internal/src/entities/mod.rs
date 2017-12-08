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



