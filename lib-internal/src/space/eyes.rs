use std::any;
use std::marker;

use space;

use entities;
use sulphate;
use units;

impl space::CollisionSpace {
    pub fn entry<'a, T>(
        self: &'a mut Self,
        time: &'a mut sulphate::EventQueue,
        matter: &'a mut sulphate::EntityHeap,
        id: sulphate::EntityId,
    ) -> Entry<'a, T>
        where T: any::Any + entities::Display
    {
        let space = self;
        let before = None;
        let body = space.get::<T>(id).map(|c_body| c_body.body.clone());
        let _phantom = marker::PhantomData;

        let mut result =
            Entry { id, before, body, space, time, matter, _phantom };
        result.before = result.image();

        result
    }
}

pub struct Entry<'a, T>
    where T: any::Any + entities::Display
{
    id: sulphate::EntityId,
    space: &'a mut space::CollisionSpace,
    time: &'a mut sulphate::EventQueue,
    matter: &'a mut sulphate::EntityHeap,
    before: Option<Image>,
    pub body: Option<space::Body>,
    // T should be a parameter of the EntityId
    _phantom: marker::PhantomData<&'a mut T>,
}

impl<'a, T> Entry<'a, T>
    where T: any::Any + entities::Display
{
    pub fn get_mut(self: &mut Self) -> Option<&mut T> {
        self.matter.get_mut(self.id)
    }

    pub fn get(self: &Self) -> Option<&T> {
        self.matter.get(self.id)
    }

    pub fn now(self: &Self) -> units::Time {
        self.time.now()
    }

    pub fn image(self: &Self) -> Option<Image> {
        self.body
            .clone()
            .and_then(|body| {
                self.matter.get::<T>(self.id)
                    .and_then(entities::Display::image)
                    .map(|inner_image| Image { inner_image, body })
            })
    }
}

#[derive(Clone, PartialEq)]
pub struct Image {
    pub inner_image: entities::Image,
    pub body: space::Body,
}

impl<'a, T> Drop for Entry<'a, T>
    where T: any::Any + entities::Display
{
    fn drop(self: &mut Self) {
        let before = self.before.as_ref();
        let val_after = self.image();
        let after = val_after.as_ref();

        let ty = any::TypeId::of::<T>();
        let id = self.id;
        let uid = sulphate::EntityUId { ty, id };
        super::body::update_physics(
            &mut self.space,
            &mut self.time,
            uid,
            after,
        );

        if before != after {
            let vision_ids: Vec<EyesId> =
                self.space
                    .contents
                    .iter()
                    .map(|&(uid, _)| uid)
                    .flat_map(as_eyes)
                    .collect();
            for vision_id in vision_ids {
                update(
                    vision_id,
                    &mut self.space,
                    &mut self.time,
                    &mut self.matter,
                    before,
                    after,
                );
            }
        }
    }
}

enum EyesId {
    Player(sulphate::EntityId),
}

fn as_eyes(uid: sulphate::EntityUId) -> Option<EyesId> {
    if uid.ty == any::TypeId::of::<entities::Player>() {
        Some(EyesId::Player(uid.id))
    } else {
        None
    }
}

fn update(
    id: EyesId,
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    matter: &mut sulphate::EntityHeap,
    before: Option<&Image>,
    after: Option<&Image>,
) {
    match id {
        EyesId::Player(id) => {
            let ent = space::CollisionSpace::entry(space, time, matter, id);
            <entities::Player as Eyes>::update(ent, before, after);
        }
    }
}

pub trait Eyes: any::Any + entities::Display where Self: Sized {
    fn update(
        this: Entry<Self>,
        before: Option<&Image>,
        after: Option<&Image>
    );
}

