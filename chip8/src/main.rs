// REMINDERS:
// 1 byte = 2 hex digit (ex. 0xAF), 8 bits
// 1 nibble = 1 hex digit, 4 bits
// Opcodes are 2 bytes, 16 bits (ex. 0x44BF)
// Register VALUES are 1 byte, 8 bits, but registers are addressed with 4 bits, 1 hex digit


fn main() {
    // calls function associated with type Chip8, so calls a constructor
    let mut chip8 = Chip8::new();

    // Universal CPU loop:
    // 1) Fetch, read 2 byte opcode that pc points at, and move pc to next instruction
    // 2) Decode, pull opcode apart and determine function & registers it invokes
    // 3) Execute instruction

    // place 2 instructions into memory
    // load first instruction 0x7A15, using 2 bytes and 2 memory addresses
    chip8.memory[0x200] = 0x7A;
    chip8.memory[0x201] = 0x15;
    // load second instruction 0x1234
    chip8.memory[0x202] = 0x12;
    chip8.memory[0x203] = 0x34;

    // run fetch-decode cycle twice
    for _ in 0..2 {
        // starting at pc = 0x200, pull instruction in addresses 0x200 and 0x201
        // then, increment pc to start again at 0x202 and pull the next instruction
        let opcode = chip8.fetch();
        println!("{:#06X}", opcode);
        chip8.decode(opcode);
    }
}

struct Chip8 {
    memory: [u8; 4096], // 4096 bytes of RAM, each cell is a byte
    v: [u8; 16], // general registers, V0-VF
    i: u16, // index register, holds addresses
    pc: u16, // program counter, address of next instruction in memory
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

    // Use %mut self to "borrow" (take ownership) of the machine and change it
    // In this case, we change the program counter
    // Don't bring in a copy of the machine, borrow it with &
    fn fetch(&mut self) -> u16 {

        // Fetch, read 2 byte opcode that pc points at, and move pc to next instruction

        // Memory is stored one byte at a time, but an opcode is two bytes
        // You can ONLY index memory as an integer literal (ex. 0x50) or as a usize
        // Store each opcode byte into a u8
        let high_byte = self.memory[self.pc as usize] as u16;
        let low_byte = self.memory[(self.pc + 1) as usize] as u16;

        // Combine high and low bytes into a single word (u16, two bytes long)
        // use mut to change variable later
        let mut opcode = high_byte << 8;
        opcode = opcode | low_byte;

        // move pc up 2 memory address for point at the memory address of the next instruction
        self.pc += 2;

        // Use no semicolons to have opcode return
        opcode
    }

    fn decode(&mut self, opcode: u16) {
        // Decode, pull opcode apart and determine function & registers it invokes

        // Each opcode packs meaning in 4 hex digits (nibbles), which each hex digit is a byte
        // Ex. hex nibble 7 = 0111 in binary
        // Meaning library for opcode 0xWXYZ / 0xWXNN / 0xWNNN:
        // W, top nibble & instruction family byte
        // X, first register address
        // Y, second register address
        // Z or N 4-bit or single byte value
        // NN = YZ, an 8-bit value
        // NNN = XYZ, 12-bit address
        
        // EXAMPLE
        // mask with & to pull out nibble, and then use >> to slid down to ones place
        // acts as a boolean operation that pulls out digit aligned with F
        // let mut example = (0x7A15 & 0xF000);
        //println!("{:#04X}", example);

        // shift down 12 bits to get to 4 bits in hex (3 digits, 4 bits each, so 3 * 4 = 12)
        // example = example >> 12;
        // println!("{:#01X}", example);

        // pull apart opcode into nibbles

        let opcode_group = (opcode & 0xF000) >> 12; // instruction family
        // NOTE: register VALUES are 1 byte, 8 bits, but registers are addressed with 4 bits, 1 hex digit
        // Cast registers address x, y as usize to address register
        let x = ((opcode & 0x0F00) >> 8) as usize; // usually first register address
        let y = ((opcode & 0x00F0) >> 4) as usize; // usually second register address
        let n = (opcode & 0x000F) as u8; // value that goes into 8-bit register, so u8
        let nn = (opcode & 0x00FF) as u8; // value that goes into 8-bit register, so u8
        let nnn = opcode & 0x0FFF; // 12-bit address

        // Match each opcode to a specific instruction

        match opcode_group {
            // Add means add 0xNN to the register VA (register 10)
            0x7 => {
                println!("ADD: V{:X} += {:#04X}", x, nn)

            }
            // Jump means set pc to the address given by hex number 0xnnn
            0x1 => {
                println!("JUMP to {:#05X}", nnn)
            }
            _ => {
                println!("Unknown opcode: {:#06X}", opcode);
            }
        }
    }  
}