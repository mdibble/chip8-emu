extern crate rand;

use rand::Rng;

pub struct CHIP8 {
    memory: [u8; 4096], // System memory (4K, 8-bit values)
    opcode: u16,        // Current opcode (16-bit value)
    stack: [u16; 16],   // Stack (16 levels, 16-bit values)
    v: [u8; 16],        // CPU Registers (16, 8-bit values)
    i: u16,             // Index register (16-bit value)
    pc: u16,            // Program counter (16-bit value)
    sp: u16,            // Stack pointer (16-bit value)
    delay_timer: u8,    // Delay timer (8-bit value)
    sound_timer: u8,    // Sound timer (8-bit value)
    gfx: [u8; 64 * 32], // Graphics (2048 pixels, 64x32 arrangement, 8-bit values)
    key: [bool; 16]       // Keypad (Status of 16 keys [0-F], 8-bit values)
}

impl CHIP8 {
    pub fn init() -> CHIP8 {
        let mut chip8 = CHIP8 {
            memory: [0; 4096],
            opcode: 0,
            stack: [0; 16],
            v: [0; 16],
            i: 0x200,
            pc: 0x200,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            gfx: [0; 64 * 32],
            key: [false; 16]
        };

        for i in 0..80 {
            chip8.memory[i] = FONTSET[i];
        }

        chip8
    }

    pub fn inject(&mut self, injection: Vec<u8>) {
        for i in 0..injection.len() {
            self.memory[0x200 + i] = injection[i];
        }
    }

    pub fn cycle(&mut self) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16);
        
        let op1 = ((self.opcode & 0xF000) >> 12) as u8;
        let op2 = ((self.opcode & 0x0F00) >> 8) as u8;
        let op3 = ((self.opcode & 0x00F0) >> 4) as u8;
        let op4 = (self.opcode & 0x000F) as u8;

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        
        print!("Executing 0x{:x}{:x}{:x}{:x}... ", op1, op2, op3, op4);
        
        match (op1, op2, op3, op4) {
            (0x0, 0x0, 0xE, 0x0) => self.op_00e0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00ee(),
            (0x1, _, _, _) => self.op_1nnn(),
            (0x2, _, _, _) => self.op_2nnn(),
            (0x3, _, _, _) => self.op_3xnn(op2),
            (0x4, _, _, _) => self.op_4xnn(op2),
            (0x5, _, _, 0x0) => self.op_5xy0(op2, op3),
            (0x6, _, _, _) => self.op_6xnn(op2),
            (0x7, _, _, _) => self.op_7xnn(op2),
            (0x8, _, _, 0x0) => self.op_8xy0(op2, op3),
            (0x8, _, _, 0x1) => self.op_8xy1(op2, op3),
            (0x8, _, _, 0x2) => self.op_8xy2(op2, op3),
            (0x8, _, _, 0x3) => self.op_8xy3(op2, op3),
            (0x8, _, _, 0x4) => self.op_8xy4(op2, op3),
            (0x8, _, _, 0x5) => self.op_8xy5(op2, op3),
            (0x9, _, _, 0x0) => self.op_9xy0(op2, op3),
            (0xA, _, _, _) => self.op_annn(),
            (0xB, _, _, _) => self.op_bnnn(),
            (0xC, _, _, _) => self.op_cxnn(op2),
            (0xD, _, _, _) => self.op_dxyn(op2, op3),
            (0xE, _, 0x9, 0xE) => self.op_ex9e(op2),
            (0xE, _, 0xA, 0x1) => self.op_exa1(op2),
            (0xF, _, 0x0, 0x7) => self.op_fx07(op2),
            (0xF, _, 0x1, 0x5) => self.op_fx15(op2),
            (0xF, _, 0x1, 0x8) => self.op_fx18(op2),
            (0xF, _, 0x1, 0xE) => self.op_fx1e(op2),
            (0xF, _, 0x2, 0x9) => self.op_fx29(op2),
            (0xF, _, 0x3, 0x3) => self.op_fx33(op2),
            (0xF, _, 0x5, 0x5) => self.op_fx55(op2),
            (0xF, _, 0x6, 0x5) => self.op_fx65(op2),
            _ => panic!("Couldn't match opcode! Instruction given: 0x{:x}", self.opcode)
        }

        println!("success!");
    }

    // 00E0: Clears the screen
    fn op_00e0(&mut self) {
        for i in 0..self.gfx.len() {
            self.gfx[i] = 0;
        }
        self.pc = self.opcode & 0x0FFF;
    }

    // 00EE: Returns from a subroutine
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        self.pc += 2; // are you supposed to do this? lol
    }

    // 1NNN: Jumps to address NNN
    fn op_1nnn(&mut self) {
        self.pc = self.opcode & 0x0FFF;
    }

    // 2NNN: Calls subroutine at NNN
    fn op_2nnn(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.opcode & 0x0FFF;
    }

    // 3XNN: Skips the next instruction if VX equals NN
    fn op_3xnn(&mut self, x: u8) {
        if self.v[x as usize] == (self.opcode & 0x00FF) as u8 {
            self.pc += 4;
        }
        else {
            self.pc += 2;
        }
    }

    // 4XNN: Skips the next instruction if VX doesn't equal NN
    fn op_4xnn(&mut self, x: u8) {
        if self.v[x as usize] != (self.opcode & 0x00FF) as u8 {
            self.pc += 4;
        }
        else {
            self.pc += 2;
        }
    }

    // 5XY0: Skips the next instruction if VX equals VY
    fn op_5xy0(&mut self, x: u8, y: u8) {
        if self.v[x as usize] == self.v[y as usize] as u8 {
            self.pc += 4;
        }
        else {
            self.pc += 2;
        }
    }

    // 6XNN: Sets VX to NN
    fn op_6xnn(&mut self, x: u8) {
        self.v[x as usize] = (self.opcode & 0x00FF) as u8;
        self.pc += 2;
    }

    // 7XNN: Adds NN to VX (carry flag is not changed)
    fn op_7xnn(&mut self, x: u8) {
        self.v[x as usize] = self.v[x as usize].wrapping_add((self.opcode & 0x00FF) as u8);
        self.pc += 2;
    }

    // 8XY0: Sets VX to the value of VY
    fn op_8xy0(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[y as usize];
        self.pc += 2;
    }

    // 8XY1: Sets VX to VX or VY (bitwise OR operation)
    fn op_8xy1(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
        self.pc += 2;
    }

    // 8XY2: Sets VX to VX and VY (bitwise AND operation)
    fn op_8xy2(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
        self.pc += 2;
    }

    // 8XY3: Sets VX to VX xor VY (bitwise xor operation)
    fn op_8xy3(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
        self.pc += 2;
    }

    // 8XY4: Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
    fn op_8xy4(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[x as usize].wrapping_add(self.v[y as usize]);

        if self.v[x as usize] > 0xFF - self.v[y as usize] {
            self.v[0xF] = 1;
        }
        else {
            self.v[0xF] = 0;
        }
        self.pc += 2;
    }

    // 8XY5: VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't
    fn op_8xy5(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[x as usize].wrapping_sub(self.v[y as usize]);

        if self.v[y as usize] > self.v[x as usize] {
            self.v[0xF] = 1;
        }
        else {
            self.v[0xF] = 0;
        }
        self.pc += 2;
    }

    // 9XY0: Skips the next instruction if VX doesn't equal VY
    fn op_9xy0(&mut self, x: u8, y: u8) {
        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 4;
        }
        else {
            self.pc += 2;
        }
    }

    // ANNN: Sets I to the address NNN
    fn op_annn(&mut self) {
        self.i = (self.opcode & 0x0FFF) as u16;
        self.pc += 2;
    }

    // BNNN: Jumps to the address NNN plus V0
    fn op_bnnn(&mut self) {
        self.pc = (self.opcode & 0x0FFF) as u16 + self.v[0] as u16;
    }

    // CXNN: Sets VX to the result of a bitwise and operation on a random number and NN
    fn op_cxnn(&mut self, x: u8) {
        let rng: u8 = rand::thread_rng().gen();
        self.v[x as usize] = rng & (self.opcode & 0x00FF) as u8;
        self.pc += 2;
    }

    // DXYN: Draws a sprite at coordinate (VX, XY) w/ width 8 pixels and height N pixels
    fn op_dxyn(&mut self, x: u8, y: u8) {
        let x_coord: u8 = self.v[x as usize];
        let y_coord: u8 = self.v[y as usize];
        
        let w = 8 as u8;
        let h = (self.opcode & 0x000F) as u8;
        
        self.v[0xF] = 0;

        let mut pixel: u8;

        for row in 0..h as usize {
            pixel = self.memory[self.i as usize + row];
            for col in 0..w as usize {
                if pixel & (0x80 >> col) != 0 {
                    if self.gfx[x_coord as usize + col + ((y_coord as usize + row) * 64)] == 1 {
                        self.v[0xF] = 1;
                    }
                    self.gfx[x_coord as usize + col + ((y_coord as usize + row) * 64)] ^= 1;
                }
            }
        }
        
        // println!();
        // for i in 0..32 {
        //     for j in 0..64 {
        //         print!("{} ", self.gfx[j + (i * 64)] as u8);
        //     }
        //     println!();
        // }

        self.pc += 2;
    }

    // EX9E: Skips the next instruction if the key stored in VX is pressed
    fn op_ex9e(&mut self, x: u8) {
        if self.key[self.v[x as usize] as usize] == true {
            self.pc += 4;
        }
        else {
            self.pc += 2;
        }
    }

    // EXA1: Skips the next instruction if the key stored in VX isn't pressed
    fn op_exa1(&mut self, x: u8) {
        if self.key[self.v[x as usize] as usize] == false {
            self.pc += 4;
        }
        else {
            self.pc += 2;
        }
    }

    // FX07: Sets VX to the value of the delay timer
    fn op_fx07(&mut self, x: u8) {
        self.v[x as usize] = self.delay_timer;
        self.pc += 2;
    }

    // FX15: Sets the delay timer to VX
    fn op_fx15(&mut self, x: u8) {
        self.delay_timer = self.v[x as usize];
        self.pc += 2;
    }

    // FX18: Sets the sound timer to VX
    fn op_fx18(&mut self, x: u8) {
        self.sound_timer = self.v[x as usize];
        self.pc += 2;
    }

    // FX1E: Adds VX to I, VF is not affected
    fn op_fx1e(&mut self, x: u8) {
        self.i += self.v[x as usize] as u16;
        self.pc += 2;
    }

    // FX29: Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font
    fn op_fx29(&mut self, x: u8) {
        self.i = self.v[x as usize] as u16 * 5;
        self.pc += 2;
    }

    // FX33: Stores the binary-coded decimal representation of VX. Ex. 255 -> I = 2, I + 1 = 5, I + 2 = 5)
    fn op_fx33(&mut self, x: u8) {
        let val = self.v[x as usize];
        self.memory[self.i as usize] = val / 100; 
        self.memory[(self.i + 1) as usize] = (val / 10) % 10; 
        self.memory[(self.i + 2) as usize] = val % 10; 
        self.pc += 2;
    }

    // Dumps V0 to VX inclusive in memory starting at address I. The offset from I is increased by 1 for each value written
    fn op_fx55(&mut self, x: u8) {
        for index in 0..=x as usize {
            self.memory[self.i as usize + index] = self.v[index];
        }
        self.pc += 2;
    }

    // Fills V0 to VX inclusive with values from memory starting at address I. The offset from I is increased by 1 for each value written
    fn op_fx65(&mut self, x: u8) {
        for index in 0..=x as usize {
            self.v[index] = self.memory[self.i as usize + index];
        }
        self.pc += 2;
    }
}

const FONTSET: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];