pub struct ScreenBuffer {
    width: u16,
    height: u16,
    pixels: Vec<bool>,
}

impl ScreenBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width * height) as usize;
        ScreenBuffer {
            width: width,
            height: height,
            pixels: vec![false; size] }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn clear(&mut self) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = false;
        }
    }

    pub fn xor(&mut self, x: u16, y: u16) -> bool {
        let index = self.index(x, y);
        let result = self.pixels[index];
        self.pixels[index] ^= true;
        result
    }

    pub fn get_pixel(&self, x: u16, y: u16) -> bool {
        let index = self.index(x, y);
        self.pixels[index]
    }

    fn index(&self, x: u16, y: u16) -> usize {
        let mut index: usize = (x * self.height() + y) as usize;
        index = index % self.pixels.len();
        index
    }
}
