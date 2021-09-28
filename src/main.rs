extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

const HEIGHT: i32 = 20;
const WIDTH: i32 = 10;

mod controller;
mod piece_gen;
mod player;
mod tetris;
use crate::controller::{Action, Controller, ResultController};
use crate::piece_gen::{Piece, PieceModel, PieceView};
use crate::tetris::{Board, Tetris};

//la librairie est linké donc pas besoin de limporter comme un module
use ui::background::Background;

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
    let main_texture = main_creator
        .create_texture_target(main_creator.default_pixel_format(), 300, 600)
        .unwrap();

    let texture_creator_back = canvas.texture_creator();
    let mut background = Background::new(&mut canvas, &texture_creator_back);

    let mut tetris = Tetris::new(canvas, main_texture, &background); //canvas is moved , so it is not accessible anymore

    let mut timer = sdl_context.timer()?;
    let mut event_pump = sdl_context.event_pump()?;

    //show background
    //let texture_creator_back = tetris.canvas.texture_creator();
    //let mut background = Background::new(&mut tetris.canvas, &texture_creator_back);
    //tetris.set(&background);
    background.show(&mut tetris.canvas, &mut tetris.main_texture);

    //on déclare notre pièce
    //let mut  piece: Piece = piece_gen::Piece::new(); // on le déclare comme mutable
    //tetris.draw(&piece);
    //ther is just one player but Iterator

    for player in &tetris.player {
        player.draw_piece(&mut tetris.canvas, &mut tetris.main_texture);
    }

    tetris
        .canvas
        .copy(&tetris.main_texture, None, None)
        .expect("Cant copy");
    tetris.canvas.present();
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
                    keycode: Some(Keycode::Right),
                    ..
                } => tetris.on_key_right(),

                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    for player in &mut tetris.player {
                        let act = Action::Left(player.piece.clone(), &player.board);
                        //act is moved. So, it cant be access after that.
                        let result_controller = Tetris::check(act);
                        match result_controller {
                            ResultController::Ok => {
                                player.piece.translate_left();
                                player.draw_piece(&mut tetris.canvas, &mut tetris.main_texture);
                                tetris.canvas.present();
                            }
                            _ => {}
                        }
                    }
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    for player in &mut tetris.player {
                        let act = Action::Bottom(player.piece.clone(), &player.board);
                        //act is moved. So, it cant be access after that.
                        let result_controller = Tetris::check(act);
                        match result_controller {
                            ResultController::Ok => {
                                player.piece.translate_down();
                                player.draw_piece(&mut tetris.canvas, &mut tetris.main_texture);
                                tetris.canvas.present();
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        new_time = timer.ticks() as i32;
        if new_time - old_time >= 1000 {
            old_time = new_time;
            for player in &mut tetris.player {
                let act = Action::Bottom(player.piece.clone(), &player.board);
                let result_controller = Tetris::check(act);
                match result_controller {
                    ResultController::Ok => {
                        player.piece.translate_down();
                        //on dessinne la nouvelle pièce
                        player.draw_piece(&mut tetris.canvas, &mut tetris.main_texture);
                        tetris.canvas.present();
                    }
                    ResultController::BottomBorder | ResultController::CollisionPieceBottom => {
                        player.update_board();
                        player.piece.reinit();
                        //tetris.draw(&piece, &piece);
                        player.draw_piece(&mut tetris.canvas, &mut tetris.main_texture);
                        //tetris.canvas.copy(&tetris.main_texture, None, None).expect("Cant copy");
                        tetris.canvas.present();
                    }
                    _ => {}
                }
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
