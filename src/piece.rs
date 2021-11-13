extern crate sdl2;

use std::convert::TryInto;
use std::fmt;

use getset::{Getters, Setters};
use sdl2::rect::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceName {
    Barre,
    Square,
    Eclair,
    Te,
    Coude,
}

impl fmt::Display for PieceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PieceName::Barre => write!(f, "barre"),
            PieceName::Square => write!(f, "square"),
            PieceName::Eclair => write!(f, "eclair"),
            PieceName::Te => write!(f, "te"),
            PieceName::Coude => write!(f, "coude"),
        }
    }
}

// une pièe doit implémenter un certain nombre de méthode
pub trait PieceModel {
    fn new() -> Self;
    fn get_center(&self) -> Point;
    fn translate_right(&mut self) -> Self;
    fn translate_left(&mut self) -> Self;
    fn translate_down(&mut self) -> Self;
    fn rotate_right(&self) -> Piece;
}

#[derive(Clone)] // cela nous permet dutiliser le trait de clone -> self.clone()
#[derive(Debug, Getters, Setters)]
pub struct Piece {
    #[getset(get = "pub", set = "pub")]
    name: PieceName,
    #[getset(get = "pub", set = "pub")]
    data: [Point; 4],
    count: i32,
}

impl Piece {}

impl PieceModel for Piece {
    fn new() -> Self {
        // data piece n'a pas à être mutable -> ne pas confondre le pointeur et les valeurs pointées.
        crate::Piece {
            name: PieceName::Barre,
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
        return match self.name {
            PieceName::Eclair => self.data.get(1).unwrap().clone(),
            PieceName::Barre => self.data.get(1).unwrap().clone(),
            PieceName::Te => self.data.get(1).unwrap().clone(),
            PieceName::Coude => self.data.get(1).unwrap().clone(),
            _ => self.data.get(0).unwrap().clone(),
        };
    }

    fn translate_right(&mut self) -> Self {
        let copy = self.clone();
        for case in &mut self.data {
            case.x += 1;
        }
        copy
    }

    fn translate_left(&mut self) -> Self {
        let copy = self.clone();
        for case in &mut self.data {
            case.x -= 1;
        }
        copy
    }

    fn translate_down(&mut self) -> Piece {
        let copy = self.clone();
        for case in &mut self.data {
            case.y += 1;
        }
        copy
    }

    fn rotate_right(&self) -> Piece {
        let mut copy = self.clone();
        let old_data = copy.data;

        //Square doesn't need rotation
        if self.name == PieceName::Square {
            return copy;
        }
        let center = self.get_center();
        for case_index in copy.data.iter_mut().enumerate() {
            let index = case_index.0;
            let case = case_index.1;

            case.x = old_data.get(index).unwrap().y() - center.y() + center.x();
            case.y = center.x() - old_data.get(index).unwrap().x() + center.y();
        }
        copy
    }
}

pub struct IntoIterPiece {
    piece: Piece,
    count: usize,
}

pub struct IterPiece<'a> {
    piece: &'a Piece,
    count: usize,
}

impl Iterator for IterPiece<'_> {
    type Item = (Point, PieceName); //

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.count;
        if index < self.piece.data.len().try_into().unwrap() {
            self.count += 1;
            Some((self.piece.data[index], self.piece.name))
        } else {
            None
        }
    }
}

impl Iterator for IntoIterPiece {
    type Item = (Point, PieceName); //

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.count;
        if index < self.piece.data.len().try_into().unwrap() {
            self.count += 1;
            Some((self.piece.data[index], self.piece.name))
        } else {
            None
        }
    }
}

impl IntoIterator for Piece {
    type Item = <IntoIterPiece as Iterator>::Item;
    type IntoIter = IntoIterPiece;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterPiece {
            piece: self,
            count: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Piece {
    type Item = <IterPiece<'a> as Iterator>::Item;
    type IntoIter = IterPiece<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterPiece {
            piece: &self,
            count: 0,
        }
    }
}
