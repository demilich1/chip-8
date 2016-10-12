extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::thread;

use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

pub struct Screen {
    tx: Sender<ScreenSignal>
}

enum ScreenSignal {
    ClearScreen,
}

impl Screen {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let thread = thread::spawn(move || {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem.window("CHIP-8", 800, 600)
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

        Screen {
            tx: tx
        }
    }


}
