use std::borrow::Borrow;
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

// Notre controller générique
pub struct Controller<T: View> {
    pub view: T,
    pub model: TetrisModel,
}

impl Controller<TetrisView <'_> > {
    pub fn new<'a>(
        canvas: Canvas<Window>,
        texture: Texture<'a>,
        background: Background<'a>,
    ) -> Controller<TetrisView<'a>> {
        Controller {
            view: TetrisView::new(canvas, texture, background),
            model: TetrisModel::new(),
        }
    }
}

// on implemente le trait controller complet
impl TraitController for Controller<TetrisView <'_>> {
    fn update_view(&mut self) {
        self.view.update_v(self.model.borrow());
    }

    fn update_model(&mut self, event: TetrisEvent) {
        self.model.update_m(event);
        self.update_view();
    }
}
