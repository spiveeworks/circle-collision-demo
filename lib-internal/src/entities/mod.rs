use std::Any;

pub mod player;

#[derive(PartialEq)]
pub struct Image {
    Player(player::Image),
    None,
}


pub trait Display {
    fn image(self: &Self) -> Image;
}


pub struct TrackImage<'a, T> {
    id: sulphate::EntityId,
    before: Image,
    space: &'a mut sulphate::EntityHeap,
    time: &'a mut sulphate::EventQueue,
}

pub fn track_image<T>(
    id: sulphate::EntityId,
    space: &'a mut sulphate::EntityHeap,
    time: &'a mut sulphate::EventQueue
) -> Option<TrackImage<T>>
    where T: any::Any + Display
{
    let ent: &T = space.get(id);
    let before = ent.image();

    TrackImage { id, before, space, time }
}

impl<'a, T> Drop for TrackImage<'a, T>
    where T: Any + Display
{
    let ent: &T = self.space.get(self.id);
    let after = ent.image();
    if before != after {
        // TODO what?!
    }
}
