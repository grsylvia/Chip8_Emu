fn main() {
    // calls function associated with type Chip8, so calls a constructor
    let chip8 = Chip8::new();

    // print out the bytes associated with the digit 0 
    println!("Font bytes for '0' starting a 0x50 (beginning of fonts in mem):");
    for offset in 0..5 {
        // :, introduces format spec
        // #, add the 0x prefix
        // 04, pad to 4 characters wide
        // X, set format as hex
        // pulls each byte in font memory range for digit 0
        // NOTE: memory is addressed in bytes, and returns / stores bytes
        println!("memory[{:#04X}] = {:#04X}", 0x50 + offset, chip8.memory[0x50 + offset])

    

        
    }
}

struct Chip8 {
    memory: [u8; 4096], // 4096 bytes of RAM, each cell is a byte
    v: [u8; 16], // general registers, V0-VF
    i: u16, // index register, holds addresses
    pc: u16, // program counter, address of next instruction
    stack: [u16; 16], // stack, saves return addresses
    sp: u8, // stack pointer
    delay_timer: u8,
    sound_timer: u8,
    display: [bool; 64 * 32], // 64 * 32 screen, each pixel on or off
    keypad: [bool; 16] // 16 keys, pressed or not
}

const FONT_SET: [u8; 80] = [
    // loads in the display digits 0-F
    // decode each row into binary to see the digit literally drawn out in binary
    // first row, 0 digit
    // Byte    Binary      Pixels (1 = on)
    // 0xF0    1111 0000   ████
    // 0x90    1001 0000   █  █
    // 0x90    1001 0000   █  █
    // 0x90    1001 0000   █  █
    // 0xF0    1111 0000   ████
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Chip8 {
    fn new() -> Self { // function that returns Chip8
        let mut chip8 = Chip8 { // mutable, rust vars are read-only by default
            // clear all memory and registers
            memory: [0; 4096], 
            v: [0; 16], 
            i: 0,
            // start program counter at beginning of program space
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            // clear screen
            display: [false; 64 * 32],
            keypad: [false; 16],
        };

        // after initializing, load font set into memory starting at 0x50
        // offset each font pixel in memory starting from 0x50 using pointers
        // ex. first pixel of 0 loaded into 0x50
        for (offset, &byte) in FONT_SET.iter().enumerate() {
            chip8.memory[0x50 + offset] = byte
        }

        // let chip8 be the return value of new()
        chip8
    }
}