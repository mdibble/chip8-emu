extern crate sdl2; 

use chip8::CHIP8;

use std::fs;
use std::time::Duration;
use std::thread;

mod chip8;

fn main() {
    let mut system = CHIP8::init();
    
    let payload_path = "rom/pong2.ch8";
    let payload = fs::read(payload_path);

    let payload = match payload {
        Ok(g) => g,
        Err(_) => panic!("Couldn't retrieve payload! Path given: '{}'", payload_path)
    };
    
    system.inject(payload);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("CHIP-8 Emulator", 640, 320).position_centered().build().unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    loop {
        system.process_input(&mut event_pump);
        if system.key[1] && system.key[2] && system.key[3] && system.key[12] && system.key[8] {
            break; // TEMPORARY solution for exiting the system
        }
        system.cycle();
        system.draw(&mut canvas);
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 500));
    }
}