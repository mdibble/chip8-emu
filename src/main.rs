use chip8::Chip8;

mod chip8;

fn main() {
    let mut chip8 = Chip8::init();

    let mut payload: [u8; 4096 - 512];
    // Must initialize all of array upon creation?

    chip8.inject_data(payload);
}