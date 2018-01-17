extern crate piston_app;

extern crate induction;

fn main() {
    let app = induction::start_app();

    piston_app::run_until_escape(app);
}
