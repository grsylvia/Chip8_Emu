// REMINDERS:
// 1 byte = 2 hex digit (ex. 0xAF), 8 bits
// 1 nibble = 1 hex digit, 4 bits
// Opcodes are 2 bytes, 16 bits (ex. 0x44BF)
// Register VALUES are 1 byte, 8 bits, but registers are addressed with 4 bits, 1 hex digit

// loads cpu module
mod cpu;
// lets you write Chip8 instead of cpu::Chip8
use cpu::Chip8;

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
        chip8.cycle();
    }
}
