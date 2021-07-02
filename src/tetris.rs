use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::Controller;
use std::convert::TryInto;
use sdl2::rect::Point;

pub struct Tetris {
    pub canvas: Canvas<Window>,
    pub board: Board, // contient le plateau de jeu
    w: i32,
    h: i32,
    x: i32,
    y: i32,
    size_box: i32,
}

impl Tetris{
    pub fn new(canvas: Canvas<Window>, w: i32, h: i32, x: i32, y: i32) -> Tetris{
	Tetris{
	    canvas: canvas,
	    w:300,
	    h:600,
	    x:100,
	    y:200,
	    board: Board::new(),
	    size_box: h/20,
	}
    }
}

impl Controller for Tetris{}


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
