// REMINDERS:
// 1 byte = 2 hex digit (ex. 0xAF), 8 bits
// 1 nibble = 1 hex digit, 4 bits
// Opcodes are 2 bytes, 16 bits (ex. 0x44BF)
// Register VALUES are 1 byte, 8 bits, but registers are addressed with 4 bits, 1 hex digit

const FRAMES_PER_SECOND: u16 = 60;
const CPU_HZ: u16 = 540;
const CYCLES_PER_FRAME: u16 = CPU_HZ / 60;

// loads cpu module
mod cpu;
// lets you write Chip8 instead of cpu::Chip8
use cpu::Chip8;

use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    // calls function associated with type Chip8, so calls a constructor
    let mut chip8 = Chip8::new();
    chip8.load_rom("test_roms/1-chip8-logo.ch8").expect("failed to load ROM");

    loop {
        let time_interval = Duration::from_secs_f64(1.0 / FRAMES_PER_SECOND as f64);
        // doesn't need mut, it's reassigned every frame
        let next_tick = Instant::now() + time_interval;

        for cycle_count in 0..CYCLES_PER_FRAME {
            chip8.cycle();
        }

        chip8.draw_display();
        if Instant::now() < next_tick {
                sleep (next_tick - Instant::now());
        }
    }
}
