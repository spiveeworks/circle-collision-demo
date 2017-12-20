use city_internal::entities;
use city_internal::space;
use city_internal::units;

use piston_window as app;

pub fn draw(
    draw: &space::Image,
    time: units::Time,
    center: app::math::Matrix2d,
    graphics: &mut app::G2d,
) {
    // transform screen so that player is at the center
    let position = draw.body.position(time);
    let trans = center_on(center, position);

    use city_internal::entities::Image::*;
    match draw.inner_image {
        Player(ref player) => {
            player.draw(time, trans, graphics);
        },
    }
}

fn center_on<T>(center: T, position: units::Position) -> T
    where T: app::Transformed
{
    let pos = floatify_position(position);
    center.trans(pos[0], pos[1])
}

trait Draw {
    fn draw(
        self: &Self,
        time: units::Time,
        center: app::math::Matrix2d,
        graphics: &mut app::G2d,
    );
}

impl Draw for entities::player::Image {
    fn draw(
        self: &Self,
        time: units::Time,
        center: app::math::Matrix2d,
        graphics: &mut app::G2d,
    ) {
        // draw a circle at this new center
        let color = [1.0, 0.0, 0.0, 1.0];
        let radius = 10.0;
        let circle = Circle { color, radius };
        circle.draw(time, center, graphics);
    }
}


fn floatify_position(position: units::Position) -> [f64; 2] {
    let origin: units::Position = Default::default();
    let vec = position - origin;
    [vec.x.into(), vec.y.into()]
}


struct Circle {
    color: [f32; 4],
    radius: f64,
}

impl Draw for Circle {
    fn draw(
        self: &Self,
        _time: units::Time,
        trans: app::math::Matrix2d,
        graphics: &mut app::G2d
    ) {
        let Circle { color, radius } = *self;
        // we could transform but this seems clearer
        let rect = [-radius, -radius, 2.0 * radius, 2.0 * radius];

        app::ellipse(color, rect, trans, graphics);
    }
}




