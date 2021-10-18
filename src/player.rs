use crate::controller::{ResultController, TetrisEvent};
use crate::piece_gen::PieceModel;
use crate::Board;
use crate::Piece;
use sdl2::rect::Rect;

pub struct Player {
    pub pos_board: Rect, //location of the board on the main_window
    pub board: Board,
    pub piece: Piece,
    //pub background: &'a Background<'a>,
    //stat: Stat, TODO -> implement score
    //pos_stat: Rect, -> position des stats sur le
}

impl<'a> Player {
    pub fn new() -> Player {
        Player {
            pos_board: Rect::new(0, 0, 0, 0),
            board: Board::new(),
            piece: Piece::new(),
            //background: background,
        }
    }

    pub fn set_main_player() {}

    pub fn set_player(player_i: i32, number_player: i32) {}

    pub fn update_board(&mut self) {
        for case in self.piece.get_points() {
            (self.board.get_mut_ij(case.x, case.y)).set_empty(false);
        }
    }

    pub fn update_model(&mut self, event: TetrisEvent) -> ResultController {
        match event {
            TetrisEvent::Right => {
                //box is a keyword in rust
                for point in self.piece.get_points() {
                    if point.x + 1 >= crate::WIDTH {
                        return ResultController::RightBorder;
                    }
                    if self.board.get_i_j(point.x + 1, point.y).empty() == false {
                        return ResultController::CollisionPiece;
                    }
                }
                self.piece.translate_right();
            }

            TetrisEvent::Left => {
                //box is a keyword in rust
                for point in self.piece.get_points() {
                    if point.x - 1 < 0 {
                        return ResultController::RightBorder;
                    }
                    if self.board.get_i_j(point.x - 1, point.y).empty() == false {
                        return ResultController::CollisionPiece;
                    }
                }
                self.piece.translate_left();
            }

            TetrisEvent::Bottom => {
                for point in self.piece.get_points() {
                    if point.y + 1 >= crate::HEIGHT {
                        return ResultController::BottomBorder;
                    }
                    if self.board.get_i_j(point.x, point.y + 1).empty() == false {
                        return ResultController::CollisionPieceBottom;
                    }
                }
                self.piece.translate_down();
            }
            _ => {}
        }
        ResultController::Ok
    }
}
