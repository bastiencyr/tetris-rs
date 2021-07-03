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
use crate::piece_gen::{Barre, PieceGen};
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
	//.accelerated()
	//.present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut tetris = Tetris::new(canvas, 200, 400, 100, 200);//canvas is moved , so it is not accessible anymore
    
    let mut timer = sdl_context.timer()?;
    let mut event_pump = sdl_context.event_pump()?;

    //show background
    let texture_creator_back = tetris.canvas.texture_creator();
    let mut background = Background::new(&texture_creator_back);
    //tetris.set(&background);
    background.show(&mut tetris.canvas);

    //on déclare notre pièce
    let mut  barre: Barre = piece_gen::Barre::new(); // on le déclare comme mutable
    //tetris.draw(&piece);
    barre.draw(&mut tetris.canvas, &barre);

    //tetris.show()
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
		Event::KeyUp { keycode: Some(Keycode::Space), ..} => {
		    let act = Action::Right(barre.clone(), &tetris.board);
		    //act is moved. So, it cant be access after that.
		    let result_controller = Tetris::check(act);
		    match result_controller {
			ResultController::Ok =>{
			    let old_position: Barre = barre.translate_right(); // piece est mutable dans son scope mais si on lappelle dans une autre fonction, il faut le préciser.
			    background.show(&mut tetris.canvas);
			    barre.draw(&mut tetris.canvas, &old_position);
			    //pour des raisons de perf, present n'est pas appelé dans draw
			    //tetris.canvas.clear();
			    tetris.canvas.present();
			    tetris.canvas.clear();
			}
			_ => {}
		    }
		    //std::thread::sleep(Duration::from_millis(1000));
		}
                _ => {}
            }
        }
	
	new_time = timer.ticks() as i32;
	if new_time-old_time >= 1000{
	    old_time = new_time;

	    let act = Action::Bottom(barre.clone(), &tetris.board);
	    let result_controller = Tetris::check(act);
	    match result_controller {
		ResultController::Ok =>{
		    let old_position: Barre = barre.translate_down(); // piece est mutable dans son scope mais si on lappelle dans une autre fonction, il faut le préciser.
		    background.show(&mut tetris.canvas);
		    barre.draw(&mut tetris.canvas, &old_position);
		    //pour des raisons de perf, present n'est pas appelé dans draw
		    tetris.canvas.present();
		    tetris.canvas.clear();
		    //tetris.canvas.present();
		}
		_ => {}
	    }
	}

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
