use std::collections::HashMap;

use sdl2::rect::Point;
use sdl2::rect::Rect;

use crate::Board;
use crate::controller::TetrisEvent;
use crate::model::ResultUpdateModel;
use crate::Piece;
use crate::piece::PieceModel;

pub struct Player {
    pub pos_board: Rect, //location of the board on the main_window
    pub board: Board,
    pub piece: Piece,
    init_randomizer: bool,
    pieces: HashMap<u32, Piece>, //Lazy initialization of pieces
                                 //stat: Stat, TODO -> implement score
                                 //pos_stat: Rect, -> position des stats sur le
}

impl<'a> Player {
    pub fn new() -> Player {
        Player {
            pos_board: Rect::new(0, 0, 0, 0),
            board: Board::new(),
            piece: Piece::new(),
            init_randomizer: false,
            pieces: HashMap::new(),
        }
    }

    pub fn set_main_player() {}

    pub fn set_player(player_i: i32, number_player: i32) {}

    pub fn update_board(&mut self) {
        for case in self.piece.data() {
            (self.board.get_case_borrow_mut(case.x, case.y)).set_empty(false);
        }
    }

    pub fn update_model(&mut self, event: TetrisEvent) -> ResultUpdateModel {
        match event {
            TetrisEvent::Right => {
                //box is a keyword in rust
                for point in self.piece.data() {
                    if point.x + 1 >= crate::WIDTH {
                        return ResultUpdateModel::RightBorder;
                    }
                    if self.board.get_case_borrow(point.x + 1, point.y).empty() == false {
                        return ResultUpdateModel::CollisionPiece;
                    }
                }
                self.piece.translate_right();
            }

            TetrisEvent::Left => {
                //box is a keyword in rust
                for point in self.piece.data() {
                    if point.x - 1 < 0 {
                        return ResultUpdateModel::RightBorder;
                    }
                    if self.board.get_case_borrow(point.x - 1, point.y).empty() == false {
                        return ResultUpdateModel::CollisionPiece;
                    }
                }
                self.piece.translate_left();
            }

            TetrisEvent::Bottom => {
                for point in self.piece.data() {
                    if point.y + 1 >= crate::HEIGHT {
                        self.board.update_board(&self.piece);
                        self.re_init_piece();
                        return ResultUpdateModel::BottomBorder;
                    }
                    if self.board.get_case_borrow(point.x, point.y + 1).empty() == false {
                        self.board.update_board(&self.piece);
                        self.re_init_piece();
                        return ResultUpdateModel::CollisionPieceBottom;
                    }
                }
                self.piece.translate_down();
            }

            TetrisEvent::Up => {
                let rotate_piece = self.piece.rotate_right();
                for point in rotate_piece.data() {
                    if point.x < 0 {
                        return ResultUpdateModel::LeftBorder;
                    }
                    if point.x >= crate::WIDTH {
                        return ResultUpdateModel::RightBorder;
                    }
                    if point.y >= crate::HEIGHT {
                        return ResultUpdateModel::BottomBorder;
                    }
                    if point.y < 0 {
                        return ResultUpdateModel::BottomBorder;
                    }
                    if self.board.get_case_borrow(point.x, point.y).empty() == false {
                        return ResultUpdateModel::CollisionPiece;
                    }
                }
                self.piece = rotate_piece;
            }

            _ => {}
        }
        ResultUpdateModel::Ok
    }

    fn init_randomizer(&mut self) {
        if self.init_randomizer == false {
            let mut p = Piece::new();
            p.set_data([
                Point::from((0, 0)),
                Point::from((0, 1)),
                Point::from((0, 2)),
                Point::from((0, 3)),
            ]);
            p.set_name(String::from("barre"));
            self.pieces.insert(0, p.clone());

            p.set_name(String::from("square"));
            p.set_data([
                Point::from((0, 0)),
                Point::from((0, 1)),
                Point::from((1, 0)),
                Point::from((1, 1)),
            ]);
            self.pieces.insert(1, p.clone());

            p.set_name(String::from("eclair"));
            p.set_data([
                Point::from((0, 1)),
                Point::from((1, 0)),
                Point::from((2, 0)),
                Point::from((1, 1)),
            ]);
            self.pieces.insert(2, p.clone());

            p.set_name(String::from("coude"));
            p.set_data([
                Point::from((0, 0)),
                Point::from((1, 0)),
                Point::from((2, 0)),
                Point::from((1, 1)),
            ]);
            self.pieces.insert(3, p.clone());
        }
    }

    fn get_random_piece(&mut self) -> Piece {
        self.init_randomizer();
        let i: u32 = rand::random();
        return self.pieces.get(&(i % 4)).unwrap().clone();
    }

    fn re_init_piece(&mut self) {
        let random_piece = self.get_random_piece();
        self.piece.set_data(*random_piece.data());
        self.piece.set_name(random_piece.name().to_string());
        self.piece.set_old_data(*random_piece.old_data());
    }
}
