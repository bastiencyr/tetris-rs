use std::borrow::BorrowMut;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use ui::background::Background;

use crate::model::{Model, TetrisModel};

pub trait View {
    fn update_v(self: &mut Self, model: &TetrisModel);
}

pub struct TetrisView<'a> {
    pub main_texture: Texture<'a>,
    pub canvas: Canvas<Window>,
    //Background
    pub background: Rc<Background<'a>>,
}

impl TetrisView<'_> {
    pub fn new<'a>(
        canvas: Canvas<Window>,
        texture: Texture<'a>,
        //background must be behind an rc for with_texture_canvas function. I think
        //that with_texture_canvas take ownership of self and so on the whole structure.
        //It may be solve by edition 2021...
        background: Rc<Background<'a>>,
    ) -> TetrisView<'a> {
        TetrisView {
            canvas,
            main_texture: texture,
            background,
        }
    }

    pub fn draw_board(&mut self, model: &TetrisModel) {
        let back = Rc::clone(&self.background);

        self.canvas
            .with_texture_canvas(self.main_texture.borrow_mut(), |texture_canvas| {
                //Copy the background
                texture_canvas
                    .copy(&back.background_texture, None, None)
                    .expect("Cant copy");

                // Copy already present piece with our iterator
                texture_canvas.set_draw_color(Color::RGBA(63, 63, 63, 255));
                for case in &model.get_model().player[0].board {
                    if case.empty() == false {
                        let rect = Rect::new(case.x() * 30, case.y() * 30, 28, 28);
                        texture_canvas.fill_rect(rect);
                    }
                }

                // Copy the current piece falling
                for case in model.get_model().player[0].piece.data() {
                    let x = case.x() as i32 * 30;
                    let y = case.y() as i32 * 30;
                    texture_canvas
                        .fill_rect(Rect::from((x, y, 28, 28)))
                        .expect("Rectange pas dessinable");
                }

                //Ghost piece
                texture_canvas.set_blend_mode(BlendMode::Blend);
                texture_canvas.set_draw_color(Color::RGBA(43, 43, 100, 80));
                for case in model.get_model().player[0].ghost_piece().data() {
                    let x = case.x() as i32 * 30;
                    let y = case.y() as i32 * 30;
                    texture_canvas
                        .fill_rect(Rect::from((x, y, 28, 28)))
                        .expect("Rectange pas dessinable");
                }

                texture_canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
            });
    }

    pub fn draw_score(&self, model: &TetrisModel) {
        //println!("{}", model.player[0].score())
    }
}

impl View for TetrisView<'_> {
    fn update_v(self: &mut Self, model: &TetrisModel) {
        self.canvas.clear();
        self.draw_board(model);
        self.draw_score(model);
        self.canvas
            .copy(&self.main_texture, None, None)
            .expect("Cant copy");
        self.canvas.present();
    }
}
