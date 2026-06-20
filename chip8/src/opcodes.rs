use super::{Chip8, Instruction};

impl Chip8 {
    // pulls nibbles from decode via returned Instruction struct, and executes instructions
    // takes in Instruction struct
    pub fn execute(&mut self, instr: Instruction) {
        match instr.instruction_family {
            0x1 => self.op_jump(instr),
            0x3 => self.op_skip_eq_nn(instr),
            0x4 => self.op_skip_ne_nn(instr),
            0x5 => self.op_skip_eq_reg(instr),
            0x6 => self.op_set(instr),
            0x7 => self.op_add(instr),
            0x9 => self.op_skip_ne_reg(instr),
            0xA => self.op_set_index(instr),
            _ => println!("Unknown opcode: {:#06X}", instr.opcode),
        }
    }

    // Jump means set pc to the address given by hex number 0xnnn
    fn op_jump(&mut self, instr: Instruction) {
        println!("JUMP to {:#05X}", instr.nnn);
        self.pc = instr.nnn;
        println!("Next instruction address in memory is now {:#05X}", self.pc);
    }

    // 0x3XNN
    // Skips next instruction if register value equals nn value
    // Compares a variable to a constant and helps implement if / else statements
    fn op_skip_eq_nn(&mut self, instr: Instruction) {
        if self.v[instr.x] == instr.nn {
            self.pc += 2;
        }
    }

    // 0x4XNN
    // Skips next instruction if register value does not equals nn value
    // Compares a variable to a constant and helps implement if / else statements
    fn op_skip_ne_nn(&mut self, instr: Instruction) {
        if self.v[instr.x] != instr.nn {
            self.pc += 2;
        }
    }

    // 0x5XYN
    // Skips next instruction if x register equals y register in opcode
    // Compares two variables and helps implement if / else statements
    fn op_skip_eq_reg(&mut self, instr: Instruction) {
        if self.v[instr.x] == self.v[instr.y] {
            self.pc += 2;
        }
    }

    // 0x9XY0
    // Skips next instruction if x register equals y register in opcode
    // Compares two variables and helps implement if / else statements
    fn op_skip_ne_reg(&mut self, instr: Instruction) {
        if self.v[instr.x] != self.v[instr.y] {
            self.pc += 2;
        }
    }

    // Sets register in opcode to nn value provided
    fn op_set(&mut self, instr: Instruction) {
        self.v[instr.x] = instr.nn;
        println!("SET: V{:X} = {:#04X}", instr.x, self.v[instr.x]);
    }

    // Adds 8-bit value 0xNN to the register VA (register 10)
    fn op_add(&mut self, instr: Instruction) {
        println!("ADD: V{:X} += {:#04X}", instr.x, instr.nn);
        // Overflow wrapping is a attribute of Chip8, need to override Rust errors to allow
        self.v[instr.x] = self.v[instr.x].wrapping_add(instr.nn);
        println!("ADD: V{:X} is now {:#04X}", instr.x, self.v[instr.x]);
    }

    // Sets index register in opcode to nnn value provided
    fn op_set_index(&mut self, instr: Instruction) {
        self.i = instr.nnn;
        println!("SET INDEX = {:#05X}", self.i);
    }
}
