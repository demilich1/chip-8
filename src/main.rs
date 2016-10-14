extern crate sdl2;
extern crate rand;

mod chip8;
mod rom;
mod mem;
mod screen;
mod opcode;

use chip8::Chip8;
use rom::Rom;

fn main() {
    let rom = Rom::load("./roms/PONG");

    let mut chip8 = Chip8::new();
    chip8.init();

    chip8.load_rom(rom);

    for _i in 0..1024 {
        chip8.run_cycle();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
