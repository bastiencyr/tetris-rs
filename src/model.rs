use crate::controller::TetrisEvent;
use crate::piece::Piece;
use crate::player;
use crate::player::Player;

pub trait Model {
    fn update_m(self: &mut Self, event: TetrisEvent);
    fn get_piece(self: &Self) -> &Piece;
}

pub struct TetrisModel {
    pub player: Vec<player::Player>,
}

impl TetrisModel {
    pub fn new<'a>() -> TetrisModel {
        TetrisModel {
            player: vec![Player::new()],
        }
    }

    pub fn update_board<T: crate::PieceModel>(&mut self, piece: &T) {
        self.player[0].board.update_board(piece);
    }
}

impl Model for TetrisModel {
    fn update_m(self: &mut Self, event: TetrisEvent) {
        // la logique du modèle est implémentée dans un autre fichier
        self.player[0].update_model(event);
    }

    fn get_piece(self: &Self) -> &Piece {
        return &self.player[0].piece;
    }
}

//Les résultats de la demande dun utilisateur. Le temps est aussi un "utilisateur"
pub enum ResultUpdateModel {
    RightBorder,
    LeftBorder,
    BottomBorder,
    Ok,
    CollisionPiece,
    CollisionPieceBottom,
}
