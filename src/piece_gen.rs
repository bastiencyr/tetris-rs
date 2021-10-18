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
    //décale la pièce vers la droite sans vérification
    fn translate_right(&mut self) -> Self;
    fn translate_left(&mut self) -> Self;
    fn translate_down(&mut self) -> Self;
    //retourne les positions de chaque carré de la pièce
    fn get_points(&self) -> [Point; 4];
    fn get_old_points(&self) -> [Point; 4];
    fn get_random_piece() -> [Point; 4];
}

#[derive(Clone)] // cela nous permet dutiliser le trait de clone -> self.clone()
#[derive(Debug)]
pub struct Piece {
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
            None // on renvoie None pour le dernier élément. Et y a pas une connerie de s'arrêter a lavat dernier comme en c++
        }
    }
}

impl Piece {}

impl PieceModel for Piece {
    fn get_points(&self) -> [Point; 4] {
        return self.data.clone();
    }

    fn get_old_points(&self) -> [Point; 4] {
        return self.old_data.clone();
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

    fn reinit(&mut self) {
        self.data = Piece::get_random_piece();
        self.old_data = self.data;
    }

    fn get_random_piece() -> [Point; 4] {
        let mut pieces = HashMap::new();
        pieces.insert(
            0,
            [
                Point::from((0, 0)),
                Point::from((0, 1)),
                Point::from((0, 2)),
                Point::from((0, 3)),
            ],
        );
        pieces.insert(
            1,
            [
                Point::from((0, 0)),
                Point::from((0, 1)),
                Point::from((1, 0)),
                Point::from((1, 1)),
            ],
        );
        pieces.insert(
            2,
            [
                Point::from((0, 1)),
                Point::from((1, 0)),
                Point::from((2, 0)),
                Point::from((1, 1)),
            ],
        );
        pieces.insert(
            3,
            [
                Point::from((0, 0)),
                Point::from((1, 0)),
                Point::from((2, 0)),
                Point::from((1, 1)),
            ],
        );
        pieces.insert(
            4,
            [
                Point::from((0, 1)),
                Point::from((1, 0)),
                Point::from((2, 0)),
                Point::from((1, 1)),
            ],
        );
        let i: u32 = rand::random();
        return *pieces.get(&(i % 5)).unwrap();
    }

    fn new() -> Self {
        // data piece n'a pas à être mutable -> ne pas confondre le pointeur et les valeurs pointées.
        crate::Piece {
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
}
