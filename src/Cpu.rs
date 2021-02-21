
pub struct Cpu {
    // index register : two bytes, 4 hex digits
    pub i: u16,

    // program counter : two bytes, 4 hex digits
    pub pc: u16,

    // memory : u8 : 1 byte addresses, 4096 addresses
    pub memory: [u8; 4096],
    
    // stack : u16: 16-bit addresses, 16 addresses
    pub stack: [u16; 16],

    // registers : u8 8-bit addresses, 16 addresses
    pub v: [u8; 16],

    // stack pointer : u8 : 1 byte
    pub sp: u8,

    // delay timer : u8 : 1 byte
    pub dt: u8,
}