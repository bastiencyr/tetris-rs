use sdl2::rect::Rect;

use crate::controller::TetrisEvent;
use crate::model::ResultUpdateModel;
use crate::piece::PieceModel;
use crate::Board;
use crate::Piece;

pub struct Player {
    pub pos_board: Rect, //location of the board on the main_window
    pub board: Board,
    pub piece: Piece,
    //stat: Stat, TODO -> implement score
    //pos_stat: Rect, -> position des stats sur le
}

impl<'a> Player {
    pub fn new() -> Player {
        Player {
            pos_board: Rect::new(0, 0, 0, 0),
            board: Board::new(),
            piece: Piece::new(),
        }
    }

    pub fn set_main_player() {}

    pub fn set_player(player_i: i32, number_player: i32) {}

    pub fn update_board(&mut self) {
        for case in self.piece.get_points() {
            (self.board.get_mut_ij(case.x, case.y)).set_empty(false);
        }
    }

    pub fn update_model(&mut self, event: TetrisEvent) -> ResultUpdateModel {
        match event {
            TetrisEvent::Right => {
                //box is a keyword in rust
                for point in self.piece.get_points() {
                    if point.x + 1 >= crate::WIDTH {
                        return ResultUpdateModel::RightBorder;
                    }
                    if self.board.get_i_j(point.x + 1, point.y).empty() == false {
                        return ResultUpdateModel::CollisionPiece;
                    }
                }
                self.piece.translate_right();
            }

            TetrisEvent::Left => {
                //box is a keyword in rust
                for point in self.piece.get_points() {
                    if point.x - 1 < 0 {
                        return ResultUpdateModel::RightBorder;
                    }
                    if self.board.get_i_j(point.x - 1, point.y).empty() == false {
                        return ResultUpdateModel::CollisionPiece;
                    }
                }
                self.piece.translate_left();
            }

            TetrisEvent::Bottom => {
                for point in self.piece.get_points() {
                    if point.y + 1 >= crate::HEIGHT {
                        self.board.update_board(&self.piece);
                        self.piece.reinit();
                        return ResultUpdateModel::BottomBorder;
                    }
                    if self.board.get_i_j(point.x, point.y + 1).empty() == false {
                        self.board.update_board(&self.piece);
                        self.piece.reinit();
                        return ResultUpdateModel::CollisionPieceBottom;
                    }
                }
                self.piece.translate_down();
            }

            TetrisEvent::Up => {
                self.piece.rotate_right();
            }

            _ => {}
        }
        ResultUpdateModel::Ok
    }
}
