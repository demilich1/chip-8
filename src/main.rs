mod chip8;
mod rom;
mod mem;

use chip8::Chip8;
use rom::Rom;

fn main() {
    let rom = Rom::load("./roms/PONG");

    let mut chip8 = Chip8::new();
    chip8.init();

    chip8.load_rom(rom);

    for i in 0..3 {
        chip8.run_cycle();
    }
}
