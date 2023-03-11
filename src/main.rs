use camera::Camera;
use profile::scope;
use renderer::Renderer;

use scene::{create_test_scene};
use window::Window;

mod camera;
mod image;
mod profile;
mod ray;
mod renderer;
mod scene;
mod utils;
mod window;

pub fn main() {
    let mut window = Window::new("Realtime renderer", 1280, 860);
    let mut camera = Camera::new(45.0, 0.1, 100.0);
    let mut renderer = Renderer::new();

    let scene = create_test_scene();

    while !window.should_close() {
        let _profile = scope("Run loop");
        camera.on_resize(window.get_width(), window.get_height());
        camera.on_update(0.0);
        renderer.on_resize(window.get_width(), window.get_height());
        renderer.render(&scene, &camera);

        window.show(renderer.final_image());
        window.update();
    }
}
