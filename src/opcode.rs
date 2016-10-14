#[derive(Debug)]
pub enum OpCode {
    SYS {addr: u16}, // 0nnn; System call (ignored)
    CLR, // 00E0; Clear the screen
    RET, // 00EE; Return from subroutine
    JUMP { addr: u16 }, // 1nnn; Jump to address nnn
    CALL { addr: u16 }, // 2nnn; Call routine at address
    SKE { s: u8, nn: u8 }, // 3snn; Skip next instruction if register s equals nn
    SKNE { s: u8, nn: u8 }, // 4snn; Do not skip next instruction if register s equals nn
    SKRE { s: u8, t: u8 }, // 5st0; Skip if register s equals register t
    LOAD { s: u8, nn: u8 }, // 6snn; Load register s with value nn
    ADD { s: u8, nn: u8 }, // 7snn; Add value nn to register s
    MOVE { s: u8, t: u8 }, // 8st0; Move value from register s to register t
    OR { s: u8, t: u8 }, // 8st1; Perform logical OR on register s and t and store in t
    AND { s: u8, t: u8 }, // 8st2; Perform logical AND on register s and t and store in t
    XOR { s: u8, t: u8 }, // 8st3; Perform logical XOR on register s and t and store in t
    ADDR { s: u8, t: u8 }, // 8st4; Add s to t and store in s - register F set on carry
    SUB { s: u8, t: u8 }, // 8st5; Subtract s from t and store in s - register F set on !borrow
    SHR { s: u8 }, // 8s06; Shift bits in register s 1 bit to the right - bit 0 shifts to register F
    SHL { s: u8 }, // 8s0E; Shift bits in register s 1 bit to the left - bit 7 shifts to register F
    SKRNE { s: u8, t: u8 }, // 9st0; Skip next instruction if register s not equal register t
    LOADI { addr: u16 }, // Annn; Load index with value nnn
    JUMPI { addr: u16 }, // Bnnn; Jump to address nnn + index
    RAND { t: u8, nn: u8 }, // Ctnn; Generate random number between 0 and nn and store in t
    DRAW { s: u8, t: u8, n: u8 }, // Dstn; Draw n byte sprite at x location reg s, y location reg t
    MOVED { t: u8 }, // Ft07; Move delay timer value into register t
    KEYD { t: u8 }, // Ft0A; Wait for keypress and store in register t
    LOADD { s: u8 }, // Fs15; Load delay timer with value in register s
    LOADS { s: u8 }, // Fs18; Load sound timer with value in register s
    ADDI { s: u8 }, // Fs1E; Add value in register s to index
    LDSPR { s: u8 }, // Fs29; Load index with sprite from register s
    BCD { s: u8 }, // Fs33; Store the binary coded decimal value of register s at index
    STOR { s: u8 }, // Fs55; Store the values of register s registers at index
    READ { s: u8 }, // Fs65; Read back the stored values at index into registers
}

pub fn decode(val: u16) -> OpCode {
    //println!("Decoding opcode 0x{:X}", val);
    let first_nibble = val & 0xF000;
    match first_nibble {
        0x0000 => {
            match get_n234(val) {
                0x00E0 => OpCode::CLR,
                0x00EE => OpCode::RET,
                0x0000 => OpCode::SYS{addr: get_n234(val)},
                _ => panic!("Invalid OpCode 0x{:X}", val),
            }
        }
        0x1000 => OpCode::JUMP { addr: get_n234(val) },
        0x2000 => OpCode::CALL { addr: get_n234(val) },
        0x3000 => OpCode::SKE { s: get_n2(val), nn: get_n34(val) },
        0x4000 => OpCode::SKNE { s: get_n2(val) as u8, nn: get_n34(val) },
        0x5000 => OpCode::SKRE { s: get_n2(val) as u8, t: get_n3(val) },
        0x6000 => OpCode::LOAD { s: get_n2(val) as u8, nn: get_n34(val) },
        0x7000 => OpCode::ADD { s: get_n2(val) as u8, nn: get_n34(val) },
        0x8000 => {
            let last_nibble = val & 0x000F;
            let s = get_n2(val);
            let t = get_n3(val);
            match last_nibble {
                0 => OpCode::MOVE {s: s, t: t},
                1 => OpCode::OR {s: s, t: t},
                2 => OpCode::AND {s: s, t: t},
                3 => OpCode::XOR {s: s, t: t},
                4 => OpCode::ADDR {s: s, t: t},
                5 => OpCode::SUB {s: s, t: t},
                6 => OpCode::SHR {s: s},
                _ => panic!("Invalid OpCode 0x{:X}", val),
            }
        },
        0x9000 => OpCode::SKRNE { s: get_n2(val), t: get_n3(val) },
        0xA000 => OpCode::LOADI { addr: get_n234(val) },
        0xB000 => OpCode::JUMPI { addr: get_n234(val) },
        0xC000 => OpCode::RAND { t: get_n2(val), nn: get_n34(val) },
        0xD000 => OpCode::DRAW { s: get_n2(val), t: get_n3(val), n: get_n4(val) },
        0xF000 => {
            let n34 = get_n34(val);
            let reg = get_n2(val);
            match n34 {
                0x0007 => OpCode::MOVED {t: reg},
                0x000A => OpCode::KEYD {t: reg},
                0x0015 => OpCode::LOADD {s: reg},
                0x0018 => OpCode::LOADS {s: reg},
                0x001E => OpCode::ADDI {s: reg},
                0x0029 => OpCode::LDSPR {s: reg},
                0x0033 => OpCode::BCD {s: reg},
                0x0055 => OpCode::STOR {s: reg},
                0x0065 => OpCode::READ {s: reg},
                _ => panic!("Invalid OpCode 0x{:X}", val),
            }
        },
        _ => panic!("Invalid OpCode 0x{:X}", val),
    }
}

#[inline]
fn get_n2(opcode : u16) -> u8 {
    (opcode >> 8 & 0x000F) as u8
}

#[inline]
fn get_n3(opcode : u16) -> u8 {
    (opcode >> 4 & 0x000F) as u8
}

#[inline]
fn get_n4(opcode : u16) -> u8 {
    (opcode & 0x000F) as u8
}

#[inline]
fn get_n34(opcode : u16) -> u8 {
    (opcode & 0x00FF) as u8
}

#[inline]
fn get_n234(opcode : u16) -> u16 {
    opcode & 0x0FFF
}
