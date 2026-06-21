// REMINDERS:
// 1 byte = 2 hex digit (ex. 0xAF), 8 bits
// 1 nibble = 1 hex digit, 4 bits
// Opcodes are 2 bytes, 16 bits (ex. 0x44BF)
// Register VALUES are 1 byte, 8 bits, but registers are addressed with 4 bits, 1 hex digit
// If function or variable called from main, set as public (pub) 

// Universal CPU loop:
// 1) Fetch, read 2 byte opcode that pc points at, and move pc to next instruction
// 2) Decode, pull opcode apart and determine function & registers it invokes
// 3) Execute instruction

// Import access to filesystem and IO error types
use std::fs;
use std::io;

// import opcode functions from opcodes.rs
#[path = "opcodes.rs"]
mod opcodes;

// define data stored within struct Chip8
pub struct Chip8 {
    pub memory: [u8; 4096],
    // 4096 bytes of RAM, each cell is a byte
    v: [u8; 16],
    // general registers, V0-VF
    // last register VF is a status flag (flag register), that records side effects or results of operation
    i: u16,
    // index register, holds addresses to give first express for a memory range
    // used to point to first address in memory holding spirtes, fonts
    // points at first address where data will be saved or loaded from registers
    pc: u16,
    // program counter, address of next instruction in memory
    stack: [u16; 16],
    // stack for storing return addresses, can handle 16 nested subroutines
    sp: u8,
    // stack pointer
    delay_timer: u8,
    sound_timer: u8,
    // 64 * 32 screen, each pixel on or off
    display: [bool; 64 * 32],
    // 16 keys, pressed or not
    keypad: [bool; 16],
    // when true, opcode debug messages are printed to the terminal
    debug: bool,
}

#[derive(Clone, Copy)]
pub struct Instruction {
    // Each opcode packs instruction in 4 hex digits (nibbles), which each hex digit is 4 bits
    // Ex. hex nibble 7 = 0111 in binary
    // Meaning library for opcode 0xWXYZ / 0xWXNN / 0xWNNN:
    // W, top nibble & instruction family byte
    // X, first register address
    // Y, second register address
    // Z or N 4-bit or single byte value
    // NN = YZ, an 8-bit value
    // NNN = XYZ, 12-bit address
    
    opcode: u16, // keep raw opcode for error messages
    instruction_family: u16, // top nibble, instruction family
    // Cast registers address x, y as usize
    x: usize, // usually first register address
    y: usize, // usually second register address
    n: u8, // value that goes into 8-bit register, so u8
    nn: u8, // value that goes into 8-bit register, so u8
    nnn: u16, // 12-bit address
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

// impl gives functionality to Chip8 struct, separating data and function

impl Chip8 {
    // function that builds and returns Chip8 virtual machine 
    pub fn new() -> Self {
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
            // debug messages on by default
            debug: true,
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

    // turn opcode debug messages on or off
    pub fn set_debug(&mut self, enabled: bool) {
        self.debug = enabled;
    }

    // print a debug message only when debug mode is enabled
    fn debug_log(&self, message: &str) {
        if self.debug {
            println!("{message}");
        }
    }

    // Use &mut self to "borrow" (take ownership) of the machine and change it
    // Don't bring in a copy of the machine, borrow it with &
    // In this case, we change the program counter
    // Fetch, read 2 byte opcode that pc points at, and move pc to next instruction
    pub fn fetch(&mut self) -> u16 {
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

    // Decode, pull opcode apart and determine function & registers it invokes
    // Returns struct of split apart op-code Instruction
    pub fn decode(&self, opcode: u16) -> Instruction {
        // pull apart opcode into nibbles and build Instruction struct to return 

        Instruction {
            opcode,
            instruction_family: (opcode & 0xF000) >> 12,
            x:     ((opcode & 0x0F00) >> 8) as usize,
            y:     ((opcode & 0x00F0) >> 4) as usize,
            n:     (opcode & 0x000F) as u8,
            nn:    (opcode & 0x00FF) as u8,
            nnn:   opcode & 0x0FFF,
        }
    }  

    // pulls nibbles from decode via returned Instruction struct, and executes instructions
    // takes in Instruction struct
    pub fn execute(&mut self, instr: Instruction) {
        match instr.instruction_family {
            0x0 => match instr.nn {
                0xEE => self.op_return(),
                0xE0 => {},
                _ => println!("Unknown opcode: {:#06X}", instr.opcode),

            }
            0x1 => self.op_jump(instr),
            0x2 => self.op_call(instr),
            0x3 => self.op_skip_eq_nn(instr),
            0x4 => self.op_skip_ne_nn(instr),
            0x5 => self.op_skip_eq_reg(instr),
            0x6 => self.op_set(instr),
            0x7 => self.op_add(instr),
            0x8 => match instr.n {
                0x0 => self.op_set_reg(instr),
                0x1 => self.op_or(instr),
                0x2 => self.op_and(instr),
                0x3 => self.op_xor(instr),
                0x4 => self.op_add_reg(instr),
                0x5 => self.op_sub_reg(instr),
                0x6 => self.op_shift_right(instr),
                0x7 => self.op_sub_reverse(instr),
                0xE => self.op_shift_left(instr),
                _ => println!("Unknown opcode: {:#06X}", instr.opcode),
            }
            0x9 => self.op_skip_ne_reg(instr),
            0xA => self.op_set_index(instr),
            0xB => self.op_jump_add_v0(instr),
            0xD => self.op_display_sprite(instr),
            0xE => match instr.n {
                0xE => self.op_skip_keypress(instr),
                0x1 => self.op_skip_nokeypress(instr),
                _ => println!("Unknown opcode: {:#06X}", instr.opcode),
            }
            _ => println!("Unknown opcode: {:#06X}", instr.opcode),
        }
    }

    // Cycles through fetch, decode, and execute
    pub fn cycle(&mut self) {
        // Pulls instruction from memory using the address set in program counter (pc)
        // After fetch, increment pc + 2 to get to next instruction address
        let opcode = self.fetch();
        // Breaks up opcode into nibbles / bytes to execute instructions on, and stores in instr struct
        let instr = self.decode(opcode);
        // Executes instruction by matching opcode to function and changing memory
        self.execute(instr);
    }

    // Load a binary rom, and return an IO error on failure
    pub fn load_rom(&mut self, path: &str) -> Result<(), io::Error> {
        // reads the file, and ? returns early if failure
        // rom is Vec<u8>, a growable array of bytes
        let rom = fs::read(path)?;

        // check to confirm that rom isn't larger than program space
        if rom.len() > (0x1000 - 0x200) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "ROM larger than avaliable program space in memory"));
        }

        // writes rom into memory
        for (offset, &byte) in rom.iter().enumerate() {
            self.memory[0x200 + offset] = byte;
        }
        Ok(())
    }

}