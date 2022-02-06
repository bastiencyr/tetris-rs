extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::VideoSubsystem;

use ui::background::Background;

use crate::board::Board;
use crate::controller::{Controller, TetrisEvent, TraitController};
use crate::model::TetrisModel;
use crate::piece::Piece;
use crate::view::TetrisView;

const HEIGHT: i32 = 20;
const WIDTH: i32 = 10;

mod board;
mod controller;
mod model;
mod piece;
mod player;
mod view;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let (mut canvas, mut creator) = init_canvas(&video_subsystem);
    let main_texture = creator
        .create_texture_target(creator.default_pixel_format(), 300, 600)
        .unwrap();
    let background = Background::new(&mut canvas, &creator);
    let mut controller = Controller::new();
    controller.set_model(TetrisModel::new());

    let view = TetrisView::builder(canvas, main_texture, background)
        .color_piece(Color::RGBA(180, 73, 63, 255))
        .color_line(Color::RGBA(70, 50, 50, 255))
        .color_back2(Color::RGBA(10, 20, 30, 255))
        .color_ghost(Color::RGBA(60, 46, 90, 80))
        .build();

    let id_view = controller.register_view(Box::new(view));

    let timer = sdl_context.timer()?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut running = true;
    let mut old_time = 0;
    let mut new_time;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    controller.update_model(TetrisEvent::Left); //on_key_right();
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    controller.update_model(TetrisEvent::Right); //on_key_right();
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    controller.update_model(TetrisEvent::Bottom); //on_key_right();
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    controller.update_model(TetrisEvent::Up); //on_key_right();
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    controller.update_model(TetrisEvent::Space); //on_key_right();
                }

                _ => {}
            }
        }

        new_time = timer.ticks() as i32;
        // send a signal every 10 milliseconds
        if new_time - old_time >= 10 {
            old_time = new_time;
            controller.update_model(TetrisEvent::Time(10 as usize));
        }
    }
    Ok(())
}

fn init_canvas(video_subsystem: &VideoSubsystem) -> (WindowCanvas, TextureCreator<WindowContext>) {
    let window = video_subsystem
        .window("SDL2", 300, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let canvas = window
        .into_canvas()
        .software()
        //.accelerated()
        //.present_vsync()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let creator = canvas.texture_creator();
    (canvas, creator)
}
