mod chip8;
mod keys;
mod mem;
mod opcode;
mod rom;
mod screen_buffer;

use chip8::Chip8;
use rom::Rom;

use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, ContextBuilder, GameResult};

const CYCLES_PER_FRAME: u8 = 3;

fn main() -> GameResult {
    let window_setup = WindowSetup::default().title("CHIP-8");
    let (mut ctx, mut event_loop) = ContextBuilder::new("chip_8", "demilich")
        .window_setup(window_setup)
        .build()
        .expect("Could not create ggez context!");

    let mut window = MainWindow::new(&mut ctx)?;

    match event::run(&mut ctx, &mut event_loop, &mut window) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
    Ok(())
}

struct MainWindow {
    redraw: bool,
    chip8: Chip8,
    width: u16,
    height: u16,
    scale: u16,
    buffer: Vec<u8>,
}

impl MainWindow {
    pub fn new(_ctx: &mut Context) -> GameResult<MainWindow> {
        let scale = 10;
        let width = chip8::SCREEN_WIDTH * scale;
        let height = chip8::SCREEN_HEIGHT * scale;
        let rom = Rom::load("./roms/PONG2");

        let mut chip8 = Chip8::new();
        chip8.init();
        chip8.load_rom(rom);

        let state = MainWindow {
            redraw: true,
            chip8,
            width,
            height,
            scale,
            buffer: vec![0; width as usize * height as usize * 4],
        };
        Ok(state)
    }
}

impl EventHandler for MainWindow {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for _ in 0..CYCLES_PER_FRAME {
            self.chip8.run_cycle();
            if self.chip8.redraw() {
                self.redraw = true;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.redraw {
            return Ok(());
        }
        self.redraw = false;

        graphics::clear(ctx, graphics::BLACK);
        for y in 0..chip8::SCREEN_HEIGHT {
            for x in 0..chip8::SCREEN_WIDTH {
                let start_x = x * self.scale;
                let start_y = y * self.scale;
                let end_x = start_x + self.scale;
                let end_y = start_y + self.scale;
                let pixel = self.chip8.screen_buffer().get_pixel(x, y);
                for render_x in start_x..end_x {
                    for render_y in start_y..end_y {
                        let index =
                            (render_y as usize * self.width as usize + render_x as usize) * 4;
                        self.buffer[index] = if pixel { 255 } else { 0 };
                        self.buffer[index + 1] = if pixel { 255 } else { 0 };
                        self.buffer[index + 2] = if pixel { 255 } else { 0 };
                        self.buffer[index + 3] = 255;
                    }
                }
            }
        }

        let image = graphics::Image::from_rgba8(ctx, self.width, self.height, &self.buffer)?;

        let dst = Point2::new(0.0, 0.0);
        graphics::draw(ctx, &image, (dst,))?;
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        match key {
            KeyCode::Escape => self.chip8.set_key(0x0),
            KeyCode::Key1 => self.chip8.set_key(0x1),
            KeyCode::Key2 => self.chip8.set_key(0x2),
            KeyCode::Key3 => self.chip8.set_key(0x3),
            KeyCode::Key4 => self.chip8.set_key(0xC),
            KeyCode::Q => self.chip8.set_key(0x4),
            KeyCode::W => self.chip8.set_key(0x5),
            KeyCode::E => self.chip8.set_key(0x6),
            KeyCode::R => self.chip8.set_key(0xD),
            KeyCode::A => self.chip8.set_key(0x7),
            KeyCode::S => self.chip8.set_key(0x8),
            KeyCode::D => self.chip8.set_key(0x9),
            KeyCode::F => self.chip8.set_key(0xE),
            KeyCode::Y => self.chip8.set_key(0xA),
            KeyCode::X => self.chip8.set_key(0x0),
            KeyCode::C => self.chip8.set_key(0xB),
            KeyCode::V => self.chip8.set_key(0xF),
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods) {
        match key {
            KeyCode::Escape => self.chip8.unset_key(0x0),
            KeyCode::Key1 => self.chip8.unset_key(0x1),
            KeyCode::Key2 => self.chip8.unset_key(0x2),
            KeyCode::Key3 => self.chip8.unset_key(0x3),
            KeyCode::Key4 => self.chip8.unset_key(0xC),
            KeyCode::Q => self.chip8.unset_key(0x4),
            KeyCode::W => self.chip8.unset_key(0x5),
            KeyCode::E => self.chip8.unset_key(0x6),
            KeyCode::R => self.chip8.unset_key(0xD),
            KeyCode::A => self.chip8.unset_key(0x7),
            KeyCode::S => self.chip8.unset_key(0x8),
            KeyCode::D => self.chip8.unset_key(0x9),
            KeyCode::F => self.chip8.unset_key(0xE),
            KeyCode::Y => self.chip8.unset_key(0xA),
            KeyCode::X => self.chip8.unset_key(0x0),
            KeyCode::C => self.chip8.unset_key(0xB),
            KeyCode::V => self.chip8.unset_key(0xF),
            _ => (),
        }
    }
}
