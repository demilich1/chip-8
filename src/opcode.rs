pub enum OpCode {
    SYS,
    CLR,
    RET,
    JUMP,
    CALL,
    SKE,
    SKNE,
    SKRE,
    LOAD,
    ADD,
    MOVE,
    OR,
    AND,
    XOR,
    ADDR,
    SUB,
    SHR,
    SHL,
    SKRNE,
    LOADI,
    JUMPI,
    RAND,
    DRAW,
    MOVED,
    KEYD,
    LOADD,
    LOADS,
    ADDI,
    LDSPR,
    BCD,
    STOR,
    READ
}

impl From<u16> for OpCode {
    fn from(val: u16) -> Self {
        OpCode::SYS
    }
}
