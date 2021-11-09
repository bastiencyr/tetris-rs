use crate::controller::TetrisEvent;
use crate::player;
use crate::player::Player;

pub trait Model {
    fn update_m(self: &mut Self, event: TetrisEvent);
    fn get_model(self: &Self) -> &TetrisModel;
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
}

impl Model for TetrisModel {
    fn update_m(self: &mut Self, event: TetrisEvent) {
        // la logique du modèle est implémentée dans un autre fichier
        self.player[0].update_model(event);
    }

    fn get_model(self: &Self) -> &TetrisModel {
        self
    }
}

//Les résultats de la demande dun utilisateur. Le temps est aussi un "utilisateur"
#[derive(PartialEq, Eq)]
pub enum ResultUpdateModel {
    RightBorder,
    LeftBorder,
    BottomBorder,
    Ok,
    CollisionPiece,
    CollisionPieceBottom,
}
