extern crate sdl2;

use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

//la librairie est linké donc pas besoin de limporter comme un module
use ui::background::Background;

use crate::board::Board;
use crate::controller::{Controller, TetrisEvent, TraitController};
use crate::piece::Piece;

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

    let window = video_subsystem
        .window("SDL2", 300, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        //.accelerated()
        //.present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    let main_creator = canvas.texture_creator();
    let mut main_texture = main_creator
        .create_texture_target(main_creator.default_pixel_format(), 300, 600)
        .unwrap();

    let texture_creator_back = canvas.texture_creator();
    let background = Rc::new(Background::new(&mut canvas, &texture_creator_back));
    background.copy_back_to_texture(&mut canvas, &mut main_texture);

    let mut controller = Controller::new(canvas, main_texture, background);
    controller.update_view();

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
