use chip8::CHIP8;

use std::fs;

mod chip8;

fn main() {
    let mut system = CHIP8::init();
    
    let payload_path = "rom/pong2.c8";
    let payload = fs::read(payload_path);

    let payload = match payload {
        Ok(g) => g,
        Err(_) => panic!("Couldn't retrieve payload! Path given: '{}'", payload_path)
    };
    
    system.inject(payload);

    loop {
        system.cycle();
    }
}