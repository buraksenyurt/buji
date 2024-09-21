extern crate sdl2;

use crate::states::EngineState;
use crate::world::World;
use crate::{ActorContext, AssetStore};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::thread::sleep;
use std::time::{Duration, Instant};

const NANOS_PER_SECOND: u32 = 1_000_000_000;
const FPS: u32 = 60;

pub struct GameEngine {
    canvas: WindowCanvas,
    asset_store: AssetStore,
    texture_creator: TextureCreator<WindowContext>,
    fps: u32,
    background_color: Color,
    sdl_context: sdl2::Sdl,
    pub world: World,
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

                    for (actor, actor_context) in &self.world.actors {
                        if let Some(new_state) = actor.update(&mut actor_context.borrow_mut()) {
                            state = new_state;
                        }

                        if let Err(e) = Self::draw_actor(
                            &mut self.canvas,
                            &mut self.asset_store,
                            &self.texture_creator,
                            &actor_context.borrow(),
                        ) {
                            eprintln!("Failed to draw actor: {}", e);
                        }

                        actor.draw(&actor_context.borrow());
                    }

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

    pub fn draw_actor(
        canvas: &mut WindowCanvas,
        asset_store: &mut AssetStore,
        texture_creator: &TextureCreator<WindowContext>,
        actor: &ActorContext,
    ) -> Result<(), String> {
        let image_data = asset_store.load_or_insert(actor.id, &actor.image_path)?;
        let mut image_data_mut = image_data.to_vec();
        let surface = sdl2::surface::Surface::from_data(
            image_data_mut.as_mut_slice(),
            64,
            64,
            64 * 3,
            sdl2::pixels::PixelFormatEnum::RGB24,
        )
        .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let scale_factor = actor.scale.0;
        let (width, height) = (texture.query().width, texture.query().height);
        let scaled_width = (width as f32 * scale_factor) as u32;
        let scaled_height = (height as f32 * scale_factor) as u32;

        let position = &actor.position;
        let target_rect = Rect::new(position.x, position.y, scaled_width, scaled_height);

        canvas.copy_ex(&texture, None, Some(target_rect), 0.0, None, false, false)?;

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
    pub fn set_window(mut self, title: &str, width: u32, height: u32) -> Self {
        self.title = title.to_string();
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
            world: World::default(),
            asset_store: AssetStore::default(),
        }
    }
}
