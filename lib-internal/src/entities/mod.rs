pub mod player;

pub use self::player::Player;

#[derive(PartialEq, Clone)]
pub enum Image {
    Player(player::Image),
}


pub trait Display {
    fn image(self: &Self) -> Option<Image>;
}



