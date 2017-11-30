use std::sync::mpsc;

use entities;
use sulphate;
use physics;

pub struct Player {
    body: physics::Body,
    // stimulus from the game world
    update: mpsc::Sender<Update>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Image {
    pub body: physics::Body,
}

impl entities::Display for Player {
    fn image(self: &Self) -> entities::Image {
        let body = self.body.clone();
        let img = Image { body };
        entities::Image::Player(img)
    }
}

pub enum Control {
    Move {
        velocity: physics::Velocity
    }
}

pub struct Update {
    pub when: physics::Time,
    pub what: UpdateData,
}

pub enum UpdateData {
    Created {
        id: sulphate::EntityId,
        position: physics::Position,
    },
    Update {
        id: sulphate::EntityId,
        before: entities::Image,
        after: entities::Image,
    },
}

impl Control {
    pub fn apply(
        space: &mut sulphate::EntityHeap,
        time: &mut sulphate::EventQueue,
        id: sulphate::EntityId,
        data: Control,
    ) {
        use self::Control::*;
        match data {
            Move { velocity } => {
                let now = time.now();
                let mut this =
                    entities::TrackImage::track_image(space, time, id);
                if let Some(ent) = this.get_mut() {
                    let player: &mut Player = ent;
                    player.body.bounce(velocity, now);
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
        space: &'a mut sulphate::EntityHeap,
        time: &'a mut sulphate::EventQueue,
        position: physics::Position,
        update: mpsc::Sender<Update>,
    ) -> entities::TrackImage<'a, Player> {
        let body = physics::Body::new_frozen(position);
        let player = Player { body, update };
        let this = entities::TrackImage::track_new(space, time, player);

        let when = this.now();
        let id = this.id();
        let what = UpdateData::Created { id, position };
        let update = Update { when, what };
        this.get().unwrap().send(update);

        this
    }

    pub fn update(
        this: entities::TrackImage<Player>,
        id: sulphate::EntityId,
        before: entities::Image,
        after: entities::Image,
    ) {
        if let Some(player) = this.get() {
            let when = this.now();
            let what = UpdateData::Update { id, before, after };
            let update = Update { when, what };
            player.send(update);
        }
    }
}

