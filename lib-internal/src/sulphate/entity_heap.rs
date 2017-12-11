use std::any;
use std::any::Any;
use std::collections;

use rand;

pub type ID = u64;

// TODO switch to macro-generated enums please?
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UID {
    pub id: ID,
    pub ty: any::TypeId,
}


// heap in the memory sense not the queue sense
pub struct EntityHeap {
    content: collections::HashMap<UID, Box<Any>>,
    key_seed: rand::XorShiftRng,
}

static DOWNCAST_ERROR: &'static str = "\
Value stored under incorrect type information. \
";

fn unwrap_box<T: Any>(box_val: Box<Any>) -> T {
    *box_val.downcast()
            .ok()
            .expect(DOWNCAST_ERROR)
}

fn unwrap_box_ref<T: Any>(box_ref: &Box<Any>) -> &T {
    box_ref.downcast_ref()
           .expect(DOWNCAST_ERROR)
}

fn unwrap_box_mut<T: Any>(box_mut: &mut Box<Any>) -> &mut T {
    box_mut.downcast_mut()
           .expect(DOWNCAST_ERROR)
}

impl EntityHeap {
    pub fn new() -> EntityHeap {
        let content = collections::HashMap::new();
        let key_seed = rand::weak_rng();
        EntityHeap { content, key_seed }
    }

    pub fn get<T: Any>(self: &Self, id: ID) -> Option<&T> {
        let ty = any::TypeId::of::<T>();
        let uid = UID { id, ty };
        self.content
            .get(&uid)
            .map(unwrap_box_ref)
    }

    pub fn get_mut<T: Any>(self: &mut Self, id: ID) -> Option<&mut T> {
        let ty = any::TypeId::of::<T>();
        let uid = UID { id, ty };
        self.content
            .get_mut(&uid)
            .map(unwrap_box_mut)
    }

    fn new_id(self: &mut Self, ty: any::TypeId) -> ID {
        use rand::Rng;
        loop {
            let id = self.key_seed.next_u64();
            let uid = UID { id, ty };
            if !self.content.contains_key(&uid) {
                return id;
            }
        }
    }

    pub fn add<T: Any>(self: &mut Self, v: T) -> ID {
        let ty = any::TypeId::of::<T>();
        let val = Box::new(v);
        let id = self.new_id(ty);
        let uid = UID { id, ty };
        let overflow = self.content
                           .insert(uid, val);
        // this is fine, it will just drop the value,
        // but when debugging I'd want to know what happened
        debug_assert!(overflow.is_none(), "reused key");
        id
    }

    pub fn remove<T: Any>(self: &mut Self, id: ID) -> Option<T> {
        let ty = any::TypeId::of::<T>();
        let uid = UID { id, ty };
        self.content
            .remove(&uid)
            .map(unwrap_box)
    }
}


