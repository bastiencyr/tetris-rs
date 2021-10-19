use crate::model::{Model, TetrisModel};
use crate::view::{TetrisView, View};
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

// Un controlleur contient deux méthodes : update_view et
pub trait TraitController {
    fn update_view(&mut self);
    fn update_model(&mut self, event: TetrisEvent);
}

// Notre controller concret
pub struct Controller<'a> {
    pub view: TetrisView<'a>,
    pub model: TetrisModel,
}

impl Controller<'_> {
    pub fn new<'a>(
        canvas: Canvas<Window>,
        texture: Texture<'a>,
        background: Background<'a>,
    ) -> Controller<'a> {
        Controller {
            view: TetrisView::new(canvas, texture, background),
            model: TetrisModel::new(),
        }
    }
}

// on implemente le trait controller complet
impl TraitController for Controller<'_> {
    fn update_view(&mut self) {
        self.view.update_v(&mut self.model);
    }

    fn update_model(&mut self, event: TetrisEvent) {
        self.model.update_m(event);
        self.update_view();
    }
}
