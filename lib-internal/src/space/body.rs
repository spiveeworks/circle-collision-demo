use units;

use super::CollisionSpace;

impl CollisionSpace {
    pub(super) fn update_physics<T>(self: &mut Self) {
        unimplemented!();
    }
}

pub struct CollisionBody {
    pub body: Body,
    speed: units::Speed,
    march_time: units::Time,
    radius: units::Distance,
}

impl CollisionBody {
    pub fn set_velocity(self: &mut Self, vel: units::Velocity) {
        unimplemented!();
    }

    pub fn set_position(self: &mut Self, pos: units::Position) {
        unimplemented!();
    }
}

enum MarchResult {
    /// Objects far from eachother and cannot collide with eachother until
    /// at least this time
    March(units::Time),
    /// Objects close to eachother and will actually collide at this time
    Collide(units::Time),
    /// Objects close to eachother but will not collide
    Miss,
}

// this is the edge-to-edge distance at which ray-marching and precise hit-scan
// are considered equally preferable
// this is a variable for optimization,
// but its optimal value will change between builds,
// hypothetically you could select this value during an automated release
// process
fn march_threshold() -> units::Distance {
    5.into()
}

fn march(
    one: &CollisionBody,
    other: &CollisionBody,
    time: units::Time
) -> MarchResult {
    let one_pos = one.body.position(time);
    let other_pos = other.body.position(time);
    let centre_dist_squared = (one_pos - other_pos).squared();

    // the maximum distance for testing collision
    let proximity = one.radius + other.radius + march_threshold();

    // if they are close enough, check for collision properly
    if centre_dist_squared < proximity.squared() {
        if let Some(collide_time) = collide_time(one, other, time) {
            MarchResult::Collide(collide_time)
        } else {
            MarchResult::Miss
        }
    } else {
        // otherwise march for a while
        let centre_dist = units::Scalar::rough_sqrt(centre_dist_squared);
        let edge_dist = centre_dist - proximity;
        let march_time = edge_dist / (one.speed + other.speed);
        MarchResult::March(time + march_time)
    }
}

// gives the collision time,
// gives None if they are pointed away from eachother,
//     or they will miss eachother
//     or they are already inside eachother
fn collide_time(
    one: &CollisionBody,
    other: &CollisionBody,
    time: units::Time
) -> Option<units::Time> {
    // we will work with a relative reference frame
    let rel_pos = one.body.position(time) - other.body.position(time);
    let rel_vel = one.body.velocity() - other.body.velocity();

    // they miss if their velocity and displacement are in the same direction
    // A -p-> B -v->
    let inner = units::Vector::inner(rel_pos, rel_vel);
    if inner < 0 {
        // this is the time at which the bodies will be closest
        // it follows from assuming (p + vt) is orthogonal to v
        let near_time: units::Duration = - inner / rel_vel.squared();

        // they collide when there's no room between their circumferences
        let coll_dist: units::Distance = one.radius + other.radius;

        // they also miss if they never get close enough (by definition)
        let near: units::Displacement = rel_pos + rel_vel * near_time; 
        if coll_dist.squared() < near.squared() {
            // this comes from completing the square in (p + vt)^2 = d^2
            let diff_squared = (coll_dist.squared() - rel_pos.squared())
                             / rel_vel.squared()
                             + near_time.squared();
            let diff: units::Duration = units::Scalar::sqrt(diff_squared);
            let coll_time = near_time - diff;

            // we also say they miss if they are already inside eachother
            // TODO we need to be able to coordinate release of contact as well
            if coll_time < 0 {
                None
            } else {
                Some(time + coll_time)
            }
        } else {
            None
        }
    } else {
        None
    }
}


#[derive(Clone, Debug)]
pub struct Body {
    last_position: units::Position,
    current_velocity: units::Velocity,
    last_time: units::Time,
}

impl PartialEq for Body {
    fn eq(self: &Self, other: &Self) -> bool {
        // note that we don't check for equivalence
        // this is so that rounding errors are propagated consistently
        let pos_eq = self.last_position == other.last_position;
        let vel_eq = self.current_velocity == other.current_velocity;
        let time_eq = self.last_time == other.last_time;
        // stationary objects have the same position regardless of when they
        // became still.
        let time_eq_enough = {
            if vel_eq == Default::default() {
                true
            } else {
                time_eq
            }
        };
        pos_eq && vel_eq && time_eq_enough
    }
}

impl Eq for Body {}


impl Body {
    pub fn new(
        position: units::Position,
        velocity: units::Velocity,
        time: units::Time
    ) -> Body {
        Body {
            last_position: position,
            current_velocity: velocity,
            last_time: time,
        }
    }

    pub fn with_end_point(
        start: units::Position,
        end: units::Position,
        start_time: units::Time,
        travel_time: units::Duration,
    ) -> Body {
        if travel_time != 0 {
            Body {
                last_position: start,
                current_velocity: (end - start) / travel_time,
                last_time: start_time,
            }
        } else {
            Body::new_frozen(end)
        }
    }

    pub fn new_frozen(position: units::Position) -> Body {
        Body {
            last_position: position,
            current_velocity: Default::default(),
            last_time: Default::default(),
        }
    }

    pub fn position(&self, now: units::Time) -> units::Position {
        let dtime = now - self.last_time;
        let displacement = self.current_velocity * dtime;
        self.last_position + displacement
    }

    pub fn velocity(&self) -> units::Velocity {
        self.current_velocity
    }

    pub fn split(
        &self,
        velocity: units::Velocity,
        now: units::Time
    ) -> Body {
        Body {
            last_position: self.position(now),
            current_velocity: velocity,
            last_time: now,
        }
    }

    pub fn split_to(
        &self,
        end_point: units::Position,
        now: units::Time,
        end_time: units::Time
    ) -> Body {
        Body::with_end_point(
            self.position(now),
            end_point,
            now,
            end_time - now
        )
    }

    pub fn bounce(
        &mut self,
        velocity: units::Velocity,
        now: units::Time
    ) {
        *self = self.split(velocity, now);
    }

    pub fn bounce_to(
        &mut self,
        end_point: units::Position,
        now: units::Time,
        end_time: units::Time
    ) {
        *self = self.split_to(end_point, now, end_time);
    }

    pub fn freeze(&mut self, now: units::Time) {
        self.bounce(Default::default(), now);
    }
}


