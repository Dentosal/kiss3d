extern crate native;
extern crate kiss3d;
extern crate "nalgebra" as na;

use na::Pnt3;
use kiss3d::window::Window;
use kiss3d::light;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let mut window = Window::new("Kiss3d: points");

    window.set_light(light::StickToCamera);

    while window.render() {
        let a = Pnt3::new(-0.1, -0.1, 0.0);
        let b = Pnt3::new(0.0, 0.1, 0.0);
        let c = Pnt3::new(0.1, -0.1, 0.0);

        window.draw_point(&a, &Pnt3::new(1.0, 0.0, 0.0));
        window.draw_point(&b, &Pnt3::new(0.0, 1.0, 0.0));
        window.draw_point(&c, &Pnt3::new(0.0, 0.0, 1.0));
    }
}
