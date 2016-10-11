extern crate sdl2;

mod chip8;
mod rom;
mod mem;
mod screen;

use chip8::Chip8;
use rom::Rom;

fn main() {
    let rom = Rom::load("./roms/PONG");

    let mut chip8 = Chip8::new();
    chip8.init();

    chip8.load_rom(rom);

    for i in 0..200 {
        chip8.run_cycle();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
