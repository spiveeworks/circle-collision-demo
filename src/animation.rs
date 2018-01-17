use std::collections::hash_map;

type Time = f64;
type Duration = f64;

type EntityUId = (u64, u64);

struct Body {
    position: [f64; 3],
    velocity: [f64; 3],
    updated: Time,
}

struct Image {
    body: Body,
    image: ImageVariant,
}

type ImageVariant = u64;

struct Scene {
    sequences: hash_map::HashMap<EntityUId, Sequence>,
}

struct Sequence {
    locations: Vec<(Duration, Image)>,
    final_duration: Duration,
}



