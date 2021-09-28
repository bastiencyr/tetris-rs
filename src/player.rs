use crate::piece_gen;
use crate::piece_gen::PieceModel;
use crate::piece_gen::PieceView;
use crate::Board;
use crate::Controller;
use crate::Piece;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;
use std::convert::TryInto;
use ui::background::Background;

pub struct Player<'a> {
    pub pos_board: Rect, //location of the board on the main_window
    pub board: Board,
    pub piece: Piece,
    pub background: &'a Background<'a>,
    //stat: Stat, TODO -> implement score
    //pos_stat: Rect, -> position des stats sur le
}

impl<'a> Player<'_> {
    pub fn new(background: &'a Background<'a>) -> Player<'a> {
        Player {
            pos_board: Rect::new(0, 0, 0, 0),
            board: Board::new(),
            piece: Piece::new(),
            background: background,
        }
    }

    pub fn set_main_player() {}

    pub fn set_player(player_i: i32, number_player: i32) {}

    pub fn draw_piece(&self, canvas: &mut render::Canvas<Window>, texture: &mut render::Texture) {
        //on redessine le fond
        for pt in self.piece.get_old_points() {
            let rect = Rect::new(pt.x * 30, pt.y * 30, 28, 28);
            let result = canvas.with_texture_canvas(texture, |texture_canvas| {
                texture_canvas
                    .copy(&self.background.background_texture, rect, rect)
                    .expect("Cant copy");
            });
            canvas.copy(&texture, rect, rect).expect("Cant copy");
        }

        self.piece.draw(canvas, texture);
        // on copie uniquement les pièces sur notre texture.
        for pt in self.piece.get_points() {
            //println!("pt x, {}, pt y: {}", pt.x, pt.y);
            let rect = Rect::new(pt.x * 30, pt.y * 30, 28, 28);
            canvas.copy(&texture, rect, rect).expect("Cant copy");
        }
    }

    pub fn draw_back(&self, canvas: &mut render::Canvas<Window>, texture: &mut render::Texture) {
        self.piece.draw_back(canvas, texture);
    }

    //fn erase_piece ; TODO -> effacer une pièce du jeu

    pub fn update_board(&mut self) {
        for case in self.piece.get_points() {
            (self.board.get_mut_ij(case.x, case.y)).set_empty(false);
        }
    }
}
