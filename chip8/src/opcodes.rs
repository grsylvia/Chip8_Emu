use super::{Chip8, Instruction};
use rand::RngExt;

// pub(super) makes function public for parent module (cpu.rs)

impl Chip8 {

    // 0x00E0
    // Clears display by setting all boolean values in 64 * 32 display to false
    pub(super) fn op_clear_display(&mut self) {
        self.display = [false; 64 * 32];
        self.debug_log(&format!("[00E0] CLS"));
    }

    // 0x00EE
    // return from subroutine
    pub(super) fn op_return(&mut self) {
        // go back to the top of stack for the latest return address stored
        self.sp -= 1;
        // set program counter to latest return address stored
        self.pc = self.stack[self.sp as usize];
        // next fetch resumes at the instruction following the original call
    }

    // 0x1NNN
    // Set program counter (pc) to the address given by hex number 0xnnn
    pub(super) fn op_jump(&mut self, instr: Instruction) {
        self.pc = instr.nnn;
        self.debug_log(&format!("[1NNN] JUMP -> pc = {:#05X}", self.pc));
    }

    // 0x2NNN
    // Subroutines are reusable blocks of code you can call
    // To call a subroutine, you must jump to the address of its first instruction
    // ALSO, you must record the address to jump back to after the subroutine is finished
    // Subroutines can call other subroutines, so you can have nested return addresses to remember
    // Finish C -> Back to return address in B -> Finish B -> Back to return address in A
    // Stacks are Last In, First Out (LIFO), you push a return address on top, and pop the top one off
    // Stack pointer (sp) holds the address of next free slot in the stack
    // op_call calls a subroutine by storing the return address (pc, next instruction in memory)
    pub(super) fn op_call(&mut self, instr: Instruction) {
        // store return address before jumping to subroutine
        self.stack[self.sp as usize] = self.pc;
        // increment sp to update to next free slot in stack
        self.sp += 1;
        // jump to subroutine
        self.pc = instr.nnn;
    }

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
    // Adds value of register y to register x, sets flag register to 1 if the addition overflows
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
    // Subtracts value of register y from register x, sets flag register to 1 if there is no borrow
    pub(super) fn op_sub_reg(&mut self, instr: Instruction) {
        // checks if borrow is required (VX >= VY)
        let no_borrow = self.v[instr.x] >= (self.v[instr.y]);
        // subtracts values in VX and VY, wrapping backwards if borrow is required
        self.v[instr.x] = self.v[instr.x].wrapping_sub(self.v[instr.y]);
        // sets borrow register to 1 if no borrow
        self.v[0xF] = no_borrow as u8;

        self.debug_log(&format!(
            "[8XY5] SUB V{:X} -= V{:X} -> {:#04X} (VF={})",
            instr.x, instr.y, self.v[instr.x], self.v[0xF]));
    }

    // 0x8XY6
    // Shifts value of register x right by 1 (divides by two), stores the lost least significant bit in the flag register
    pub(super) fn op_shift_right(&mut self, instr: Instruction) {
        // pulls the least significant bit from the value in register x
        let lsb = self.v[instr.x] & 1;
        // if you shift a binary number to the right by 1 bit, it divides the number by two
        // shift over right by one and store the new values into register x
        self.v[instr.x] = self.v[instr.x] >> 1;
        // store the least significant bit (right-most bit) into flag register
        self.v[0xF] = lsb;

        self.debug_log(&format!(
            "[8XY6] SHR V{:X} >> 1 -> {:#04X} (VF={})",
            instr.x, self.v[instr.x], self.v[0xF]));
    }

    // 0x8XY7
    // Subtracts value of register x from register y and stores the result in register x, sets flag register to 1 if there is no borrow
    pub(super) fn op_sub_reverse(&mut self, instr: Instruction) {
        // checks if no borrow is required (VY >= VX)
        let no_borrow = self.v[instr.y] >= (self.v[instr.x]);
        // subtracts values in VY and VX, wrapping backwards if borrow is required
        self.v[instr.x] = self.v[instr.y].wrapping_sub(self.v[instr.x]);
        // sets borrow register to 1 if no borrow
        self.v[0xF] = no_borrow as u8;

        self.debug_log(&format!(
            "[8XY7] SUBN V{:X} = V{:X} - V{:X} -> {:#04X} (VF={})",
            instr.x, instr.y, instr.x, self.v[instr.x], self.v[0xF]));
    }

    // 0x8XYE
    // Shifts value of register x left by 1 (multiplies by two), stores the lost most significant bit in the flag register
    pub(super) fn op_shift_left(&mut self, instr: Instruction) {
        // shifts register value (type u8) down 7 bits to the right, pulling the msb
        let msb = self.v[instr.x] >> 7;
        // if you shift a binary number to the left by 1 bit, it multiples the number by two
        // shift over left by one and store the new values into register x
        self.v[instr.x] = self.v[instr.x] << 1;
        // store the most significant bit (left-most bit) into flag register
        self.v[0xF] = msb;

        self.debug_log(&format!(
            "[8XYE] SHL V{:X} << 1 -> {:#04X} (VF={})",
            instr.x, self.v[instr.x], self.v[0xF]));
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
    // Set program counter to nnn plus value in register V0
    pub(super) fn op_jump_add_v0(&mut self, instr: Instruction) {
        self.pc += (instr.nnn) + (self.v[0x0] as u16);
    }

    // 0xCXNN
    // Generates a random number from 0 to 255, and ANDs that number with nn
    pub(super) fn op_add_rand(&mut self, instr: Instruction) {
        let mut rng = rand::rng();
        // (0..=255) is an inclusive range
        let random: u8 = rng.random_range(0..=255);
        self.v[instr.x] = random & instr.nn;
    }

    // 0xDXYN
    // Reads n bytes from memory, starting at the index register i
    // Display the bytes from memory as sprites on screen starting at (Vx, Vy)
    pub(super) fn op_display_sprite(&mut self, instr: Instruction) {
        let mut sprite: Vec<u8> = Vec::new();
        for offset in 0..instr.n {
            sprite.push(self.memory[(self.i + (offset as u16)) as usize]);
        }

        // Set initial x and y coordinates on display per Vx and Vy values
        // Use mod 64, 32 to have x-coordinates wrap around display
        let initial_x_coord = self.v[instr.x] % 64;
        let initial_y_coord = self.v[instr.y] % 32;
        // Set collision flag to zero
        self.v[0xF] = 0x0;

        // Sprite => sequence of bytes in memory
        // Each byte, one horizontal row, 8 pixels wide
        // Bit = 1 => pixel is on, msb is the leftmost pixel
        
        // iterate through each row (each byte represents one horizontal row)
        for (row, &byte) in sprite.iter().enumerate() {
            for col in 0..8 {
                /*1101_0011  >> 7
                = 0000_0001        (bit 7 shifted all the way down)
                & 0000_0001
                = 1  pixel ON */
                /*1101_0011  >> 5
                = 0000_0110        
                & 0000_0001        (mask keeps only the lowest)
                = 0  pixel OFF */
                // slides bits down to check if bit is ON or OFF via AND mask 
                let pixel_on = (byte >> (7 - col) & 1);

                if pixel_on == 1 {
                    let x_coord = (initial_x_coord as usize) + col;
                    let y_coord = (initial_y_coord as usize) + row;
                    // get the array index for the pixel in the display per current col and row 
                    let display_index = (y_coord * 64) + x_coord;
                    // write pixel to display
                    self.display[display_index] = true;
                }
            }
        }
    }

    // 0xEX9E
    // Increase program counter by 2 if the key # stored in Vx is pressed
    pub(super) fn op_skip_keypress(&mut self, instr: Instruction) {
        if self.keypad[self.v[instr.x] as usize] {
            self.pc += 2;
        }
    }

    // 0xEXA1
    // Increase program counter by 2 if the key # stored in Vx is not pressed
    pub(super) fn op_skip_nokeypress(&mut self, instr: Instruction) {
        if !self.keypad[self.v[instr.x] as usize] {
            self.pc += 2;
        }
    }

    // 0xFX07
    // Set the value of the delay timer into Vx
    pub(super) fn op_save_dt(&mut self, instr: Instruction) {
        self.v[instr.x] = self.delay_timer;
    }

    // 0xFX0A

    // 0xFX15
    // Set the value of Vx to the display timer
    pub(super) fn op_load_dt(&mut self, instr: Instruction) {
        self.delay_timer = self.v[instr.x];
    }

    // 0xFX18
    // Set the value of Vx to the sound timer
    pub(super) fn op_load_st(&mut self, instr: Instruction) {
        self.sound_timer = self.v[instr.x];
    }

    // 0xFX1E
    // Adds the values of index register and Vx, and then stores result in index register
    pub(super) fn op_add_index(&mut self, instr: Instruction) {
        self.i = self.i + (self.v[instr.x] as u16);
    }

    // 0xFX29

    // 0xFX33

    // 0xFX55

    // 0xFX65
}
