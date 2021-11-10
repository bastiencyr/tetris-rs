use std::borrow::Borrow;

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use ui::background::Background;

use crate::model::{Model, TetrisModel};
use crate::view::{TetrisView, View};

//Contient le controlleur

pub enum TetrisEvent {
    Right,
    Left,
    Bottom,
    Up,
    Space,
    Time(usize),
}

// Un controlleur contient deux méthodes : update_view et
pub trait TraitController {
    fn update_view(&mut self);
    fn update_model(&mut self, event: TetrisEvent);
}

// Notre controller générique
pub struct Controller <'a>{
    pub view2: Vec<Box<dyn View + 'a>>,
    //pub view: T,
    pub model: TetrisModel,
}

//not the right way to initialize a controller. The view and the model must be registered to
//the controller
impl Controller<'_> {
    pub fn new<'a>(
        canvas: Canvas<Window>,
        texture: Texture<'a>,
        background: Background<'a>,
    ) -> Controller<'a> {
        Controller {
            view2: vec![
                Box::new(TetrisView::new(canvas, texture, background))
            ],
            model: TetrisModel::new(),
        }
    }
}

// on implemente le trait controller complet
impl TraitController for Controller <'_> {
    fn update_view(&mut self) {
        for view in self.view2.iter_mut() {
            view.update_v(self.model.borrow());
        }
    }

    fn update_model(&mut self, event: TetrisEvent) {
        self.model.update_m(event);
        self.update_view();
    }
}
