use std::any;
use std::marker;

use sulphate;
use physics::units;

pub mod player;

#[derive(PartialEq)]
pub enum Image {
    Player(player::Image),
    None,
}


pub trait Display {
    fn image(self: &Self) -> Image;
}


pub struct TrackImage<'a, T>
    where T: 'a + any::Any + Display
{
    id: sulphate::EntityId,
    before: Image,
    space: &'a mut sulphate::EntityHeap,
    time: &'a mut sulphate::EventQueue,
    // what on earth should this be?
    _phantom: marker::PhantomData<&'a mut T>,
}

fn get_image<T>(space: &sulphate::EntityHeap, id: sulphate::EntityId) -> Image
    where T: any::Any + Display
{
    space.get(id)
         .map(<T as Display>::image)
         .unwrap_or(Image::None)
}

impl<'a, T> TrackImage<'a, T>
    where T: any::Any + Display
{
    pub fn track_image(
        space: &'a mut sulphate::EntityHeap,
        time: &'a mut sulphate::EventQueue,
        id: sulphate::EntityId,
    ) -> Self {
        let before = get_image::<T>(space, id);
        let _phantom = marker::PhantomData;

        TrackImage { id, before, space, time, _phantom }
    }

    pub fn track_new(
        space: &'a mut sulphate::EntityHeap,
        time: &'a mut sulphate::EventQueue,
        value: T,
    ) -> Self {
        let before = Image::None;
        let id = space.add(value);
        let _phantom = marker::PhantomData;

        TrackImage { id, before, space, time, _phantom }
    }

    pub fn id(self: &Self) -> sulphate::EntityId {
        self.id
    }

    pub fn now(self: &Self) -> units::Time {
        self.time.now()
    }

    pub fn get(self: &Self) -> Option<&T> {
        self.space.get(self.id)
    }

    pub fn get_mut(self: &mut Self) -> Option<&mut T> {
        self.space.get_mut(self.id)
    }

    pub fn remove(self: &mut Self) -> Option<T> {
        self.space.remove(self.id)
    }
}

impl<'a, T> Drop for TrackImage<'a, T>
    where T: any::Any + Display
{
    fn drop(self: &mut Self) {
        let after = get_image::<T>(self.space, self.id);
        if self.before != after {
            // TODO what?!
            unimplemented!()
        }
    }
}
