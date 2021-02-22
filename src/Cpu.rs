
/*
    The CHIP8 system has 35 opcodes 
    which are each 16 bits : 2 bytes long
*/
type opcode = u16;

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

/* 
   Memory is 4kB : 4096 bytes
   The index register addresses 12 bits,
   which is 4096 addresses.

   Each byte : 8 bits is an address

   Memory is write-able

   The CHIP-8 program should be loaded into the machine
   starting at the address 200 : 0x200 - 0xFFF

   0x000 - 0x1FF Chip 8 Interpreter
   0x050 - 0x0A0 4x5 pixel font set
   0x200 - 0xFFF program ROM and work RAM
*/

impl Cpu {

    pub fn new() -> Self {    

        Cpu {
            i: 0,
            pc: 0x200,
            memory: [0u8; 4096],
            stack: [0u16; 16],
            v: [0u8; 16],
            sp: 0,
            dt: 0,
        }

    }

    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[0x200 + i] = byte;
                print!("{:x} ", self.memory[0x200 + i]);
            } else {
                break
            }
        }
    }
 
    
}