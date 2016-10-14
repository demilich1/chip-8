extern crate sdl2;
extern crate rand;

mod chip8;
mod opcode;
mod rom;
mod mem;
mod screen;
mod screen_buffer;
mod keys;

use chip8::Chip8;
use rom::Rom;

fn main() {
    let rom = Rom::load("./roms/VERS");

    let mut chip8 = Chip8::new();
    chip8.init();

    chip8.load_rom(rom);

    loop {
        chip8.run_cycle();
        std::thread::sleep(std::time::Duration::from_millis(4));
    }
}
