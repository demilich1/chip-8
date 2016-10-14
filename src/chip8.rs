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
const DEFAULT_PC_INC: u16 = 2;

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

        self.pc += self.execute_opcode(opcode);

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

    fn execute_opcode(&mut self, opcode: OpCode) -> u16 {
        match opcode {
            OpCode::SYS { addr } => self.sys(addr),
            OpCode::CLR => self.clr(),
            OpCode::RET => self.ret(),
            OpCode::JUMP { addr } => self.jump(addr),
            OpCode::CALL { addr } => self.call(addr),
            OpCode::SKE { s, nn } => self.ske(s, nn),
            OpCode::SKNE { s, nn } => self.skne(s, nn),
            OpCode::LOAD { s, nn } => self.load(s, nn),
            OpCode::ADD { s, nn } => self.add(s, nn),
            OpCode::MOVE { s, t } => self.move_reg(s, t),
            OpCode::OR { s, t } => self.or(s, t),
            OpCode::AND { s, t } => self.and(s, t),
            OpCode::XOR { s, t } => self.xor(s, t),
            OpCode::ADDR { s, t } => self.addr(s, t),
            OpCode::SUB { s, t } => self.sub(s, t),
            OpCode::SHR { s } => self.shr(s),
            OpCode::SHL { s } => self.shl(s),
            OpCode::SKRNE { s, t } => self.skrne(s, t),
            OpCode::LOADI { addr } => self.loadi(addr),
            OpCode::JUMPI { addr } => self.jumpi(addr),
            OpCode::RAND { t, nn } => self.rand(t, nn),
            OpCode::DRAW { s, t, n } => self.draw(s, t, n),
            OpCode::BCD { s } => self.bcd(s),
            _ => panic!("opcode {:?} not implemented yet", opcode),
        }
    }

    fn sys(&self, addr: u16) -> u16 {
        println!("Ignoring opcode SYS to addr {:?}", addr);
        DEFAULT_PC_INC
    }

    fn clr(&mut self) -> u16 {
        self.screen.clear();
        DEFAULT_PC_INC
    }

    fn ret(&mut self) -> u16 {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
        // TODO the program counter gets incremented by 2 every time, is this correct?
        0
    }

    fn jump(&mut self, addr : u16) -> u16 {
        self.pc = addr;
        0
    }

    fn call(&mut self, addr: u16) -> u16 {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
        0
    }

    fn ske(&self, s: u8, nn: u8) -> u16 {
        if self.regs[s as usize] == nn {
            DEFAULT_PC_INC
        } else {
            DEFAULT_PC_INC + DEFAULT_PC_INC
        }
    }

    fn skne(&self, s: u8, nn: u8) -> u16 {
        if self.regs[s as usize] != nn {
            DEFAULT_PC_INC
        } else {
            DEFAULT_PC_INC + DEFAULT_PC_INC
        }
    }

    fn load(&mut self, s : u8, nn : u8) -> u16 {
        self.regs[s as usize] = nn;
        DEFAULT_PC_INC
    }

    fn add(&mut self, s : u8, nn : u8) -> u16 {
        self.regs[s as usize] += nn;
        DEFAULT_PC_INC
    }

    fn move_reg(&mut self, s : u8, t : u8) -> u16 {
        self.regs[t as usize] = self.regs[s as usize];
        DEFAULT_PC_INC
    }

    fn or(&mut self, s : u8, t : u8) -> u16 {
            self.regs[t as usize] = self.regs[s as usize] | self.regs[t as usize];
            DEFAULT_PC_INC
    }

    fn and(&mut self, s : u8, t : u8) -> u16 {
            self.regs[t as usize] = self.regs[s as usize] & self.regs[t as usize];
            DEFAULT_PC_INC
    }

    fn xor(&mut self, s : u8, t : u8) -> u16 {
            self.regs[t as usize] = self.regs[s as usize] ^ self.regs[t as usize];
            DEFAULT_PC_INC
    }

    fn addr(&mut self, s : u8, t : u8) -> u16 {
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
        };
        DEFAULT_PC_INC
    }

        fn sub(&mut self, s : u8, t : u8) -> u16 {
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
            };
            DEFAULT_PC_INC
        }

                fn shr(&mut self, s : u8) -> u16 {
                    let s_val = self.regs[s as usize];
                    self.regs[REG_F] = s_val & 0x0001;
                    self.regs[s as usize] = s_val >> 1;
                    DEFAULT_PC_INC
                }

                fn shl(&mut self, s : u8) -> u16 {
                    let s_val = self.regs[s as usize];
                    self.regs[REG_F] = s_val & (1 << 7);
                    self.regs[s as usize] = s_val << 1;
                    DEFAULT_PC_INC
                }

                        fn skrne(&mut self, s : u8, t : u8) -> u16 {
                            if self.regs[s as usize] != self.regs[t as usize] {
                                DEFAULT_PC_INC
                            } else {
                                DEFAULT_PC_INC + DEFAULT_PC_INC
                            }
                        }

                        fn loadi(&mut self, addr : u16) -> u16 {
                            self.i_reg = addr;
                            DEFAULT_PC_INC
                        }

                        fn jumpi(&mut self, addr : u16) -> u16 {
                            self.pc = self.i_reg + addr;
                            DEFAULT_PC_INC
                        }

                        fn rand(&mut self, t : u8, nn : u8) -> u16 {
                            let between = Range::new(0, nn);
                            let mut rng = rand::thread_rng();
                            self.regs[t as usize] = between.ind_sample(&mut rng);
                            DEFAULT_PC_INC
                        }

                        fn draw(&mut self, s : u8, t : u8, n: u8) -> u16 {
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
                            DEFAULT_PC_INC
                        }

                                        fn bcd(&mut self, s : u8) -> u16 {
                                            let vx = self.regs[s as usize];
                                            self.memory.set(self.i_reg, (vx / 100) as u8);
                                            self.memory.set(self.i_reg + 1, ((vx / 10) % 10) as u8);
                                            self.memory.set(self.i_reg + 2, ((vx % 100) % 10) as u8);
                                            DEFAULT_PC_INC
                                        }


}
