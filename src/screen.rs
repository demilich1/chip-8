extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::rect::Rect;

use std::thread;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, TryRecvError};

pub const SCREEN_WIDTH: u32 = 64;
pub const SCREEN_HEIGHT: u32 = 32;
pub const PIXEL_SIZE: u32 = 8;

pub struct Screen {
    tx: Sender<ScreenSignal>,
}

pub struct ScreenBuffer {
    pixels: Vec<bool>,
}

impl ScreenBuffer {
    pub fn new() -> Self {
        let size = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
        ScreenBuffer { pixels: vec![false; size] }
    }

    pub fn width(&self) -> u32 {
        SCREEN_WIDTH
    }

    pub fn height(&self) -> u32 {
        SCREEN_HEIGHT
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        let index: usize = (x * SCREEN_HEIGHT + y) as usize;
        self.pixels[index] = true
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> bool {
        let index: usize = (x * SCREEN_HEIGHT + y) as usize;
        self.pixels[index]
    }
}

enum ScreenSignal {
    Draw { pixels: ScreenBuffer },
}

impl Screen {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let thread = thread::spawn(move || {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem.window("CHIP-8",
                        SCREEN_WIDTH * PIXEL_SIZE,
                        SCREEN_HEIGHT * PIXEL_SIZE)
                .position_centered()
                .opengl()
                .build()
                .unwrap();

            let mut renderer = window.renderer().build().unwrap();

            renderer.set_draw_color(Color::RGB(15, 15, 15));
            renderer.clear();
            renderer.present();
            let mut event_pump = sdl_context.event_pump().unwrap();

            'running: loop {
                match rx.try_recv() {
                    Ok(signal) => Screen::process_signal(&mut renderer, signal),
                    Err(err) => {
                        match err {
                            TryRecvError::Empty => (),
                            TryRecvError::Disconnected => {
                                panic!("An error occured while receiving screen signals: {:?}", err)
                            }
                        }
                    }
                }
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                        _ => {}
                    }
                }
                // The rest of the game loop goes here...
            }
        });

        Screen { tx: tx }
    }

    fn process_signal(renderer: &mut Renderer, signal: ScreenSignal) {
        renderer.set_draw_color(Color::RGB(15, 15, 15));
        renderer.clear();
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        match signal {
            ScreenSignal::Draw { pixels } => {
                for px in 0..pixels.width() {
                    for py in 0..pixels.height() {
                        if !pixels.get_pixel(px, py) {
                            continue;
                        }
                        let x = (px * PIXEL_SIZE) as i32;
                        let y = (py * PIXEL_SIZE) as i32;
                        let rect = Rect::new(x, y, PIXEL_SIZE, PIXEL_SIZE);
                        renderer.fill_rect(rect);
                    }
                }
            }
        }
        renderer.present();
    }


    pub fn draw(&mut self, buffer: ScreenBuffer) {
        self.tx.send(ScreenSignal::Draw { pixels: buffer });
    }
}
