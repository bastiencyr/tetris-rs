use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::Controller;
use std::convert::TryInto;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::piece_gen;


//Tetris is the "real" controller because it controls the view by calling draw
//and the model by calling check (see implementation of Controller trait bellow
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

    pub fn update_board<T:crate::PieceModel>(&mut self, piece: &T){
	self.board.update_board(piece);
    }
    
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
    pub fn draw <T: piece_gen::PieceView + piece_gen::PieceModel>(&mut self, old_piece: &T, new_piece: &T){
	 new_piece.draw(old_piece, &mut self.canvas, &mut self.main_texture);
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

#[derive(Debug)]
#[derive (Clone)]
pub struct Board{
    grid: Vec<Box>, //grid is a  vector of boxes. Each box represents in the tetris.
    count: i32,
}

impl Board{
    pub fn new()->Board{
	let mut grid = Vec::new();
	for i in 0..(crate::WIDTH*crate::HEIGHT).try_into().unwrap(){
	    grid.push(Box::new(i%crate::WIDTH, i/crate::WIDTH))
	}
	Board{
	    grid: grid,
	    count: 0,
	}
    }
    
    pub fn get_i_j(&self, i: i32, j:i32) -> &Box{
	let index = j * crate::WIDTH + i;
	&self.grid[index as usize]
    }

      pub fn get_mut_ij(&mut self, i: i32, j:i32) -> &mut Box{
	let index = j * crate::WIDTH + i;
	&mut self.grid[index as usize]
    }
    
    pub fn update_board<T: crate::PieceModel>(&mut self, piece: &T){
	for case in piece.get_points(){
	    self.get_mut_ij(case.x, case.y).empty = false;
	}
	//println!("{:#?}", self.grid);
	//self.print_mat();
    }
    
    pub fn print_mat(&self){
	for case in &self.grid{
	    if case.newline(){
		print!("\n");
	    }
	    if case.empty(){
		print!("-{}-", 0);
	    }
	    else{
		print!("-{}-", 1);
	    }
	}
	print!("\n");
    }
}
//a box represent a square object in the board
#[derive(Debug)]
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

    pub fn empty(&self) -> bool{
	self.empty
    }
    
    pub fn newline(&self) -> bool{
	if self.coord.x == 0 {
	    return true;
	}
	return false;
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
