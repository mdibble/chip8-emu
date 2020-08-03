pub struct Chip8 {
    memory: [u8; 4096], // 4K memory (8-bit values)
    opcode: u16,        // Current opcode (16-bit value)
    stack: [u16; 16],   // Stack (16 levels, 16-bit values)
    v: [u8; 16],        // CPU Registers (16, 8-bit values)
    i: u16,             // Index register (16-bit value)
    pc: u16,            // Program counter (16-bit value)
    sp: u16,            // Stack pointer (16-bit value)
    delay_timer: u8,    // Delay timer (8-bit value)
    sound_timer: u8,    // Sound timer (8-bit value)
    gfx: [u8; 64 * 32], // Graphics (2048 pixels, 64x32 arrangement, 8-bit values)
    key: [u8; 16]       // Keypad (Status of 16 keys [0-F], 8-bit values)
}

impl Chip8 {
    pub fn init() -> Chip8 {
        let mut chip8 = Chip8 {
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
            key: [0; 16]
        };
        chip8
    }

    pub fn inject_data(&mut self, data: [u8; 4096 - 512]) {
        for i in 0..(4096 - 512) {
            println!("{}", data[i]);
        }
    }
}