use super::{Chip8, Instruction};

// pub(super) makes function public for parent module (cpu.rs)

impl Chip8 {

    // 0x00E0

    // 0x00EE

    // 0x1NNN
    // Set program counter (pc) to the address given by hex number 0xnnn
    pub(super) fn op_jump(&mut self, instr: Instruction) {
        self.pc = instr.nnn;
        self.debug_log(&format!("[1NNN] JUMP -> pc = {:#05X}", self.pc));
    }

    // 0x2NNN

    // 0x3XNN
    // Skips next instruction if register value equals nn value
    // Compares a variable to a constant and helps implement if / else statements
    pub(super) fn op_skip_eq_nn(&mut self, instr: Instruction) {
        let skip = self.v[instr.x] == instr.nn;
        if skip {
            self.pc += 2;
        }
        self.debug_log(&format!(
            "[3XNN] SKIP if V{:X}({:#04X}) == {:#04X}? {}",
            instr.x, self.v[instr.x], instr.nn, skip
        ));
    }

    // 0x4XNN
    // Skips next instruction if register value does not equals nn value
    // Compares a variable to a constant and helps implement if / else statements
    pub(super) fn op_skip_ne_nn(&mut self, instr: Instruction) {
        let skip = self.v[instr.x] != instr.nn;
        if skip {
            self.pc += 2;
        }
        self.debug_log(&format!(
            "[4XNN] SKIP if V{:X}({:#04X}) != {:#04X}? {}",
            instr.x, self.v[instr.x], instr.nn, skip
        ));
    }

    // 0x5XYN
    // Skips next instruction if x register equals y register in opcode
    // Compares two variables and helps implement if / else statements
    pub(super) fn op_skip_eq_reg(&mut self, instr: Instruction) {
        let skip = self.v[instr.x] == self.v[instr.y];
        if skip {
            self.pc += 2;
        }
        self.debug_log(&format!(
            "[5XY0] SKIP if V{:X}({:#04X}) == V{:X}({:#04X})? {}",
            instr.x, self.v[instr.x], instr.y, self.v[instr.y], skip
        ));
    }

    // 0x6XNN
    // Sets register in opcode to nn value provided
    pub(super) fn op_set(&mut self, instr: Instruction) {
        self.v[instr.x] = instr.nn;
        self.debug_log(&format!("[6XNN] SET V{:X} = {:#04X}", instr.x, self.v[instr.x]));
    }

    // 0x7XNN
    // Adds 8-bit value 0xNN to the register VA (register 10)
    pub(super) fn op_add(&mut self, instr: Instruction) {
        // Overflow wrapping is a attribute of Chip8, need to override Rust errors to allow
        self.v[instr.x] = self.v[instr.x].wrapping_add(instr.nn);
        self.debug_log(&format!(
            "[7XNN] ADD V{:X} += {:#04X} -> {:#04X}",
            instr.x, instr.nn, self.v[instr.x]
        ));
    }

    // 0x8XY0
    // Sets value of register x to value of register y
    pub(super) fn op_set_reg(&mut self, instr: Instruction) {
        self.v[instr.x] = self.v[instr.y];
        self.debug_log(&format!(
            "[8XY0] SET V{:X} = V{:X} ({:#04X})",
            instr.x, instr.y, self.v[instr.x]
        ));
    }

    // 0x8XY1
    // Performs bitwise OR operation, stores result into Vx
    pub(super) fn op_or(&mut self, instr: Instruction) {
        self.v[instr.x] = self.v[instr.x] | self.v[instr.y];
        self.debug_log(&format!(
            "[8XY1] OR  V{:X} |= V{:X} -> {:#04X}",
            instr.x, instr.y, self.v[instr.x]
        ));
    }

    // 0x8XY2
    // Performs bitwise AND operation, stores result into Vx
    pub(super) fn op_and(&mut self, instr: Instruction) {
        self.v[instr.x] = self.v[instr.x] & self.v[instr.y];
        self.debug_log(&format!(
            "[8XY2] AND V{:X} &= V{:X} -> {:#04X}",
            instr.x, instr.y, self.v[instr.x]
        ));
    }

    // 0x8XY3
    // Performs bitwise XOR operation, stores result into Vx
    pub(super) fn op_xor(&mut self, instr: Instruction) {
        self.v[instr.x] = self.v[instr.x] ^ self.v[instr.y];
        self.debug_log(&format!(
            "[8XY3] XOR V{:X} ^= V{:X} -> {:#04X}",
            instr.x, instr.y, self.v[instr.x]
        ));
    }

    // 0x8XY4
    pub(super) fn op_add_reg(&mut self, instr: Instruction) {
        // checks if addition of two u8 values overflows, assigns 1 (TRUE) if overflow
        let overflow = (self.v[instr.x] as u16) + (self.v[instr.y] as u16) > 0xFF;
        // adds values in VX and VY, with wrapping
        self.v[instr.x] = self.v[instr.x].wrapping_add(self.v[instr.y]);
        // sets flag register to 1 if overflow
        self.v[0xF] = overflow as u8;

        self.debug_log(&format!(
            "[8XY4] ADD V{:X} += V{:X} -> {:#04X} (VF={})",
            instr.x, instr.y, self.v[instr.x], self.v[0xF]));
    }

    // 0x8XY5
    pub(super) fn op_sub_reg(&mut self, instr: Instruction) {
        // checks if borrow is required (VX >= VY)
        let no_borrow = self.v[instr.x] >= (self.v[instr.y]);
        // subtracts values in VX and VY, wrapping backwards if borrow is required
        self.v[instr.x] = self.v[instr.x].wrapping_sub(self.v[instr.y]);
        // sets borrow register to 1 if no borrow
        self.v[0xF] = no_borrow as u8;
    }

    // 0x8XY6
    pub(super) fn op_shift_right(&mut self, instr: Instruction) {
        // pulls the least significant bit from the value in register x
        let lsb = self.v[instr.x] & 1;
        // if you shift a binary number to the right by 1 bit, it divides the number by two
        // shift over by one and store the new values into register x
        self.v[instr.x] = self.v[instr.x] >> 1;
        // store the least significant bit (right-most bit) into flag register
        self.v[0xF] = lsb;
    }

    // 0x8XY7
    pub(super) fn op_sub_reverse() {

    }

    // 0x8XYE
    pub(super) fn op_shift_left() {
        
    }

    // 0x9XY0
    // Skips next instruction if x register equals y register in opcode
    // Compares two variables and helps implement if / else statements
    pub(super) fn op_skip_ne_reg(&mut self, instr: Instruction) {
        let skip = self.v[instr.x] != self.v[instr.y];
        if skip {
            self.pc += 2;
        }
        self.debug_log(&format!(
            "[9XY0] SKIP if V{:X}({:#04X}) != V{:X}({:#04X})? {}",
            instr.x, self.v[instr.x], instr.y, self.v[instr.y], skip
        ));
    }

    // 0xANNN
    // Sets index register in opcode to nnn value provided
    pub(super) fn op_set_index(&mut self, instr: Instruction) {
        self.i = instr.nnn;
        self.debug_log(&format!("[ANNN] SET I = {:#05X}", self.i));
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
