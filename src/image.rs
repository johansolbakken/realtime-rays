use std::iter::repeat;

pub type Color = u32;

#[derive(Debug)]
pub struct Image {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: repeat(0).take(width * height).collect::<Vec<_>>(),
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width == width && self.height == height {
            return;
        }

        self.pixels = repeat(0).take(width * height).collect::<Vec<_>>();
        self.width = width;
        self.height = height;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let location = x + y * self.width;
        assert!(location < self.width * self.height);
        self.pixels[location] = color;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let location = x + y * self.width;
        assert!(location < self.width * self.height);
        self.pixels[location]
    }
}
