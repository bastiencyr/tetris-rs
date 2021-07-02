use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::Controller;
use std::convert::TryInto;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Tetris <'a>{
    pub canvas: Canvas<Window>,
    pub board: Board, // contient le plateau de jeu
    w: i32,
    h: i32,
    x: i32,
    y: i32,
    size_box: i32,
    pub main_texture: Texture<'a>,
}

impl Tetris<'_>{
    pub fn new(canvas: Canvas<Window>, texture: Texture, w: i32, h: i32, x: i32, y: i32) -> Tetris{
	Tetris{
	    canvas: canvas,
	    w:300,
	    h:600,
	    x:100,
	    y:200,
	    board: Board::new(),
	    size_box: h/20,
	    main_texture: texture,
	}
    }
    pub fn draw <T: crate::PieceGen>(&mut self, old_piece: &T, new_piece: &T){
	self.canvas.with_texture_canvas(&mut self.main_texture, |texture_p| {
	    texture_p.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
	    for case in old_piece.get_points(){
		let x  = case.x() as i32 * 30;
		let y = case.y() as i32 * 30;
		texture_p.fill_rect(Rect::from((x, y, 28, 28))).expect("Rectange pas dessinable");
	    }
	    
	    texture_p.set_draw_color(sdl2::pixels::Color::RGBA(63, 63, 63, 255));
	    for case in new_piece.get_points()
	    {
		let x  = case.x() as i32 * 30;
		let y = case.y() as i32 * 30;
		texture_p.fill_rect(Rect::from((x, y, 28, 28))).expect("Rectange pas dessinable");
	    }
	    texture_p.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
	}
	);
	//self.canvas.copy(&self.main_texture, None, None).expect("Cant copy");
    }
}

impl <'a> Controller for Tetris <'a>{}


//notre jeu sur lequel un joueur peut évoluer
//encore une fois, on va faire bcp dabstraction. Une instance de tetris
//ne pourra pa accéder simplement à son board. Il faudra demander si
//telle pièce peut aller a droite... De plus on va créer un itérateur pour
//ajouter de labstaction
// un board est un vecteur de case. Chaque case étant une structure. 
// board
#[derive (Clone)]
pub struct Board{
    grid: Vec<Box>, //grid is a  vector of boxes. Each box represents in the tetris.
    count: i32,
}

impl Board{
    pub fn new()->Board{
	let mut grid = Vec::new();
	for i in 0..(crate::WIDTH*crate::HEIGHT).try_into().unwrap(){
	    grid.push(Box::new(i/crate::WIDTH, i/crate::WIDTH))
	}
	Board{
	    grid: grid,
	    count: 0,
	}
    }
}
//a box represent a square object in the board
#[derive (Clone)]
pub struct Box{
    coord: Point,
    empty: bool,
}

impl Box{
    fn new(i: i32, j: i32)->Box{
	Box {
	    coord: Point::new(i,j),
	    empty: true,
	}
    }
}

//bancal -> à revoir
impl Iterator for Board { //on implémenter litérateur
    type Item = Box;

    fn next(&mut self) -> Option<Self::Item> {
	let index = self.count as usize;
        if self.count < self.grid.len().try_into().unwrap() {
            self.count += 1;
            Some(self.grid[index].clone())
        } else {
	    self.count = 0;
            None // on renvoie None pour le dernier élément. Et y a pas une connerie de s'arrêter a lavat dernier comme en c++
        }
    }
}
