extern crate sdl2;

use sdl2::rect::Point;
use std::collections::HashMap;
use std::convert::TryInto;

// une pièe doit implémenter un certain nombre de méthode
pub trait PieceModel {
    // translateRight modifie la donnée de notre modèle.
    //retourne une nouvelle pièce
    fn new() -> Self;
    fn reinit(&mut self);
    fn get_center(&self) -> Point;
    //décale la pièce vers la droite sans vérification
    fn translate_right(&mut self) -> Self;
    fn translate_left(&mut self) -> Self;
    fn translate_down(&mut self) -> Self;
    fn rotate_right(&mut self) -> Piece;
    fn rotate_left(&mut self) -> Piece;
    //retourne les positions de chaque carré de la pièce
    fn get_points(&self) -> [Point; 4];
    fn get_old_points(&self) -> [Point; 4];
    fn get_random_piece() -> Piece;
}

#[derive(Clone)] // cela nous permet dutiliser le trait de clone -> self.clone()
#[derive(Debug)]
pub struct Piece {
    name: String,
    old_data: [Point; 4],
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

    fn reinit(&mut self) {
        let random_piece = Piece::get_random_piece();
        self.data = random_piece.data;
        self.name = random_piece.name;
        self.old_data = self.data;
    }

    fn get_center(&self) -> Point {
        if self.name == "eclair" {
            return self.data.get(1).unwrap().clone();
        }
        if self.name == "barre" {
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

    fn rotate_left(&mut self) -> Piece {
        todo!()
    }

    fn get_points(&self) -> [Point; 4] {
        return self.data.clone();
    }

    fn get_old_points(&self) -> [Point; 4] {
        return self.old_data.clone();
    }

    fn get_random_piece() -> Piece {
        let mut pieces = HashMap::new();
        let mut p = Piece::new();

        p.data = [
            Point::from((0, 0)),
            Point::from((0, 1)),
            Point::from((0, 2)),
            Point::from((0, 3)),
        ];
        p.name = String::from("barre");
        pieces.insert(0, p.clone());

        p.name = String::from("square");
        p.data = [
            Point::from((0, 0)),
            Point::from((0, 1)),
            Point::from((1, 0)),
            Point::from((1, 1)),
        ];
        pieces.insert(1, p.clone());

        p.name = String::from("eclair");
        p.data = [
            Point::from((0, 1)),
            Point::from((1, 0)),
            Point::from((2, 0)),
            Point::from((1, 1)),
        ];
        pieces.insert(2, p.clone());

        p.name = String::from("coude");
        p.data = [
            Point::from((0, 0)),
            Point::from((1, 0)),
            Point::from((2, 0)),
            Point::from((1, 1)),
        ];
        pieces.insert(3, p.clone());

        let i: u32 = rand::random();
        return pieces.get(&(i % 4)).unwrap().clone();
    }
}
