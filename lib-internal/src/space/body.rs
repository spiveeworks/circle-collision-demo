use std::any;
use std::cmp;

use entities;
use space;
use sulphate;
use units;

pub trait Collide: entities::Display + any::Any where Self: Sized {
    /// called when an object moves into Self or teleports into Self
    fn collide(this: space::Entry<Self>, other: space::Image);
    /// called when an object moves out of Self
    fn release(this: space::Entry<Self>, other: space::Image);
    /// called when an object teleports out of Self
    fn disappear(this: space::Entry<Self>, other: space::Image);
}

/* this is quite complicated
 *
 * if an entity hasn't changed position/velocity/radius, then nothing happens
 *
 * if it changes velocity but neither position nor radius, then a special check
 * occurs over the next instant to see which entities it is currently in
 * contact with
 * this way when you collide with an entity you remain in contact with it,
 * even if the calculations are inaccurate.
 *
 * if it changes either position or radius, then it needs to restart the
 * ray-march process, checking entities with which it is currently in contact,
 * and checking if it is in contact with stationary objects, whereas both of
 * these things are normally implicit/ignored
 */
pub fn update_physics(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    matter: &mut sulphate::EntityHeap,
    uid: sulphate::EntityUId,
    maybe_image: Option<&space::Image>,
    maybe_before: Option<&space::Image>,
) {
    let maybe_n = space.find_uid(uid);
    let mut bounce = false;

    // do nothing if the image is the same
    if let (Some(n), Some(image)) = (maybe_n, maybe_image) {
        let c_body = &space.contents[n].1;

        if c_body.body == image.body
            && c_body.radius == image.inner_image.radius()
        {
            return;
        }

        let time_now = time.now();
        if c_body.body.position(time_now) == image.body.position(time_now)
            && c_body.radius == image.inner_image.radius()
        {
            bounce = true;
        }
    }

    // remove the body's old location, to reset its priority
    if let Some(n) = maybe_n {
        if maybe_image.is_none() {
            let before = maybe_before.expect(
                "nonexistent entity with body tried to disappear"
            );
            for other in space.get_contacts(uid) {
                perform_disappearance(space, time, matter, uid, other,
                                      before.clone());
            }
        }

        space.contents.remove(n);
    }

    // add the new body, giving it the lowest priority:
    // it is responsible for any collisions with currently existent entities
    if let Some(image) = maybe_image {
        let body = image.body.clone();
        let speed = body.velocity().magnitude();
        let radius = image.inner_image.radius();
        let physics_state = {
            if bounce {
                PhysicsState::Bounce(time.now())
            } else {
                PhysicsState::NoMarch
            }
        };

        let c_body = CollisionBody { body, speed, radius, physics_state };
        space.contents.push((uid, c_body));

        if bounce {
            let bounce_event = MarchEvent { uid };
            sulphate::enqueue_relative(time, bounce_event, units::instants(1));
        } else {
            let contact = space.get_contacts(uid);
            march_relocated(space, time, matter, uid, contact, maybe_before);
        }
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

    let (march, collisions) =
        get_march_data(space, time.now(), n);
    apply_march(space, time, uid, n, march);
    apply_collisions(space, time, uid, collisions);
}

// used when the position/radius has changed,
// as such current contacts, and stationary contacts must be considered
// where they normally would not be
fn march_relocated(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    matter: &mut sulphate::EntityHeap,
    uid: sulphate::EntityUId,
    contact: Vec<sulphate::EntityUId>,
    maybe_before: Option<&space::Image>,
) {
    let n = space.find_uid(uid);
    if n.is_none() {
        return;
    }
    let n = n.unwrap();

    let (march, releases, collisions, new_stable) =
        get_march_relocated_data(space, time.now(), n, contact);

    // TODO disappearances and new_stable_contacts can cause problems
    //      since they might cause a second update_physics to be called
    //      similarly if apply_collisions starts calling events directly
    apply_march(space, time, uid, n, march);
    // NOTE this must be called before apply_collisions
    if releases.len() > 0 {
        let before = maybe_before.expect(
            "nonexistent entity with contacts tried to appear"
        );
        apply_disappearances(space, time, matter, uid, releases, before);
    }
    apply_collisions(space, time, uid, collisions);
    apply_new_stable_contacts(space, time, matter, uid, new_stable);
}

fn apply_march(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    uid: sulphate::EntityUId,
    n: usize,
    march: Option<units::Time>,
) {
    if let Some(march_time) = march {
        let march_event = MarchEvent { uid };
        sulphate::enqueue_absolute(time, march_event, march_time);
    }
    space.contents[n].1.physics_state = march.map_or(
        PhysicsState::NoMarch,
        |t| PhysicsState::March(t),
    );
}

fn apply_collisions(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    uid: sulphate::EntityUId,
    collisions: Vec<(units::Time, units::Time, sulphate::EntityUId)>,
) {
    let this = ContactData::new(space, uid);
    for (coll_time, release_time, second_uid) in collisions {
        let first = this.clone();
        let second = ContactData::new(space, second_uid);
        if space.are_in_contact(uid, second_uid) {
            let release_event = ReleaseEvent { first, second };
            sulphate::enqueue_absolute(time, release_event, release_time);
        } else if time.now() < release_time {
            let collide_event = CollideEvent { first, second, release_time };
            sulphate::enqueue_absolute(time, collide_event, coll_time);
        }
    }
}

fn apply_disappearances(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    matter: &mut sulphate::EntityHeap,
    uid: sulphate::EntityUId,
    releases: Vec<sulphate::EntityUId>,
    before: &space::Image,
) {
    for other in releases {
        perform_disappearance(space, time, matter, uid, other, before.clone());
    }
}

fn apply_new_stable_contacts(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    matter: &mut sulphate::EntityHeap,
    uid: sulphate::EntityUId,
    stable: Vec<sulphate::EntityUId>,
) {
    for other in stable {
        perform_contact(space, time, matter, uid, other, ContactType::Collision);
        space.in_contact.push((uid, other));
    }
}

enum MarchResult {
    /// Objects far from eachother and cannot collide with eachother until
    /// at least this time
    March(units::Time),
    /// Objects close to eachother and collide over this time
    Collide(units::Time, units::Time),
    /// Objects touching, and release at this time
    //Release(units::Time),
    /// Objects close but do not touch
    Miss,
    /// Objects have the same velocity and are touching
    StableContact,
    /// Objects have the same velocity and are not touching
    StableMiss,
}

fn get_march_data(
    space: &space::CollisionSpace,
    time_now: units::Time,
    n: usize,
) -> (
    Option<units::Time>,
    Vec<(units::Time, units::Time, sulphate::EntityUId)>,
) {
    let (others, rest) = space.contents.split_at(n);
    let (_, ref this) = rest[0];

    let mut march = None;
    let mut collisions = Vec::new();

    for &(other_uid, ref other) in others {
        use self::MarchResult::*;
        match march_result(this, other, time_now) {
            Miss | StableMiss | StableContact => (),
            Collide(t, u) => {
                collisions.push((t, u, other_uid));
            },
            March(t) => {
                march = Some(march.map_or(t, |u| cmp::min(t, u)));
            },
        }
    }

    (march, collisions)
}

fn get_march_relocated_data(
    space: &space::CollisionSpace,
    time_now: units::Time,
    n: usize,
    contacts: Vec<sulphate::EntityUId>,
) -> (
    Option<units::Time>,
    Vec<sulphate::EntityUId>,  // don't need a release time?
    Vec<(units::Time, units::Time, sulphate::EntityUId)>,
    Vec<sulphate::EntityUId>,
) {
    debug_assert_eq!(
        n + 1, space.contents.len(),
        "entity's prioirity not reset"
    );

    let (others, rest) = space.contents.split_at(n);
    let (_, ref this) = rest[0];

    let mut march = None;
    let mut releases = Vec::new();
    let mut collisions = Vec::new();
    let mut stable_collisions = Vec::new();

    for &(other_uid, ref other) in others {
        use self::MarchResult::*;
        match march_result(this, other, time_now) {
            Miss | StableMiss => {
                if contacts.contains(&other_uid) {
                    releases.push(other_uid);
                }
            },
            StableContact => {
                if !contacts.contains(&other_uid) {
                    stable_collisions.push(other_uid);
                }
            },
            Collide(c, r) => {
                collisions.push((c, r, other_uid));
                if contacts.contains(&other_uid) &&
                    time_now < c || r <= time_now
                {
                    releases.push(other_uid);
                }
            },
            March(t) => {
                march = Some(march.map_or(t, |u| cmp::min(t, u)));
                if contacts.contains(&other_uid) {
                    releases.push(other_uid);
                }
            },
        }
    }

    (march, releases, collisions, stable_collisions)
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
        use self::CollideResult::*;
        return match collision_stationary(one, other) {
            Collision(_, _) => MarchResult::StableContact,
            Miss => MarchResult::StableMiss,
        };
    }
    let one_pos = one.body.position(time);
    let other_pos = other.body.position(time);
    let centre_dist_squared = (one_pos - other_pos).squared();

    // the maximum distance for testing collision
    let proximity = one.radius + other.radius + march_threshold();

    // if they are close enough, check for collision properly
    if centre_dist_squared < proximity.squared() {
        use self::CollideResult::*;
        match collision_linear(one, other) {
            Collision(Some(t), Some(u)) => MarchResult::Collide(t, u),
            Miss => MarchResult::Miss,
            _ => unreachable!(),
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

enum CollideResult {
    /// The two trajectories get closer than their radii between these times.
    /// None indicates an infinite duration
    Collision(Option<units::Time>, Option<units::Time>),
    /// The two trajectories never get closer than their radii
    Miss,
}

fn collision_linear(
    one: &CollisionBody,
    other: &CollisionBody,
) -> CollideResult {
    // we will work with a relative reference frame
    // we use max so that swapping the arguments doesnt change the result
    let time = cmp::max(one.body.last_time, other.body.last_time);
    let rel_pos = one.body.position(time) - other.body.position(time);
    let rel_vel = one.body.velocity() - other.body.velocity();

    // this is the time at which the bodies will be closest
    // it follows from assuming (p + vt) is orthogonal to v
    let inner = units::Vector::inner(rel_pos, rel_vel);
    let near_time: units::Duration = - inner / rel_vel.squared();

    // they collide when there's no room between their boundaries
    let coll_dist: units::Distance = one.radius + other.radius;
    let near: units::Displacement = rel_pos + rel_vel * near_time; 
    if coll_dist.squared() < near.squared() {
        CollideResult::Miss
    } else {
        // this comes from completing the square in (p + vt)^2 = d^2
        let diff_squared = (coll_dist.squared() - rel_pos.squared())
                         / rel_vel.squared()
                         + near_time.squared();
        let diff: units::Duration = units::Scalar::sqrt(diff_squared);
        let contact_time = time + near_time - diff;
        let release_time = time + near_time + diff + units::instants(1);

        CollideResult::Collision(Some(contact_time), Some(release_time))
    }
}

fn collision_stationary(
    one: &CollisionBody,
    other: &CollisionBody,
) -> CollideResult {
    let coll_dist = one.radius + other.radius;
    let centre_disp = one.body.last_position - other.body.last_position;
    if coll_dist.squared() < centre_disp.squared() {
        CollideResult::Miss
    } else {
        CollideResult::Collision(None, None)
    }
}

#[derive(Clone, PartialEq)]
struct ContactData {
    body: space::Body,
    radius: units::Distance,
    uid: sulphate::EntityUId,
}

impl ContactData {
    fn new(
        space: &space::CollisionSpace,
        uid: sulphate::EntityUId,
    ) -> Self {
        ContactData::try_new(space, uid).expect(
            "Constructing ContactData for entity that isn't in the space"
        ).0
    }

    fn try_new(
        space: &space::CollisionSpace,
        uid: sulphate::EntityUId,
    ) -> Option<(Self, Option<units::Time>)> {
        space.get_uid(uid).map(|c_body| {
            let body = c_body.body.clone();
            let radius = c_body.radius;
            (
                ContactData { body, radius, uid },
                if let PhysicsState::Bounce(t) = c_body.physics_state {
                    Some(t)
                } else {
                    None
                },
            )
        })
    }

    fn is_bounce(
        self: &Self,
        other: &Self,
        time: units::Time,
    ) -> bool {
        self.radius == other.radius &&
            self.body.position(time) == other.body.position(time)
    }
}

struct CollideEvent {
    first: ContactData,
    second: ContactData,
    release_time: units::Time,
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
        if contact_event_relevant(space, time.now(), &self.first, &self.second)
            && !space.are_in_contact(self.first.uid, self.second.uid)
        {
            perform_contact(
                space,
                time,
                matter,
                self.first.uid,
                self.second.uid,
                ContactType::Collision,
            );
        }

        let release_event = ReleaseEvent {
            first: self.first,
            second: self.second,
        };
        sulphate::enqueue_absolute(time, release_event, self.release_time);
    }
}

fn perform_contact(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    matter: &mut sulphate::EntityHeap,
    first_uid: sulphate::EntityUId,
    second_uid: sulphate::EntityUId,
    contact_type: ContactType,
) {
    match contact_type {
        ContactType::Collision => {
            space.in_contact.push((first_uid, second_uid));
        },
        ContactType::Release | ContactType::Disappear => {
            space.release_contact(first_uid, second_uid);
        },
    }

    invoke_contact(space, time, matter, first_uid, second_uid, contact_type);
    invoke_contact(space, time, matter, second_uid, first_uid, contact_type);
}

fn perform_disappearance(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    matter: &mut sulphate::EntityHeap,
    first_uid: sulphate::EntityUId,
    second_uid: sulphate::EntityUId,
    first_image: space::Image,
) {
    space.release_contact(first_uid, second_uid);

    let contact = ContactType::Disappear;
    invoke_contact(space, time, matter, first_uid, second_uid, contact);
    invoke_contact_with(space, time, matter, second_uid, first_image, contact);
}

struct ReleaseEvent {
    first: ContactData,
    second: ContactData,
}

impl sulphate::Event for ReleaseEvent {
    fn invoke(
        self: Self,
        space: &mut space::CollisionSpace,
        time: &mut sulphate::EventQueue,
        matter: &mut sulphate::EntityHeap,
    ) {
        if contact_event_relevant(space, time.now(), &self.first, &self.second)
            && space.are_in_contact(self.first.uid, self.second.uid)
        {
            perform_contact(
                space,
                time,
                matter,
                self.first.uid,
                self.second.uid,
                ContactType::Release,
            );
        }
    }
}




fn contact_event_relevant(
    space: &mut space::CollisionSpace,
    time: units::Time,
    first: &ContactData,
    second: &ContactData,
) -> bool {
        let first_now = ContactData::try_new(space, first.uid);
        let second_now = ContactData::try_new(space, second.uid);

        if first_now == None || second_now == None {
            return false;
        }

        let (first_now, first_bounce) = first_now.unwrap();
        let (second_now, second_bounce) = second_now.unwrap();

        let first_is_bounce = first_bounce.map_or(false, |t|
            t == time &&
            ContactData::is_bounce(&first_now, first, time)
        );
        let second_is_bounce = second_bounce.map_or(false, |t|
            t == time &&
            ContactData::is_bounce(&second_now, second, time)
        );

        (first_is_bounce || first_now == *first) &&
            (second_is_bounce || second_now == *second)
}

fn invoke_contact(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    matter: &mut sulphate::EntityHeap,
    this_uid: sulphate::EntityUId,
    with_uid: sulphate::EntityUId,
    contact_type: ContactType,
) {
    let inner_image = entities::image_of(matter, with_uid).expect(
        "Collided with body of nonexistent entity"
    );
    let body = space.get_uid(with_uid).unwrap().body.clone();
    let with = space::Image { body, inner_image };

    invoke_contact_with(space, time, matter, this_uid, with, contact_type);
}

fn invoke_contact_with(
    space: &mut space::CollisionSpace,
    time: &mut sulphate::EventQueue,
    matter: &mut sulphate::EntityHeap,
    this_uid: sulphate::EntityUId,
    with: space::Image,
    contact_type: ContactType,
) {
    if this_uid.ty == any::TypeId::of::<entities::Player>() {
        let ent = space.entry::<entities::Player>(time, matter, this_uid.id);

        use self::ContactType::*;
        match contact_type {
            Collision => Collide::collide(ent, with),
            Release => Collide::release(ent, with),
            Disappear => Collide::disappear(ent, with),
        }
    }
}

#[derive(Clone, Copy)]
enum ContactType {
    Collision,
    Release,
    Disappear,
}

struct MarchEvent {
    uid: sulphate::EntityUId
}

impl MarchEvent {
    fn still_relevant(
        self: &Self,
        space: &space::CollisionSpace,
        _matter: &sulphate::EntityHeap,
        _time_now: units::Time,
        exec_time: units::Time,
    ) -> bool {
        let physics_state = space.get_uid(self.uid)
                                 .map(|body| body.physics_state);
        use self::PhysicsState::*;
        match physics_state {
            None => false,
            Some(March(march_time)) =>
                exec_time == march_time,
            Some(Bounce(bounce_time)) =>
                exec_time == bounce_time + units::instants(1),
            Some(NoMarch) => false,
        }
    }
}

impl sulphate::Event for MarchEvent {
    fn invoke(
        self: Self,
        space: &mut space::CollisionSpace,
        time: &mut sulphate::EventQueue,
        _matter: &mut sulphate::EntityHeap,
    ) {
        if self.still_relevant(space, _matter, time.now(), time.now()) {
            march(space, time, self.uid);
        }
    }
}

pub struct CollisionBody {
    pub body: Body,
    speed: units::Speed,
    physics_state: PhysicsState,
    radius: units::Distance,
}

#[derive(Clone, Copy)]
enum PhysicsState {
    NoMarch,
    March(units::Time),
    Bounce(units::Time),
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


