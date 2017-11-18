use std::sync::mpsc;

use sulphate;
use physics;

pub struct Player {
    body: physics::Body,
    // stimulus from the game world
    _update: mpsc::Sender<Update>,
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
        location: physics::Position,
    }
}

pub trait External {
    fn update(
        space: &mut sulphate::EntityHeap,
        time: &mut sulphate::EventQueue,
        id: sulphate::EntityId,
        data: Control,
    ) {
        use self::Control::*;
        match data {
            Move { velocity } => {
                if let Some(ent) = space.get_mut(id) {
                    let player: &mut Player = ent;
                    player.body.bounce(velocity, time.now());
                }
            }
        }
    }
}

impl Player {
    pub fn new(
        space: &mut sulphate::EntityHeap,
        _time: &mut sulphate::EventQueue,
        position: physics::Position,
        _update: mpsc::Sender<Update>,
    ) -> sulphate::EntityId {
        let body = physics::Body::new_frozen(position);
        let player = Player { body, _update };
        // TODO implement fn add(self: &mut Self, val: T) -> EntityId
        space.add(player)
    }
}
