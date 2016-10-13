pub enum OpCode {
    SYS, // 0nnn; System call (ignored)
    CLR, // 00E0; Clear the screen
    RET, // 00EE; Return from subroutine
    JUMP { addr: u16 }, // 1nnn; Jump to address nnn
    CALL { addr: u16 }, // 2nnn; Call routine at address
    SKE { reg: u8, val: u8 }, // 3snn; Skip next instruction if register reg equals val
    SKNE { reg: u8, val: u8 }, // Do not skip next instruction if register reg equals val
    SKRE { reg1: u8, reg2: u8 }, // Skip if register s equals register t
    LOAD { reg: u8, val: u8 },
    ADD { reg: u8, val: u8 },
    MOVE { reg1: u8, reg2: u8 },
    OR { reg1: u8, reg2: u8 },
    AND { reg1: u8, reg2: u8 },
    XOR { reg1: u8, reg2: u8 },
    ADDR { reg1: u8, reg2: u8 },
    SUB { reg1: u8, reg2: u8 },
    SHR { reg: u8 },
    SHL { reg: u8 },
    SKRNE { reg1: u8, reg2: u8 },
    LOADI { addr: u16 },
    JUMPI { addr: u16 },
    RAND { reg: u8, val: u8 },
    DRAW { reg1: u8, reg2: u8, val: u8 },
    MOVED { reg: u8 },
    KEYD { reg: u8 },
    LOADD { reg: u8 },
    LOADS { reg: u8 },
    ADDI { reg: u8 },
    LDSPR { reg: u8 },
    BCD { reg: u8 },
    STOR { reg: u8 },
    READ { reg: u8 },
}

pub fn decode(val: u16) -> OpCode {
    let first_byte = val & 0xF000;
    let remainder = val & 0x0FFF;
    match first_byte {
        0 => {
            match remainder {
                0x00E0 => OpCode::CLR,
                0x00EE => OpCode::RET,
                _ => panic!("Invalid OpCode {:?}", val),
            }
        }
        1 => OpCode::JUMP { addr: remainder },
        2 => OpCode::CALL { addr: remainder },
        2 => OpCode::CALL { addr: remainder },
        _ => panic!("Invalid OpCode {:?}", val),
    }
}
