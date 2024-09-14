extern crate sdl2;

use crate::states::EngineState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::thread::sleep;
use std::time::{Duration, Instant};

const NANOS_PER_SECOND: u32 = 1_000_000_000;
const FPS: u32 = 60;

pub struct GameEngine {
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    fps: u32,
    background_color: Color,
    sdl_context: sdl2::Sdl,
}

impl GameEngine {
    pub fn builder() -> GameEngineBuilder {
        GameEngineBuilder::default()
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut state = EngineState::Init;
        let mut last_update = Instant::now();
        let frame_duration = Duration::new(0, NANOS_PER_SECOND / self.fps);
        let mut event_pump = self.sdl_context.event_pump()?;

        loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        state = EngineState::PreExit;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        state = EngineState::PreExit;
                    }
                    _ => {}
                }
            }

            match state {
                EngineState::Init => {
                    state = EngineState::Running;
                    continue;
                }
                EngineState::Running => {
                    let now = Instant::now();
                    let delta = now.duration_since(last_update);

                    self.canvas.set_draw_color(self.background_color);
                    self.canvas.clear();

                    self.canvas.present();

                    if frame_duration > delta {
                        sleep(frame_duration - delta);
                    }

                    last_update = now;
                }
                EngineState::PreExit => {
                    state = EngineState::Exit;
                    continue;
                }
                EngineState::Exit => {
                    break;
                }
            }
        }

        Ok(())
    }
}

pub struct GameEngineBuilder {
    title: String,
    width: u32,
    height: u32,
    fps: u32,
    background_color: Color,
}

impl Default for GameEngineBuilder {
    fn default() -> Self {
        Self {
            title: "Anonymous".to_string(),
            width: 800,
            height: 600,
            fps: FPS,
            background_color: Color::RGB(0, 0, 0),
        }
    }
}

impl GameEngineBuilder {
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    pub fn background_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background_color = Color::RGB(r, g, b);
        self
    }

    pub fn build(self) -> GameEngine {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(&self.title, self.width, self.height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        GameEngine {
            canvas,
            texture_creator,
            fps: self.fps,
            background_color: self.background_color,
            sdl_context,
        }
    }
}
