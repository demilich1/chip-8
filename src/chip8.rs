use rom::Rom;
use mem::Memory;
use screen::Screen;

const ROM_START_OFFSET: u16 = 0x200;
const REGISTERS: usize = 16;

pub struct Chip8 {
    screen: Screen,
    memory: Memory,
    regs: [u8; REGISTERS],
    pc: u16,
    i_reg: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            screen: Screen::new(),
            memory: Memory::new(),
            regs: [0; REGISTERS],
            pc: 0,
            i_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn init(&mut self) {
        self.pc = ROM_START_OFFSET;
        println!("CHIP-8 init", );
    }

    pub fn load_rom(&mut self, rom: Rom) {
        self.memory.load_data(rom.into(), ROM_START_OFFSET);
    }

    pub fn run_cycle(&mut self) {
        // fetch opcode
        let opcode_raw = self.fetch_opcode();

        println!("Fetched opcode 0x{:X} at PC {:?}", opcode_raw, self.pc);

        self.pc += 1;
    }

    fn fetch_opcode(&self) -> u16 {
        let p1 = (self.memory.get(self.pc) as u16) << 8;
        let p2 = self.memory.get(self.pc + 1) as u16;
        p1 | p2
    }
}
