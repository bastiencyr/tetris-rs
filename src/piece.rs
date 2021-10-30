extern crate sdl2;

use std::convert::TryInto;

use getset::{Getters, Setters};
use sdl2::rect::Point;

// une pièe doit implémenter un certain nombre de méthode
pub trait PieceModel {
    fn new() -> Self;
    fn get_center(&self) -> Point;
    fn translate_right(&mut self) -> Self;
    fn translate_left(&mut self) -> Self;
    fn translate_down(&mut self) -> Self;
    fn rotate_right(&mut self) -> Piece;

    // SETTERS //
    fn set_name(&mut self, name: String);
    fn set_data(&mut self, data: [Point; 4]);
    fn set_old_data(&mut self, old_data: [Point; 4]);
}

#[derive(Clone)] // cela nous permet dutiliser le trait de clone -> self.clone()
#[derive(Debug)]
#[derive(Getters, Setters)]
pub struct Piece {
    #[getset(get="pub", set="pub")]
    name: String,
    #[getset(get="pub", set="pub")]
    old_data: [Point; 4],
    #[getset(get="pub", set="pub")]
    data: [Point; 4],
    count: i32,
    //color: -> couleur du fond = (R,G,B)
}

//bancal -> à revoir
impl Iterator for Piece {
    //on implémenter litérateur
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.count as usize;
        if self.count < self.data.len().try_into().unwrap() {
            self.count += 1;
            Some(self.data[index].clone())
        } else {
            self.count = 0;
            None // on renvoie None pour le dernier élément.
        }
    }
}

impl Piece {}

impl PieceModel for Piece {
    fn new() -> Self {
        // data piece n'a pas à être mutable -> ne pas confondre le pointeur et les valeurs pointées.
        crate::Piece {
            name: String::from("barre"),
            old_data: [
                Point::from((0, 0)),
                Point::from((0, 1)),
                Point::from((0, 2)),
                Point::from((0, 3)),
            ],
            data: [
                Point::from((0, 0)),
                Point::from((0, 1)),
                Point::from((0, 2)),
                Point::from((0, 3)),
            ],
            count: 0,
        }
    }

    fn get_center(&self) -> Point {
        if self.name() == "eclair" {
            return self.data.get(1).unwrap().clone();
        }
        if self.name() == "barre" {
            return self.data.get(1).unwrap().clone();
        }
        self.data.get(0).unwrap().clone()
    }

    fn translate_right(&mut self) -> Self {
        let copy = self.clone();
        self.old_data = self.data;
        for case in &mut self.data {
            // on met a jour les données
            case.x += 1;
        }
        copy
    }

    fn translate_left(&mut self) -> Self {
        let copy = self.clone();
        self.old_data = self.data;
        //pour linstant on impose pas de conditions
        for case in &mut self.data {
            // on met a jour les données
            case.x -= 1;
        }
        copy
    }

    fn translate_down(&mut self) -> Piece {
        let copy = self.clone();
        self.old_data = self.data;
        //pour linstant on impose pas de conditions
        for case in &mut self.data {
            // on met a jour les données
            case.y += 1;
        }
        copy
    }

    fn rotate_right(&mut self) -> Piece {
        let mut copy = self.clone();
        copy.old_data = copy.data;

        if self.name == String::from("square") {
            return copy;
        }
        let center = self.get_center();
        for case_index in copy.data.iter_mut().enumerate() {
            // on met a jour les données
            let index = case_index.0;
            let case = case_index.1;

            case.x = copy.old_data.get(index).unwrap().y() - center.y() + center.x();
            case.y = center.x() - copy.old_data.get(index).unwrap().x() + center.y();
        }
        //case.x = ;
        copy
    }

    fn set_name(&mut self, name: String) {
        self.set_name(name);
    }

    fn set_data(&mut self, data: [Point; 4]) {
        self.data = data;
    }

    fn set_old_data(&mut self, old_data: [Point; 4]) {
        self.old_data = old_data;
    }
}
