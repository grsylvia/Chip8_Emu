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
    chip8.load_rom("test.ch8").expect("failed to load ROM");

    for _ in 0..3 {
        chip8.cycle();
    }
}
