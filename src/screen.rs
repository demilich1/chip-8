extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::EventPump;

use screen_buffer::ScreenBuffer;
use keys::Keys;

use std::thread;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, TryRecvError};


pub const PIXEL_SIZE: u16 = 8;

pub struct Screen {
    renderer: Renderer<'static>,
    event_pump: EventPump,
    keys: Keys,
    quit: bool,
}

impl Screen {
    pub fn new(width: u16, height: u16) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("CHIP-8",
                    (width * PIXEL_SIZE) as u32,
                    (height * PIXEL_SIZE) as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        // let mut renderer = window.renderer().build().unwrap();
        //
        // renderer.set_draw_color(Color::RGB(15, 15, 15));
        // renderer.clear();
        // renderer.present();
        //
        //
        //
        //
        // let mut event_pump = sdl_context.event_pump().unwrap();
        //
        // 'running: loop {
        // match rx.try_recv() {
        // Ok(signal) => Screen::process_signal(&mut renderer, signal),
        // Err(err) => {
        // match err {
        // TryRecvError::Empty => (),
        // TryRecvError::Disconnected => {
        // panic!("An error occured while receiving screen signals: {:?}", err)
        // }
        // }
        // }
        // }
        // for event in event_pump.poll_iter() {
        // match event {
        // Event::Quit { .. } |
        // Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
        // _ => {}
        // }
        // }
        // The rest of the game loop goes here...
        // }
        //
        let renderer = window.renderer().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Screen {
            renderer: renderer,
            event_pump: event_pump,
            keys: Keys::new(),
            quit: false,
        }
    }

    pub fn draw(&mut self, buffer: &ScreenBuffer) {
        // let renderer = window.renderer().build().unwrap();
        self.renderer.set_draw_color(Color::RGB(15, 15, 15));
        self.renderer.clear();
        self.renderer.set_draw_color(Color::RGB(255, 255, 255));
        for px in 0..buffer.width() {
            for py in 0..buffer.height() {
                if !buffer.get_pixel(px, py) {
                    continue;
                }
                let x = (px * PIXEL_SIZE) as i32;
                let y = (py * PIXEL_SIZE) as i32;
                let rect = Rect::new(x, y, PIXEL_SIZE as u32, PIXEL_SIZE as u32);
                self.renderer.fill_rect(rect);
            }
        }
        self.renderer.present();
    }

    pub fn clear(&mut self) {
        self.renderer.set_draw_color(Color::RGB(15, 15, 15));
        self.renderer.clear();
        self.renderer.present();
    }

    pub fn poll_keys(&mut self) -> Keys {
        let mut keys = self.keys;
        let mut quit = false;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => quit = true,

                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => keys.set(0x1),
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => keys.set(0x2),
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => keys.set(0x3),
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => keys.set(0xC),
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => keys.set(0x4),
                Event::KeyDown { keycode: Some(Keycode::W), .. } => keys.set(0x5),
                Event::KeyDown { keycode: Some(Keycode::E), .. } => keys.set(0x6),
                Event::KeyDown { keycode: Some(Keycode::R), .. } => keys.set(0xD),
                Event::KeyDown { keycode: Some(Keycode::A), .. } => keys.set(0x7),
                Event::KeyDown { keycode: Some(Keycode::S), .. } => keys.set(0x8),
                Event::KeyDown { keycode: Some(Keycode::D), .. } => keys.set(0x9),
                Event::KeyDown { keycode: Some(Keycode::F), .. } => keys.set(0xE),
                Event::KeyDown { keycode: Some(Keycode::Y), .. } => keys.set(0xA),
                Event::KeyDown { keycode: Some(Keycode::X), .. } => keys.set(0x0),
                Event::KeyDown { keycode: Some(Keycode::C), .. } => keys.set(0xB),
                Event::KeyDown { keycode: Some(Keycode::V), .. } => keys.set(0xF),

                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => keys.unset(0x1),
                Event::KeyUp { keycode: Some(Keycode::Num2), .. } => keys.unset(0x2),
                Event::KeyUp { keycode: Some(Keycode::Num3), .. } => keys.unset(0x3),
                Event::KeyUp { keycode: Some(Keycode::Num4), .. } => keys.unset(0xC),
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => keys.unset(0x4),
                Event::KeyUp { keycode: Some(Keycode::W), .. } => keys.unset(0x5),
                Event::KeyUp { keycode: Some(Keycode::E), .. } => keys.unset(0x6),
                Event::KeyUp { keycode: Some(Keycode::R), .. } => keys.unset(0xD),
                Event::KeyUp { keycode: Some(Keycode::A), .. } => keys.unset(0x7),
                Event::KeyUp { keycode: Some(Keycode::S), .. } => keys.unset(0x8),
                Event::KeyUp { keycode: Some(Keycode::D), .. } => keys.unset(0x9),
                Event::KeyUp { keycode: Some(Keycode::F), .. } => keys.unset(0xE),
                Event::KeyUp { keycode: Some(Keycode::Y), .. } => keys.unset(0xA),
                Event::KeyUp { keycode: Some(Keycode::X), .. } => keys.unset(0x0),
                Event::KeyUp { keycode: Some(Keycode::C), .. } => keys.unset(0xB),
                Event::KeyUp { keycode: Some(Keycode::V), .. } => keys.unset(0xF),
                // Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        self.quit = quit;
        self.keys = keys;
        self.keys
    }

    pub fn quit(&self) -> bool {
        self.quit
    }
}
