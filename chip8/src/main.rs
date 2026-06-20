fn main() {
    println!("Hello, world!");
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