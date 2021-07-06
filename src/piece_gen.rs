extern crate sdl2;

use sdl2::rect::Point;
use sdl2::rect::Rect;

use sdl2::render::Canvas;
use sdl2::video::Window;
use std::convert::TryInto;
use std::collections::HashMap;
use sdl2::render;

// une pièe doit implémenter un certain nombre de méthode
pub trait PieceModel{
    // translateRight modifie la donnée de notre modèle.
    //retourne une nouvelle pièce
    fn new() -> Self;
    fn reinit(&mut self);
    //décale la pièce vers la droite sans vérification
    fn translate_right(&mut self) -> Self;
    fn translate_left(&mut self) -> Self;
    fn translate_down(&mut self) -> Self;
    //retourne les positions de chaque carré de la pièce
    fn get_points(&self) -> [Point;4];
    fn get_random_piece()->[Point;4];
}

pub trait PieceView {
    fn draw <T: PieceModel>(&self, old_piece: &T, canvas: &mut render::Canvas<Window>, texture: &mut render::Texture);
}

#[derive(Clone)] // cela nous permet dutiliser le trait de clone -> self.clone()
pub struct Piece{
    data: [Point;4],
    count: i32,
}

//bancal -> à revoir
impl Iterator for Piece { //on implémenter litérateur
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



impl Piece{
}

impl PieceModel for Piece{

    fn get_points(&self) -> [Point;4]{
	return self.data.clone();
    }

    fn translate_right(&mut self) -> Self {
	let copy = self.clone();
	for case in &mut self.data{
	    // on met a jour les données
	    case.x += 1;
    	}
    	copy
    }

    fn reinit(&mut self){
	self.data = Piece::get_random_piece();
    }

    fn get_random_piece()->[Point;4]{	
	let mut pieces = HashMap::new();
	pieces.insert(0,  [Point::from((0,0)),Point::from((0,1)),Point::from((0,2)),Point::from((0,3))]);
	pieces.insert(1,  [Point::from((0,0)),Point::from((0,1)),Point::from((1,0)),Point::from((1,1))]);
	pieces.insert(2,  [Point::from((0,1)),Point::from((1,0)),Point::from((2,0)),Point::from((1,1))]);
	pieces.insert(3,  [Point::from((0,0)),Point::from((1,0)),Point::from((2,0)),Point::from((1,1))]);
	pieces.insert(4,  [Point::from((0,1)),Point::from((1,0)),Point::from((2,0)),Point::from((1,1))]);
	let i: u32 = rand::random();
	return *pieces.get(&(i%5)).unwrap();
    }
    
    fn new()-> Self{
	// data piece n'a pas à être mutable -> ne pas confondre le pointeur et les valeurs pointées. 
	crate::Piece{
	    data:[Point::from((0,0)),Point::from((0,1)),Point::from((0,2)),Point::from((0,3))],
	    count: 0,
	}
    }
    
    fn translate_left(&mut self)->Self{
	let copy = self.clone();
	//pour linstant on impose pas de conditions
	for case in &mut self.data{
	    // on met a jour les données
	    case.x -= 1;
	}
	copy
    }

    fn translate_down(&mut self) -> Piece{
	let copy = self.clone();
	//pour linstant on impose pas de conditions
	for case in &mut self.data{
	    // on met a jour les données
	    case.y += 1;	
	}
	copy
    }
}


impl PieceView for Piece{
    fn draw <T: PieceModel>(&self, old_piece: &T, canvas: &mut render::Canvas<Window>, texture: &mut render::Texture){

	canvas.with_texture_canvas(texture, |texture_p| {
	    texture_p.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
	    for case in old_piece.get_points(){
		let x  = case.x() as i32 * 30;
		let y = case.y() as i32 * 30;
		texture_p.fill_rect(Rect::from((x, y, 28, 28))).expect("Rectange pas dessinable");
	    }
	    
	    texture_p.set_draw_color(sdl2::pixels::Color::RGBA(63, 63, 63, 255));
	    for case in self.get_points()
	    {
		let x  = case.x() as i32 * 30;
		let y = case.y() as i32 * 30;
		texture_p.fill_rect(Rect::from((x, y, 28, 28))).expect("Rectange pas dessinable");
	    }
	    texture_p.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
	}
	);
	//self.canvas.copy(&self.main_texture, None, None).expect("Cant copy");
    }
}
