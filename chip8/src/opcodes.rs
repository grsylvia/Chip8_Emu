use super::{Chip8, Instruction};

// pub(super) makes function public for parent module (cpu.rs)

impl Chip8 {

    // 0x00E0

    // 0x00EE

    // 0x1NNN
    // Jump means set pc to the address given by hex number 0xnnn
    pub(super) fn op_jump(&mut self, instr: Instruction) {
        println!("JUMP to {:#05X}", instr.nnn);
        self.pc = instr.nnn;
        println!("Next instruction address in memory is now {:#05X}", self.pc);
    }

    // 0x2NNN

    // 0x3XNN
    // Skips next instruction if register value equals nn value
    // Compares a variable to a constant and helps implement if / else statements
    pub(super) fn op_skip_eq_nn(&mut self, instr: Instruction) {
        if self.v[instr.x] == instr.nn {
            self.pc += 2;
        }
    }

    // 0x4XNN
    // Skips next instruction if register value does not equals nn value
    // Compares a variable to a constant and helps implement if / else statements
    pub(super) fn op_skip_ne_nn(&mut self, instr: Instruction) {
        if self.v[instr.x] != instr.nn {
            self.pc += 2;
        }
    }

    // 0x5XYN
    // Skips next instruction if x register equals y register in opcode
    // Compares two variables and helps implement if / else statements
    pub(super) fn op_skip_eq_reg(&mut self, instr: Instruction) {
        if self.v[instr.x] == self.v[instr.y] {
            self.pc += 2;
        }
    }

    // 0x6XNN
    // Sets register in opcode to nn value provided
    pub(super) fn op_set(&mut self, instr: Instruction) {
        self.v[instr.x] = instr.nn;
        println!("OP_SET: V{:X} = {:#04X}", instr.x, self.v[instr.x]);
    }

    // 0x7XNN
    // Adds 8-bit value 0xNN to the register VA (register 10)
    pub(super) fn op_add(&mut self, instr: Instruction) {
        println!("OP_ADD: V{:X} += {:#04X}", instr.x, instr.nn);
        // Overflow wrapping is a attribute of Chip8, need to override Rust errors to allow
        self.v[instr.x] = self.v[instr.x].wrapping_add(instr.nn);
        println!("OP_ADD: V{:X} is now {:#04X}", instr.x, self.v[instr.x]);
    }

    // 0x8XY0
    pub(super) fn op_set_reg(&mut self, instr: Instruction) {
        println!("OP_SET_REG: V{:X} is {:#04X}", instr.x, self.v[instr.x]);
        println!("OP_SET_REG: V{:X} is {:#04X}", instr.y, self.v[instr.y]);
        self.v[instr.x] = self.v[instr.y];
        println!("OP_SET_REG: V{:X} is now {:#04X}", instr.x, self.v[instr.x]);
    }

    // 0x8XY1
    // Performs bitwise OR operation, stores result into Vx
    pub(super) fn op_or(&mut self, instr: Instruction) {
        self.v[instr.x] = self.v[instr.x] | self.v[instr.y];
        println!("OP_OR: V{:X} is {:#04X}", instr.x, self.v[instr.x]);
    }

    // 0x8XY2
    // Performs bitwise AND operation, stores result into Vx
    pub(super) fn op_and(&mut self, instr: Instruction) {
        self.v[instr.x] = self.v[instr.x] & self.v[instr.y];
        println!("OP_AND: V{:X} is {:#04X}", instr.x, self.v[instr.x]);
    }

    // 0x8XY3
    // Performs bitwise XOR operation, stores result into Vx
    pub(super) fn op_xor(&mut self, instr: Instruction) {
        self.v[instr.x] = self.v[instr.x] ^ self.v[instr.y];
        println!("OP_AND: V{:X} is {:#04X}", instr.x, self.v[instr.x]);
    }

    // 0x8XY4

    // 0x8XY5

    // 0x8XY6

    // 0x8XY7

    // 0x8XYE

    // 0x9XY0
    // Skips next instruction if x register equals y register in opcode
    // Compares two variables and helps implement if / else statements
    pub(super) fn op_skip_ne_reg(&mut self, instr: Instruction) {
        if self.v[instr.x] != self.v[instr.y] {
            self.pc += 2;
        }
    }

    // 0xANNN
    // Sets index register in opcode to nnn value provided
    pub(super) fn op_set_index(&mut self, instr: Instruction) {
        self.i = instr.nnn;
        println!("OP_SET_INDEX: Index = {:#05X}", self.i);
    }

    // 0xBNNN

    // 0xCXNN

    // 0xDXYN

    // 0xEX9E

    // 0xEXA1

    // 0xFX07

    // 0xFX0A

    // 0xFX15

    // 0xFX18

    // 0xFX1E

    // 0xFX29

    // 0xFX33

    // 0xFX55

    // 0xFX65
}
