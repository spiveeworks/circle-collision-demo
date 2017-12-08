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
        let before = self.image::<T>(matter, id);
        let _phantom = marker::PhantomData;

        Entry { id, before, space: self, time, matter, _phantom }
    }

    pub fn image<T>(
        self: &Self,
        matter: &sulphate::EntityHeap,
        instance: sulphate::EntityId,
    ) -> Option<Image>
        where T: any::Any + entities::Display
    {
        let maybe_body = self.get::<T>(instance);

        maybe_body.map(|c_body| c_body.body.clone())
                  .and_then(|body| {
            matter.get::<T>(instance)
                  .expect("Nonexistent entity has body")  // really?
                  .image()
                  .map(|inner_image| Image { inner_image, body })
        })
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

    /*
    pub fn body_mut (self: &mut Self) -> Option<&mut space::Body> {
        let instance = self.id;
        let ty = any::TypeId::of::<T>();
        let uid = sulphate::EntityUId { instance, ty };
        self.space
            .contents
            .get_mut(&uid)
            .map(|c_body| &mut c_body.body)
    }
    */

    pub fn body(self: &Self) -> Option<&space::Body> {
        self.space
            .get::<T>(self.id)
            .map(|c_body| &c_body.body)
    }

    pub fn body_mut(self: &mut Self) -> Option<&mut space::Body> {
        self.space
            .get_mut::<T>(self.id)
            .map(|c_body| &mut c_body.body)
    }

    pub fn set_body(self: &mut Self, new_body: space::Body) {
        if let Some(body) = self.body_mut() {
            *body = new_body;
            return;
        }

        // else
        let id = self.id;
        let ty = any::TypeId::of::<T>();
        let uid = sulphate::EntityUId { id, ty };
        self.space.new_body(uid, new_body);
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
        let val_after = self.space.image::<T>(&self.matter, self.id);
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

