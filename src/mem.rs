const MEMORY_SIZE : usize = 4096;

pub struct Memory {
    memory: Vec<u8>
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: vec![0; MEMORY_SIZE]
        }
    }

    pub fn load_data(&mut self, data: Vec<u8>, offset: u16) {
        for i in 0..data.len() {
            let addr = offset + i as u16;
            self.set(addr, data[i]);
        }
    }

    pub fn set(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn get(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
}
