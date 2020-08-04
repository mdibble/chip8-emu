use chip8::Chip8;

use std::fs;

mod chip8;

fn main() {
    let mut system = Chip8::init();
    let path = format!("rom/pong2.c8");
    let payload = fs::read(path);
    let payload = match payload {
        Ok(g) => g,
        Err(_) => panic!("Error!")
    };
    system.inject_data(payload);

    loop {
        system.cycle();
    }
}