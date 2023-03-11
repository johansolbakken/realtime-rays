use profile::scope;
use renderer::Renderer;

use window::Window;

mod image;
mod profile;
mod renderer;
mod window;
mod utils;

pub fn main() {
    let mut window = Window::new("Realtime renderer", 800, 800);
    let mut renderer = Renderer::new();

    while !window.should_close() {
        let _profile = scope("Run loop");
        renderer.on_resize(window.get_width(), window.get_height());
        renderer.render();

        window.show(renderer.final_image());
        window.update();

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
