use rom::Rom;
use mem::Memory;
use screen::{Screen, ScreenBuffer};

const FONT_START_OFFSET: u16 = 0x050;
const ROM_START_OFFSET: u16 = 0x200;
const REGISTERS: usize = 16;

static FONT_DATA: &'static [u8] =
&[
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

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
        self.memory.load_data(FONT_DATA, FONT_START_OFFSET);
        println!("CHIP-8 init", );
    }

    pub fn load_rom(&mut self, rom: Rom) {
        let rom_data: Vec<u8> = rom.into();
        self.memory.load_data(&rom_data, ROM_START_OFFSET);
    }

    pub fn run_cycle(&mut self) {
        // fetch opcode
        let opcode_raw = self.fetch_opcode();

        println!("Fetched opcode 0x{:X} at PC 0x{:X}", opcode_raw, self.pc);
        let mut screen_buffer = ScreenBuffer::new();
        screen_buffer.set_pixel(10, 10);
        self.screen.draw(screen_buffer);

        // each instruction is 2 bytes long
        self.pc += 2;

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn fetch_opcode(&self) -> u16 {
        let p1 = (self.memory.get(self.pc) as u16) << 8;
        let p2 = self.memory.get(self.pc + 1) as u16;
        p1 | p2
    }
}
