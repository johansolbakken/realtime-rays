use crate::image::Image;

pub struct Renderer {
    image: Image,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            image: Image::new(0, 0),
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.image.resize(width, height);
    }

    pub fn final_image(&self) -> &Image {
        &self.image
    }

    pub fn render(&mut self) {
        for y in 0..self.image.get_height() {
            for x in 0..self.image.get_width() {
                self.image.set_pixel(x, y, 0xff00ff00); // ABGR
            }
        }
    }
}
