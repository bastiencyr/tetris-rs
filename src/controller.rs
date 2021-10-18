use crate::model::{Model, TetrisModel};
use crate::tetris::Board;
use crate::view::{TetrisView, View};
use crate::PieceModel;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use ui::background::Background;

//Contient le controlleur

pub enum TetrisEvent {
    Right,
    Left,
    Bottom,
    Up,
    Space,
    Help,
}

//Les résultats de la demande dun utilisateur. Le temps est aussi un "utilisateur"
pub enum ResultController {
    RightBorder,
    LeftBorder,
    BottomBorder,
    Ok,
    CollisionPiece,
    CollisionPieceBottom,
}

//les différentes actions possibles d'un joueur sur un jeu.
pub enum Action<'a, T, Board>
where
    T: PieceModel,
{
    Right(T, &'a Board),
    Bottom(T, &'a Board),
    Left(T, &'a Board),
}


// Un controlleur contient deux méthodes : update_view et
pub trait TraitController {
    fn update_view(&mut self);
    fn update_model(&mut self, event: TetrisEvent);
}

// Notre controller concret
pub struct Controller2<'a> {
    pub view: TetrisView<'a>,
    pub model: TetrisModel,
}

impl Controller2<'_> {
    pub fn new<'a>(
        canvas: Canvas<Window>,
        texture: Texture<'a>,
        background: Background<'a>,
    ) -> Controller2<'a> {
        Controller2 {
            view: TetrisView::new(canvas, texture, background),
            model: TetrisModel::new(),
        }
    }
}

// on implemente le trait controller complet
impl TraitController for Controller2<'_> {
    fn update_view(&mut self) {
        self.view.update_v(&mut self.model);
    }

    fn update_model(&mut self, event: TetrisEvent) {
        self.model.update_m(event);
        self.update_view();
    }
}
