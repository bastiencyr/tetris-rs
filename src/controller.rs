use std::borrow::Borrow;

use crate::model::{Model, TetrisModel};
use crate::view::View;

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
pub struct Controller<'a> {
    pub view2: Vec<Box<dyn View + 'a>>,
    //pub view: T,
    pub model: TetrisModel,
}

impl<'a> Controller<'a> {
    pub fn new() -> Controller<'a> {
        Controller {
            view2: vec![],
            model: TetrisModel::init(),
        }
    }

    pub fn set_model(&mut self, model: TetrisModel) {
        self.model = model;
    }

    // 'a MUST be given here
    pub fn register_view(&mut self, view: Box<dyn View + 'a>) -> usize {
        self.view2.append(&mut vec![view]);
        self.view2.len() - 1
    }
}

// on implemente le trait controller complet
impl TraitController for Controller<'_> {
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
