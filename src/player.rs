use std::collections::HashMap;

use sdl2::rect::Point;
use sdl2::rect::Rect;

use crate::Board;
use crate::controller::TetrisEvent;
use crate::model::ResultUpdateModel;
use crate::Piece;
use crate::piece::PieceModel;

pub struct Player {
    pub pos_board: Rect,
    //location of the board on the main_window
    pub board: Board,
    pub piece: Piece,
    pub ghost_piece: Piece,
    // The ghost piece printed at the bottom
    init_randomizer: bool,
    pieces: HashMap<u32, Piece>,
    //Lazy initialization of pieces
    //stat: Stat, TODO -> implement score
    //pos_stat: Rect, -> position des stats sur le
    pieces_generated: Vec<String>,
    elapse_time: usize,
    difficulties: Vec<f32>,
    //level of the player
    score: i64, // score of the player
}

impl<'a> Player {
    pub fn score(&self) -> i64 {
        return self.score;
    }

    pub fn new() -> Player {
        Player {
            pos_board: Rect::new(0, 0, 0, 0),
            board: Board::new(),
            piece: Piece::new(),
            ghost_piece: Piece::new(),
            init_randomizer: false,
            pieces: HashMap::new(),
            pieces_generated: vec![],

            //USER SCORES
            elapse_time: 0 as usize,
            difficulties: vec![1.0, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3], // The less the number is, the more the difficulty is
            score: 0,
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
                let result = self.check_bottom(&self.piece);
                if result == ResultUpdateModel::BottomBorder
                    || result == ResultUpdateModel::CollisionPieceBottom
                {
                    self.board.update_board(&self.piece);
                    self.score += 100 * (self.board.get_number_full_lines() as i64);
                    self.board.remove_full_lines();
                    self.re_init_piece();
                }
                if result == ResultUpdateModel::Ok {
                    self.piece.translate_down();
                }
                return result;
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

            TetrisEvent::Space => {
                let mut res = self.update_model(TetrisEvent::Bottom);
                while res == ResultUpdateModel::Ok {
                    res = self.update_model(TetrisEvent::Bottom);
                }
            }

            TetrisEvent::Time(x) => {
                self.elapse_time += x;
                let difficulty = self.difficulty();
                if self.elapse_time > (difficulty * 1000.) as usize {
                    let result = self.check_bottom(&self.piece);
                    if result == ResultUpdateModel::BottomBorder
                        || result == ResultUpdateModel::CollisionPieceBottom
                    {
                        self.board.update_board(&self.piece);
                        self.score += 100 * (self.board.get_number_full_lines() as i64);
                        self.board.remove_full_lines();
                        self.re_init_piece();
                    }
                    if result == ResultUpdateModel::Ok {
                        self.piece.translate_down();
                    }
                    self.elapse_time = 0;
                    return result;
                }
            }
        }
        self.update_ghost_piece();
        ResultUpdateModel::Ok
    }

    //update ghost piece according the current piece
    fn update_ghost_piece(&mut self) {
        let mut copy_piece = self.piece.clone();
        while self.check_bottom(&copy_piece) == ResultUpdateModel::Ok {
            copy_piece.translate_down();
        }
        self.ghost_piece = copy_piece; // update ghost piece
    }

    //check if piece can move to bottom (one movement only)
    fn check_bottom(&self, piece: &Piece) -> ResultUpdateModel {
        for point in piece.data() {
            if point.y + 1 >= crate::HEIGHT {
                return ResultUpdateModel::BottomBorder;
            }
            if self.board.get_case_borrow(point.x, point.y + 1).empty() == false {
                return ResultUpdateModel::CollisionPieceBottom;
            }
        }
        return ResultUpdateModel::Ok;
    }

    fn difficulty(&self) -> f32 {
        let mut difficulty = 0.;
        if self.score >= 0 {
            difficulty = self.difficulties[0];
        }
        if self.score > 500 {
            difficulty = self.difficulties[1];
        }
        if self.score > 1000 {
            difficulty = self.difficulties[2];
        }
        if self.score > 1500 {
            difficulty = self.difficulties[3];
        }
        if self.score > 2000 {
            difficulty = self.difficulties[4];
        }
        difficulty
    }

    fn init_randomizer(&mut self) {
        if self.init_randomizer == false {
            let mut p = Piece::new();
            p.set_data([
                Point::from((4, 0)),
                Point::from((4, 1)),
                Point::from((4, 2)),
                Point::from((4, 3)),
            ]);
            p.set_name(String::from("barre"));
            self.pieces.insert(0, p.clone());

            p.set_name(String::from("square"));
            p.set_data([
                Point::from((4, 0)),
                Point::from((4, 1)),
                Point::from((5, 0)),
                Point::from((5, 1)),
            ]);
            self.pieces.insert(1, p.clone());

            p.set_name(String::from("eclair"));
            p.set_data([
                Point::from((4, 1)),
                Point::from((5, 0)),
                Point::from((6, 0)),
                Point::from((5, 1)),
            ]);
            self.pieces.insert(2, p.clone());

            p.set_name(String::from("coude"));
            p.set_data([
                Point::from((4, 0)),
                Point::from((5, 0)),
                Point::from((6, 0)),
                Point::from((5, 1)),
            ]);
            self.pieces.insert(3, p.clone());
        }
    }

    fn get_random_piece(&mut self) -> Piece {
        self.init_randomizer();
        let mut i: u32 = rand::random();

        let len = self.pieces_generated.len();
        let mut start = 0;
        if len >= 3 {
            start = len - 3;
        }
        while self.pieces_generated[start..len].contains(self.pieces.get(&(i % 4)).unwrap().name())
        {
            i = rand::random();
        }
        self.pieces_generated.insert(
            self.pieces_generated.len(),
            self.pieces.get(&(i % 4)).unwrap().name().to_string(),
        );

        return self.pieces.get(&(i % 4)).unwrap().clone();
    }

    fn re_init_piece(&mut self) {
        let random_piece = self.get_random_piece();
        self.piece.set_data(*random_piece.data());
        self.piece.set_name(random_piece.name().to_string());
        self.piece.set_old_data(*random_piece.old_data());
    }
}
