use std::sync::mpsc;

use entities;
use space;
use sulphate;
use units;

pub struct Player {
    // stimulus from the game world
    update: mpsc::Sender<Update>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Image;

impl entities::Display for Player {
    fn image(self: &Self) -> Option<entities::Image> {
        let img = Image;
        Some(entities::Image::Player(img))
    }
}

pub enum Control {
    Move {
        velocity: units::Velocity
    }
}

pub struct Update {
    pub when: units::Time,
    pub what: UpdateData,
}

pub enum UpdateData {
    Created {
        id: sulphate::EntityId,
        position: units::Position,
    },
    Vision {
        before: Option<space::Image>,
        after: Option<space::Image>,
    },
}

impl Control {
    pub fn apply(
        space: &mut space::CollisionSpace,
        time: &mut sulphate::EventQueue,
        matter: &mut sulphate::EntityHeap,
        id: sulphate::EntityId,
        data: Control,
    ) {
        use self::Control::*;
        match data {
            Move { velocity } => {
                let mut this: space::Entry<Player> =
                    space.entry(time, matter, id);
                let now = this.now();
                if let Some(body) = this.body.as_mut() {
                    body.bounce(velocity, now);
                } else {
                    println!("Player has no location!");
                }
            }
        }
    }
}

impl Player {
    fn send(
        self: &Self,
        update: Update,
    ) {
        if let Err(_) = self.update.send(update) {
            println!("Player failed to send update to device");
        }
    }

    pub fn new<'a>(
        space: &'a mut space::CollisionSpace,
        time: &'a mut sulphate::EventQueue,
        matter: &'a mut sulphate::EntityHeap,
        position: units::Position,
        update: mpsc::Sender<Update>,
    ) -> space::Entry<'a, Player> {
        let player = Player { update };
        let id = matter.add(player);
        let mut this = space.entry::<Player>(time, matter, id);

        this.body = Some(space::Body::new_frozen(position));

        let when = this.now();
        let what = UpdateData::Created { id, position };
        let update = Update { when, what };
        this.get().unwrap().send(update);

        this
    }
}

impl space::Eyes for Player {
    fn update(
        this: space::Entry<Player>,
        before_ref: Option<&space::Image>,
        after_ref: Option<&space::Image>,
    ) {
        if let Some(player) = this.get() {
            let before = before_ref.cloned();
            let after = after_ref.cloned();
            let when = this.now();
            let what = UpdateData::Vision { before, after };
            let update = Update { when, what };
            player.send(update);
        }
    }
}

impl space::Collide for Player {
    fn collide(
        mut this: space::Entry<Player>,
        _other: space::Image,
    ) {
        let now = this.now();

        let body = this.body.as_mut().expect("Collided without a body");
        let velocity = body.velocity();
        let position = body.position(now);
        if velocity != Default::default() {
            let new_position = position - (velocity / velocity.magnitude());
            *body = space::Body::new_frozen(new_position);
        }
    }
}

