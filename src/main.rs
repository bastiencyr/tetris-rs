extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;


const HEIGHT: i32 = 20;
const WIDTH: i32 = 10;


mod piece_gen;
mod controller;
mod tetris;
use crate::piece_gen::{Piece, PieceModel};
use crate::controller::{ResultController, Action, Controller};
use crate::tetris::{Tetris, Board};

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


    let canvas = window
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
    let mut tetris = Tetris::new(canvas, main_texture, 200, 400, 100, 200);//canvas is moved , so it is not accessible anymore
    
    let mut timer = sdl_context.timer()?;
    let mut event_pump = sdl_context.event_pump()?;

    //show background
    let texture_creator_back = tetris.canvas.texture_creator();
    let mut background = Background::new(&texture_creator_back);
    //tetris.set(&background);
    background.show(&mut tetris.canvas, &mut tetris.main_texture);

    //on déclare notre pièce
    let mut  piece: Piece = piece_gen::Piece::new(); // on le déclare comme mutable
    //tetris.draw(&piece);
    tetris.draw(&piece, &piece);

    //tetris.show()
    tetris.canvas.copy(&tetris.main_texture, None, None).expect("Cant copy");
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
		
		Event::KeyUp { keycode: Some(Keycode::Right), ..} => {
		    let act = Action::Right(piece.clone(), &tetris.board);
		    //act is moved. So, it cant be access after that.
		    let result_controller = Tetris::check(act);
		    match result_controller {
			ResultController::Ok =>{
			    let old_position: Piece = piece.translate_right(); // piece est mutable dans son scope mais si on lappelle dans une autre fonction, il faut le préciser.
			    tetris.draw(&old_position, &piece);
			    background.show(&mut tetris.canvas, &mut tetris.main_texture);
			    tetris.canvas.copy(&tetris.main_texture, None, None).expect("Cant copy");
			    //background.show(&mut tetris.canvas);
			    //pour des raisons de perf, present n'est pas appelé dans draw
			    tetris.canvas.present();
			}
			_ => {}
		    }
		}
		
		Event::KeyUp { keycode: Some(Keycode::Left), ..} => {
		    let act = Action::Left(piece.clone(), &tetris.board);
		    //act is moved. So, it cant be access after that.
		    let result_controller = Tetris::check(act);
		    match result_controller {
			ResultController::Ok =>{
			    let old_position: Piece = piece.translate_left(); // piece est mutable dans son scope mais si on lappelle dans une autre fonction, il faut le préciser.
			    //background.show(&mut tetris.ca;
			    tetris.draw(&old_position, &piece);
			    background.show(&mut tetris.canvas, &mut tetris.main_texture);
			    //pour des raisons de perf, present n'est pas appelé dans draw
			    tetris.canvas.copy(&tetris.main_texture, None, None).expect("Cant copy");
			    tetris.canvas.present();
			}
			_ => {}
		    }
		}

		Event::KeyUp { keycode: Some(Keycode::Down), ..} => {
		    let act = Action::Bottom(piece.clone(), &tetris.board);
		    //act is moved. So, it cant be access after that.
		    let result_controller = Tetris::check(act);
		    match result_controller {
			ResultController::Ok =>{
			    let old_position: Piece = piece.translate_down(); // piece est mutable dans son scope mais si on lappelle dans une autre fonction, il faut le préciser.
			    //background.show(&mut tetris.ca;
			    tetris.draw(&old_position, &piece);
			    background.show(&mut tetris.canvas, &mut tetris.main_texture);
			    //pour des raisons de perf, present n'est pas appelé dans draw
			    tetris.canvas.copy(&tetris.main_texture, None, None).expect("Cant copy");
			    tetris.canvas.present();
			}
			_ => {}
		    }
		}

		_ => {}
            }
        }
	
	new_time = timer.ticks() as i32;
	if new_time-old_time >= 1000{
	    old_time = new_time;

	    let act = Action::Bottom(piece.clone(), &tetris.board);
	    let result_controller = Tetris::check(act);
	    match result_controller {
		ResultController::Ok =>{
		    let old_position: Piece = piece.translate_down(); // piece est mutable dans son scope mais si on lappelle dans une autre fonction, il faut le préciser.
		    tetris.draw(&old_position, &piece);
		    background.show(&mut tetris.canvas, &mut tetris.main_texture);
		    tetris.canvas.copy(&tetris.main_texture, None, None).expect("Cant copy");
		    
		    //pour des raisons de perf, present n'est pas appelé dans draw
		    tetris.canvas.present();
		}
		ResultController::BottomBorder | ResultController::CollisionPieceBottom => {
		    tetris.update_board(&piece);
		    piece.reinit();
		    tetris.draw(&piece, &piece);
		}
		_ => {}
	    }
	}

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
/*



extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-sdl2 resource-manager demo", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, 400, 300)
        .map_err(|e| e.to_string())?;

    let mut angle = 0.0;

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
        angle = (angle + 0.5) % 360.;
        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.clear();
                texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
                texture_canvas
                    .fill_rect(Rect::new(0, 0, 400, 300))
                    .expect("could not fill rect");
            })
            .map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        let dst = Some(Rect::new(0, 0, 400, 300));
        canvas.clear();
        canvas.copy_ex(
            &texture,
            None,
            dst,
            angle,
            Some(Point::new(400, 300)),
            false,
            false,
        )?;
        canvas.present();
	canvas.present();
    }

    Ok(())
}

*/
