use std::any;
use std::marker;

use sulphate;
use physics::units;

pub mod player;

#[derive(PartialEq, Clone)]
pub enum Image {
    Player(player::Image),
    Nothing,
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
         .unwrap_or(Image::Nothing)
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
        let before = Image::Nothing;
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

    pub fn reborrow<U>(
        self: &mut Self,
        id: sulphate::EntityId,
    ) -> TrackImage<U>
        where U: any::Any + Display
    {
        TrackImage::track_image(&mut self.space, &mut self.time, id)
    }

    pub fn remove(self: &mut Self) -> Option<T> {
        self.space.remove(self.id)
    }
}

fn has_eyes(thing: &any::Any) -> bool {
    any::Any::get_type_id(thing) == any::TypeId::of::<player::Player>()
}

impl<'a, T> Drop for TrackImage<'a, T>
    where T: any::Any + Display
{
    fn drop(self: &mut Self) {
        let id = self.id;
        let before = self.before.clone();
        let after = get_image::<T>(self.space, self.id);
        if before != after {
            let player_ids: Vec<sulphate::EntityId> =
                self.space
                    .iter()
                    .flat_map(|(ent_id, ent)|
                        if has_eyes(ent) {
                            Some(ent_id)
                        } else {
                            None
                        }
                   ).collect();
            for player_id in player_ids {
                let player = self.reborrow(player_id);
                player::Player::update(player, id, before.clone(), after.clone());
            }
        }
    }
}
