use std::borrow::BorrowMut;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use ui::background::Background;

use crate::model::{Model, TetrisModel};
use crate::piece::Piece;

pub trait View {
    fn update_v(self: &mut Self, model: &TetrisModel);
}

pub struct TetrisView<'a> {
    pub main_texture: Texture<'a>,
    pub canvas: Canvas<Window>,
    pub background: Background<'a>,
}

impl TetrisView<'_> {
    pub fn new<'a>(
        canvas: Canvas<Window>,
        texture: Texture<'a>,
        background: Background<'a>,
    ) -> TetrisView<'a> {
        TetrisView {
            canvas,
            main_texture: texture,
            background,
        }
    }

    pub fn draw_board(&mut self, model: &TetrisModel) {
        let texture_creator = self.canvas.texture_creator();
        //TODO its juste a workaround. I make a copy of background
        let back_workaround = Background::new(&mut self.canvas, &texture_creator);

        self.canvas
            .with_texture_canvas(self.main_texture.borrow_mut(), |texture_canvas| {

                //on recopie le fond
                texture_canvas
                    .copy(&back_workaround.background_texture, None, None)
                    .expect("Cant copy");

                // on recopie les pièces déjà présente
                texture_canvas.set_draw_color(Color::RGBA(63, 63, 63, 255));

                //we use our iterator here
                for case in &model.get_model().player[0].board {
                    if case.empty() == false {
                        let rect = Rect::new(case.x() * 30, case.y() * 30, 28, 28);
                        texture_canvas
                            .fill_rect(rect);
                    }
                }

                // on recopie la pièce qui tombait
                for case in model.get_model().player[0].piece.data() {
                    let x = case.x() as i32 * 30;
                    let y = case.y() as i32 * 30;
                    texture_canvas
                        .fill_rect(Rect::from((x, y, 28, 28)))
                        .expect("Rectange pas dessinable");
                }

                texture_canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
            });
    }

    pub fn draw_piece(&mut self, piece: &Piece) {
        //on redessine le fond
        let texture_creator = self.canvas.texture_creator();
        //TODO its juste a workaround. I make a copy of background
        let back_workaround = Background::new(&mut self.canvas, &texture_creator);
        for pt in piece.old_data() {
            let rect = Rect::new(pt.x * 30, pt.y * 30, 28, 28);
            self.canvas
                .with_texture_canvas(self.main_texture.borrow_mut(), |texture_canvas| {
                    texture_canvas
                        .copy(&back_workaround.background_texture, rect, rect)
                        .expect("Cant copy");
                });
            self.canvas
                .copy(&self.main_texture, rect, rect)
                .expect("Cant copy");
        }

        self.draw(&piece);
        // on copie uniquement les pièces sur notre texture.
        for pt in piece.data() {
            //println!("pt x, {}, pt y: {}", pt.x, pt.y);
            let rect = Rect::new(pt.x * 30, pt.y * 30, 28, 28);
            self.canvas
                .copy(&self.main_texture, rect, rect)
                .expect("Cant copy");
        }
    }

    fn draw(&mut self, piece: &Piece) {
        //println!("{:#?}::\n{:#?}", old_piece, *self);
        self.canvas
            .with_texture_canvas(self.main_texture.borrow_mut(), |texture_p| {
                texture_p.set_draw_color(sdl2::pixels::Color::RGBA(63, 63, 63, 255));
                for case in piece.data() {
                    let x = case.x() as i32 * 30;
                    let y = case.y() as i32 * 30;
                    texture_p
                        .fill_rect(Rect::from((x, y, 28, 28)))
                        .expect("Rectange pas dessinable");
                }
                texture_p.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
            });
        //self.canvas.copy(&self.main_texture, None, None).expect("Cant copy");
    }
}

impl View for TetrisView<'_> {
    fn update_v(self: &mut Self, model: &TetrisModel) {
        self.canvas.clear();
        self.draw_board(model);
        self.canvas
            .copy(&self.main_texture, None, None)
            .expect("Cant copy");
        self.canvas.present();
    }
}
