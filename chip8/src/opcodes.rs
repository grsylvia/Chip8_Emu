use super::{Chip8, Instruction};

impl Chip8 {
    // Jump means set pc to the address given by hex number 0xnnn
    pub(super) fn op_jump(&mut self, instr: Instruction) {
        println!("JUMP to {:#05X}", instr.nnn);
        self.pc = instr.nnn;
        println!("Next instruction address in memory is now {:#05X}", self.pc);
    }

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

    // 0x9XY0
    // Skips next instruction if x register equals y register in opcode
    // Compares two variables and helps implement if / else statements
    pub(super) fn op_skip_ne_reg(&mut self, instr: Instruction) {
        if self.v[instr.x] != self.v[instr.y] {
            self.pc += 2;
        }
    }

    // Sets register in opcode to nn value provided
    pub(super) fn op_set(&mut self, instr: Instruction) {
        self.v[instr.x] = instr.nn;
        println!("SET: V{:X} = {:#04X}", instr.x, self.v[instr.x]);
    }

    // Adds 8-bit value 0xNN to the register VA (register 10)
    pub(super) fn op_add(&mut self, instr: Instruction) {
        println!("ADD: V{:X} += {:#04X}", instr.x, instr.nn);
        // Overflow wrapping is a attribute of Chip8, need to override Rust errors to allow
        self.v[instr.x] = self.v[instr.x].wrapping_add(instr.nn);
        println!("ADD: V{:X} is now {:#04X}", instr.x, self.v[instr.x]);
    }

    // Sets index register in opcode to nnn value provided
    pub(super) fn op_set_index(&mut self, instr: Instruction) {
        self.i = instr.nnn;
        println!("SET INDEX = {:#05X}", self.i);
    }
}
