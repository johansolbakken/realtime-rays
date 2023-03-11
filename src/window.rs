use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video, EventPump,
};

use crate::image::Image;

pub struct Window {
    canvas: Canvas<video::Window>,
    event_pump: EventPump,
    width: usize,
    height: usize,
    should_close: bool,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(title, width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            canvas,
            event_pump,
            width,
            height,
            should_close: false,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn update(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.should_close = true;
                }
                _ => {}
            }
        }

        self.canvas.present();
    }

    pub fn show(&mut self, image: &Image) {
        for y in 0..image.get_height() {
            for x in 0..image.get_width() {
                let color = image.get_pixel(x, y);

                let r = (color & 0xff) as u8;
                let g = ((color >> 8) & 0xff) as u8;
                let b = ((color >> 16) & 0xff) as u8;
                let a = ((color >> 24) & 0xff) as u8;

                self.canvas.set_draw_color(Color::RGBA(r, g, b, a));
                let rect = Rect::new(x as i32, y as i32, 1, 1);
                self.canvas.fill_rect(rect).expect("Failed to fill rect");
            }
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }
}
