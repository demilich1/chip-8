#[derive(Debug, Copy, Clone)]
pub struct Keys {
    keys: u16
}

impl Keys {
    pub fn new() -> Self {
        Keys {
            keys: 0
        }
    }

    pub fn set(&mut self, index: u8) {
        self.keys |= 1 << index;
    }

    pub fn unset(&mut self, index: u8) {
        self.keys &= 0 << index;
    }

    pub fn get(&self, index: u8) -> bool {
        let bit = 1 << index;
        (self.keys & bit) == bit
    }

}
