use rom::Rom;
use mem::Memory;

const ROM_START_OFFSET : u16 = 0x200;

pub struct Chip8 {
    memory: Memory,
    pc: u16
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: Memory::new(),
            pc: 0
        }
    }

    pub fn init(&mut self) {
        self.pc = ROM_START_OFFSET;
        println!("CHIP-8 init", );
    }

    pub fn load_rom(&mut self, rom: Rom) {
        self.memory.load_data(rom.into(), ROM_START_OFFSET);
    }
}
