fn main() {
    
}

/* 
    Definitions

    Hz :     Times per second
    Opcode : Operation code; the portion of the binary
             instruction that specifies the operation to
             be performed
    Nibble : A 4-bit aggregation, with 2^4 (16) possible values;
             it can represent a single hex digit

*/

/* Memory is 4kB : 4096 bytes
   The index register addresses 12 bits,
   which is 4096 addresses.

   Each byte : 8 bits is an address

   Memory is write-able
*/

/*
    Font represents hexadecimal numbers 0 through F : 0 - 15
    Font data is stored in memory
    It is conventional to store the font in 050 - 09F in RAM
    
    width: 4px
    height: 5px

*/

/* 

    Display is 64px width x 32px tall
    Each pixel is a boolean value, or bit

    Redraw the screen when the emulator executes an instruction
    that modifies display data

    Sprites are drawn to the screen by reading bytes
    Each byte represents a horizontal line of the sprite
    0 is transparent, 1 will flip the pixels in the location

*/

/* 
    Stack : LIFO push/pop data
    The stack stores addresses : 16-bit (u16)

*/

/* 
    Timers : delay and sound timer
    Each timer is 1 byte in size
    
    While their value > 0,
    decrement by one 60 times per second : 60Hz

    The sound timer should beep while it is > 0

*/

/* 
    Keypad : 16 keys, hexadecimal : 0 - F
    Arranged on a 4x4 grid

    1 2 3 C
    4 5 6 D
    7 8 9 E
    A 0 B F

*/

/* 
    Fetch/decode/execute loop

    An emulator runs in an infinite loop doing these tasks:
    - Fetch
    - Decode
    - Execute

    Timing

    700Hz is a good standard

*/

/* 
    Fetch

    Read the instruction PC is pointing to in RAM

    An instruction is two bytes, so
    you will read two bytes from memory and
    combine them into one 16-bit instruction : u8 + u8 = u16

    Increment the PC by 2: skip 2 bytes forward


*/

/* 
    Decode

    Based on the first hexadecimal number : half-byte,
    many different else statements can occur
    - Try switch statements for this

    Mask off : binary AND the first number in the
    instruction, and have one case per number

    Different Cases ---

    x : Second nibble : looks up one of the 16 registers from v0
                        through VF
    
    Y : Third nibble : looks up one of the 16 registers from v0
                        through VF
    
    N : Fourth nibble : 4-bit number; a nibble

    NN : Third and fourth nibbles : the second byte

    NNN : Second, third, and fourth nibbles : 12-bit address

    How to decode ---

    Extract these values from the opcode before decoding,
    instead of doing it inside each instruction


*/

/* 
    Execute

    Do what the instruction instructs in each case

*/