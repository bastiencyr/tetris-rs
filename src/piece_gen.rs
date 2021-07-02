extern crate sdl2;

use sdl2::rect::Point;
use sdl2::rect::Rect;

use sdl2::render::Canvas;
use sdl2::video::Window;
use std::convert::TryInto;

// une pièe doit implémenter un certain nombre de méthode
pub trait PieceGen{
    // translateRight modifie la donnée de notre modèle.
    //retourne une nouvelle pièce
    fn new() -> Self;

    //décale la pièce vers la droite sans vérification
    fn translate_right(&mut self) -> Self;
    fn translate_left(&mut self);
    fn translate_down(&mut self) -> Self;
    // chaque pièce doit implémenter draw. Draw permet de mettre à jour la vue
    //canvas.present n'est pas appelé
    fn draw(&self, canvas: &mut Canvas<Window>, old_pos: &Self);
    //retourne les positions de chaque carré de la pièce
    fn get_points(&self) -> [Point;4];
}

#[derive(Clone)] // cela nous permet dutiliser le trait de clone -> self.clone()
pub struct Barre{
    data: [Point;4],
    count: i32,
}

//bancal -> à revoir
impl Iterator for Barre { //on implémenter litérateur
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



impl Barre{
}

impl PieceGen for Barre{

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

    fn new()-> Self{
	// data piece n'a pas à être mutable -> ne pas confondre le pointeur et les valeurs pointées. 
	crate::Barre{
	    data:[Point::from((0,0)),Point::from((0,1)),Point::from((0,2)),Point::from((0,3))],
	    count: 0,
	}
    }
    
    fn translate_left(&mut self){
	//pour linstant on impose pas de conditions
	for case in &mut self.data{
	    // on met a jour les données
	    case.x -= 1;
	}
    }

    fn translate_down(&mut self) -> Barre{
	let copy = self.clone();
	//pour linstant on impose pas de conditions
	for case in &mut self.data{
	    // on met a jour les données
	    case.y += 1;	
	}
	copy
    }
    
    fn draw(&self, canvas : &mut Canvas<Window>, barre: &Barre){

	//on efface lanncienne pièce
	canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
	for case in &barre.data{
	    let x  = case.x() as i32 * 30;
	    let y = case.y() as i32 * 30;
	    canvas.fill_rect(Rect::from((x, y, 28, 28))).expect("Rectange pas dessinable");
	}
	
	canvas.set_draw_color(sdl2::pixels::Color::RGBA(63, 63, 63, 255));
	for case in &self.data{
	    let x  = case.x() as i32 * 30;
	    let y = case.y() as i32 * 30;
	    canvas.fill_rect(Rect::from((x, y, 28, 28))).expect("Rectange pas dessinable");
	}
	canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
    }
}
