use rand;
use rand::distributions::{IndependentSample, Range};

use rom::Rom;
use mem::Memory;
use screen::{Screen, ScreenBuffer};

use opcode;
use opcode::OpCode;

const FONT_START_OFFSET: u16 = 0x050;
const ROM_START_OFFSET: u16 = 0x200;
const REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const REG_F: usize = 0xF;

#[cfg_attr(rustfmt, rustfmt_skip)]
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
    stack: [u16; STACK_SIZE],
    screen_buffer: ScreenBuffer,
    sp: u16, // stack pointer
    pc: u16, // program counter
    i_reg: u16, // index register
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            screen: Screen::new(),
            memory: Memory::new(),
            regs: [0; REGISTERS],
            stack: [0; STACK_SIZE],
            screen_buffer: ScreenBuffer::new(),
            sp: 0,
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
        let opcode_raw = self.fetch_opcode();
        let opcode = opcode::decode(opcode_raw);
        println!("Fetched opcode {:?} at PC 0x{:X}", &opcode, self.pc);

        self.execute_opcode(opcode);

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

    fn execute_opcode(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::SYS { addr } => println!("Ignoring opcode {:?}", opcode),
            OpCode::CLR => self.screen.clear(),
            OpCode::RET => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
                // TODO the program counter gets incremented by 2 every time, is this correct?
            }
            OpCode::JUMP { addr } => self.pc = addr,
            OpCode::CALL { addr } => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = addr;
            }
            OpCode::SKE { s, nn } => {
                if self.regs[s as usize] == nn {
                    self.pc += 2;
                }
            }
            OpCode::SKNE { s, nn } => {
                if self.regs[s as usize] != nn {
                    self.pc += 2;
                }
            }
            OpCode::LOAD { s, nn } => {
                self.regs[s as usize] = nn;
            }
            OpCode::ADD { s, nn } => {
                self.regs[s as usize] += nn;
            }
            OpCode::MOVE { s, t } => {
                self.regs[t as usize] = self.regs[s as usize];
            }
            OpCode::OR { s, t } => {
                self.regs[t as usize] = self.regs[s as usize] | self.regs[t as usize];
            }
            OpCode::AND { s, t } => {
                self.regs[t as usize] = self.regs[s as usize] & self.regs[t as usize];
            }
            OpCode::XOR { s, t } => {
                self.regs[t as usize] = self.regs[s as usize] ^ self.regs[t as usize];
            }
            OpCode::ADDR { s, t } => {
                let s_val = self.regs[s as usize];
                let t_val = self.regs[t as usize];
                // first perform a checked addition
                self.regs[s as usize] = match s_val.checked_add(t_val) {
                    Some(result) => {
                        // no overflow occured, unset carry flag and return result
                        self.regs[REG_F] = 0;
                        result
                    }
                    None => {
                        // overflow occured, set carry flag and save wrapped result
                        self.regs[REG_F] = 1;
                        s_val.wrapping_add(t_val)
                    }
                }
            }
            OpCode::SUB { s, t } => {
                let s_val = self.regs[s as usize];
                let t_val = self.regs[t as usize];
                // first perform a checked addition
                self.regs[s as usize] = match t_val.checked_sub(s_val) {
                    Some(result) => {
                        // no overflow occured, unset carry flag and return result
                        self.regs[REG_F] = 0;
                        result
                    }
                    None => {
                        // overflow occured, set carry flag and save wrapped result
                        self.regs[REG_F] = 1;
                        t_val.wrapping_sub(s_val)
                    }
                }
            }
            OpCode::SHR { s } => {
                let s_val = self.regs[s as usize];
                self.regs[REG_F] = s_val & 0x0001;
                self.regs[s as usize] = s_val >> 1;
            }
            OpCode::SHL { s } => {
                let s_val = self.regs[s as usize];
                self.regs[REG_F] = s_val & (1 << 7);
                self.regs[s as usize] = s_val << 1;
            }
            OpCode::SKRNE { s, t } => {
                if self.regs[s as usize] != self.regs[t as usize] {
                    self.pc += 2;
                }
            }
            OpCode::LOADI { nnn } => self.i_reg = nnn,
            OpCode::JUMPI { addr } => self.pc = self.i_reg + addr,
            OpCode::RAND { t, nn } => {
                let between = Range::new(0, nn);
                let mut rng = rand::thread_rng();
                self.regs[t as usize] = between.ind_sample(&mut rng);
            }
            OpCode::DRAW { s, t, n } => {
                let sx = self.regs[s as usize];
                let sy = self.regs[t as usize];
                println!("Base Sprite at {:?}, {:?}...", sx, sy);

                self.regs[REG_F] = 0;
                for y_line in 0..n {
                    let pixel_row = self.memory.get(self.i_reg + y_line as u16);
                    for x_line in 0..8 {
                        match pixel_row & (0x80 >> x_line) {
                            0 => (),
                            _ => {
                                let x = (sx + x_line) as u32;
                                let y = (sy + y_line) as u32;
                                println!("Drawing at {:?}, {:?}...", x, y);
                                if self.screen_buffer.xor(x, y) {
                                    self.regs[REG_F] = 1;
                                }
                            }
                        };
                    }
                }
                self.screen.draw(self.screen_buffer.clone());
            }
            OpCode::BCD { s } => {
                let vx = self.regs[s as usize];
                self.memory.set(self.i_reg, (vx / 100) as u8);
                self.memory.set(self.i_reg + 1, ((vx / 10) % 10) as u8);
                self.memory.set(self.i_reg + 2, ((vx % 100) % 10) as u8);
            }
            _ => panic!("opcode {:?} not implemented yet", opcode),
        };
    }
}
