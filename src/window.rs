/*use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{PixelFormatEnum},
    rect::Rect,
    render::{Canvas, TextureCreator},
    video::{self, WindowContext},
    EventPump,
};

use crate::image::Image;

pub struct Window {
    canvas: Canvas<video::Window>,
    event_pump: EventPump,
    width: usize,
    height: usize,
    should_close: bool,
    texture_creator: TextureCreator<WindowContext>,
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
        let texture_creator = canvas.texture_creator();

        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            canvas,
            event_pump,
            width,
            height,
            should_close: false,
            texture_creator,
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
        let mut texture = self
            .texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ABGR8888,
                self.width as u32,
                self.height as u32,
            )
            .map_err(|e| e.to_string())
            .unwrap();

        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let offset = y * pitch + x * 4;
                        let color = image.get_pixel(x, image.get_height() - 1 - y);

                        let r = (color & 0xff) as u8;
                        let g = ((color >> 8) & 0xff) as u8;
                        let b = ((color >> 16) & 0xff) as u8;
                        let a = ((color >> 24) & 0xff) as u8;

                        buffer[offset] = r;
                        buffer[offset + 1] = g;
                        buffer[offset + 2] = b;
                        buffer[offset + 3] = a;
                    }
                }
            })
            .unwrap();

        self.canvas.clear();
        self.canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(0, 0, self.width as u32, self.height as u32)),
            )
            .unwrap();
        self.canvas
            .copy_ex(
                &texture,
                None,
                Some(Rect::new(0, 0, self.width as u32, self.height as u32)),
                0.0,
                None,
                false,
                false,
            )
            .unwrap();

        /*
        for y in 0..image.get_height() {
            for x in 0..image.get_width() {
                let color = image.get_pixel(x, y);

                let r = (color & 0xff) as u8;
                let g = ((color >> 8) & 0xff) as u8;
                let b = ((color >> 16) & 0xff) as u8;
                let a = ((color >> 24) & 0xff) as u8;

                self.canvas.set_draw_color(Color::RGBA(r, g, b, a));
                let rect = Rect::new(x as i32, (image.get_height() - 1 - y) as i32, 1, 1);
                self.canvas.fill_rect(rect).expect("Failed to fill rect");
            }
        }
        */
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }
}
*/