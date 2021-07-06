use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::Controller;
use std::convert::TryInto;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::piece_gen;
use crate::Board;
use crate::Piece;
use crate::piece_gen::PieceModel;
use sdl2::render;
use crate::piece_gen::PieceView;

pub struct Player{
    pub pos_board: Rect, //location of the board on the main_window
    pub board: Board,
    pub piece: Piece,
    //stat: Stat, TODO -> implement score
    //pos_stat: Rect, -> position des stats sur le 
}

impl Player{

    pub fn new() -> Player{
	Player{
	    pos_board: Rect::new(0,0,0,0),
	    board: Board::new(),
	    piece: Piece::new(),
	}
    }
    
    pub fn set_main_player(){
    }

    pub fn set_player(player_i : i32, number_player : i32){
    }

    pub fn draw_piece (&self, old_piece: Piece, canvas: &mut render::Canvas<Window>, texture: &mut render::Texture){
	self.piece.draw(&old_piece, canvas, texture);
    }

    //fn erase_piece ; TODO -> effacer une pièce du jeu
}
