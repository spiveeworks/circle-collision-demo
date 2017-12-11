use std::any;
use std::cmp;

use entities;
use space;
use sulphate;
use units;

pub trait Collide: entities::Display + any::Any where Self: Sized {
    fn collide(this: space::Entry<Self>, other: space::Image);
}

pub(super) fn update_physics(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    uid: sulphate::EntityUId,
    maybe_image: Option<&space::Image>,
) {
    let maybe_n = space.find_uid(uid);

    // do nothing if the image is the same
    if let (Some(n), Some(image)) = (maybe_n, maybe_image) {
        let c_body = &space.contents[n].1;

        if c_body.body == image.body
            && c_body.radius == image.inner_image.radius()
        {
            return;
        }
    }

    // remove the body's old location, to reset its priority
    if let Some(n) = maybe_n {
        space.contents.remove(n);
    }

    // add the new body, giving it the lowest priority:
    // it is responsible for any collisions with currently existent entities
    if let Some(image) = maybe_image {
        let body = image.body.clone();
        let speed = body.velocity().magnitude();
        let radius = image.inner_image.radius();
        let march_time = None;

        let c_body = CollisionBody { body, speed, radius, march_time };
        space.contents.push((uid, c_body));

        march(space, time, uid);
    }
}

fn march(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    uid: sulphate::EntityUId,
) {
    let n = space.find_uid(uid);
    if n.is_none() {
        return;
    }
    let n = n.unwrap();

    let (march, collisions) = get_march_data(space, time.now(), n);
    apply_march_data(space, time, uid, n, march, collisions);
}

fn apply_march_data(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    uid: sulphate::EntityUId,
    n: usize,
    march: Option<units::Time>,
    collisions: Vec<(units::Time, sulphate::EntityUId)>,
) {
    if let Some(march_time) = march {
        let march_event = MarchEvent { uid };
        sulphate::enqueue_absolute(time, march_event, march_time);
    }
    space.contents[n].1.march_time = march;

    let this = CollideData::new(space, uid);
    for (coll_time, second_uid) in collisions {
        let first = this.clone();
        let second = CollideData::new(space, second_uid);
        let collide_event = CollideEvent { first, second };
        sulphate::enqueue_absolute(time, collide_event, coll_time);
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
    /// Objects have the same velocity so cannot collide
    Stable,
}

fn get_march_data(
    space: &space::CollisionSpace,
    time_now: units::Time,
    n: usize,
) -> (Option<units::Time>, Vec<(units::Time, sulphate::EntityUId)>) {
    let (others, rest) = space.contents.split_at(n);
    let (_, ref this) = rest[0];

    let mut march = None;
    let mut collisions = Vec::with_capacity(others.len());

    for &(other_uid, ref other) in others {
        use self::MarchResult::*;
        match march_result(this, other, time_now) {
            Miss | Stable => (),
            Collide(t) => {
                collisions.push((t, other_uid));
            },
            March(t) => {
                march = Some(march.map_or(t, |u| cmp::min(t, u)));
            },
        }
    }

    (march, collisions)
}

// this is the edge-to-edge distance at which ray-marching and precise hit-scan
// are considered equally preferable
// this is a variable for optimization,
// but its optimal value will change between builds,
// hypothetically you could select this value during an automated release
// process
// TODO change this to be a speed, and deal with the consequences
// (such as sufficiently slow objects getting radically inaccurate march times)
fn march_threshold() -> units::Distance {
    5.into()
}

fn march_result(
    one: &CollisionBody,
    other: &CollisionBody,
    time: units::Time
) -> MarchResult {
    if one.body.velocity() == other.body.velocity() {
        return MarchResult::Stable;
    }
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
        // end up inside the threshold
        let edge_dist = centre_dist - one.radius - other.radius;
        // safe since speeds are positive and unequal
        let march_time = edge_dist / (one.speed + other.speed);
        // don't subtract the threshold number, so that we
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
            None
        } else {
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
        }
    } else {
        None
    }
}


#[derive(Clone, PartialEq)]
struct CollideData {
    body: space::Body,
    radius: units::Distance,
    uid: sulphate::EntityUId,
}

impl CollideData {
    fn new(
        space: &space::CollisionSpace,
        uid: sulphate::EntityUId,
    ) -> Self {
        CollideData::try_new(space, uid).expect(
            "Constructing CollideData for entity that isn't in the space"
        )
    }

    fn try_new(
        space: &space::CollisionSpace,
        uid: sulphate::EntityUId,
    ) -> Option<Self> {
        space.get_uid(uid).map(|c_body| {
            let body = c_body.body.clone();
            let radius = c_body.radius;
            CollideData { body, radius, uid }
        })
    }

    /*
    /// updates the radius field to match the current state of the entity
    /// useful when followed by an equality comparison
    /// (if the entity is being changed at that instant...)
    fn update_radius(
        self: &mut Self,
        matter: &sulphate::EntityHeap,
    ) {
        if self.uid.ty == any::TypeId::of::<entities::Player> {
            let img = matter.get::<entities::Player>(uid.id)
                            .and_then(entities::Display::image);
            if let Some(image) = img {
                self.radius = img.radius;
            } else {
                false
            }
        } else {
            panic!("Unknown entity trying to collide");
        }
    }
    */
}

struct CollideEvent {
    first: CollideData,
    second: CollideData,
    // this would be faster than generating the current radius and comparing it
    // right?
    // enqueue_time: units::Time,  // the initiator should have the same time
}

impl sulphate::Event for CollideEvent {
    fn invoke(
        self: Self,
        space: &mut space::CollisionSpace,
        time: &mut sulphate::EventQueue,
        matter: &mut sulphate::EntityHeap,
    ) {
        let first_now = CollideData::try_new(space, self.first.uid);
        let second_now = CollideData::try_new(space, self.second.uid);

        if first_now.as_ref() != Some(&self.first)
        || second_now.as_ref() != Some(&self.second) {
            return;
        }

        if space.has_collided(time.now(), self.first.uid, self.second.uid) {
            return;
        }

        space.note_collided(time.now(), self.first.uid, self.second.uid);

        collide(space, time, matter, self.first.uid, self.second.uid);
        collide(space, time, matter, self.second.uid, self.first.uid);
    }
}

fn collide(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    matter: &mut sulphate::EntityHeap,
    this_uid: sulphate::EntityUId,
    with_uid: sulphate::EntityUId,
) {
    let inner_image = entities::image_of(matter, with_uid).expect(
        "Collided with body of nonexistent entity"
    );
    let body = space.get_uid(with_uid).unwrap().body.clone();
    let with = space::Image { body, inner_image };

    if this_uid.ty == any::TypeId::of::<entities::Player>() {
        let ent = space.entry::<entities::Player>(time, matter, this_uid.id);
        Collide::collide(ent, with);
    }
}


struct MarchEvent {
    uid: sulphate::EntityUId
}

impl sulphate::Event for MarchEvent {
    fn invoke(
        self: Self,
        space: &mut space::CollisionSpace,
        time: &mut sulphate::EventQueue,
        _matter: &mut sulphate::EntityHeap,
    ) {
        // has a body and that body is expecting to march right now
        let march_time = space.get_uid(self.uid)
                              .and_then(|body| body.march_time);
        if march_time == Some(time.now()) {
            march(space, time, self.uid);
        }
    }
}

pub struct CollisionBody {
    pub body: Body,
    speed: units::Speed,
    march_time: Option<units::Time>,
    radius: units::Distance,
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


