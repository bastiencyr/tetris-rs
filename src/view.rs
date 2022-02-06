use std::borrow::{Borrow, BorrowMut};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator, WindowCanvas};
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
    pub background: Background<'a>,
    color_piece: Color,
    color_ghost: Color,
}

pub struct TetrisViewBuilder<'a> {
    pub main_texture: Texture<'a>,
    pub canvas: Canvas<Window>,
    //Background
    pub background: Background<'a>,
    color_piece: Color,
    color_ghost: Color,
}

impl<'b> TetrisViewBuilder<'b> {
    pub fn new(
        mut canvas: WindowCanvas,
        texture: Texture<'b>,
        background: Background<'b>,
    ) -> TetrisViewBuilder<'b> {
        TetrisViewBuilder {
            main_texture: texture,
            canvas,
            background,
            color_piece: Color::RGBA(63, 63, 63, 255),
            color_ghost: Color::RGBA(43, 43, 100, 80),
        }
    }

    pub fn color_ghost(mut self, color_piece: Color) -> Self {
        self.color_ghost = color_piece;
        self
    }

    pub fn color_piece(mut self, color_piece: Color) -> Self {
        self.color_piece = color_piece;
        self
    }

    pub fn color_line(mut self, color_back: Color) -> Self {
        self.background.set_color(color_back, &mut self.canvas);
        self
    }

    pub fn color_back2(mut self, color_back: Color) -> Self {
        self.background.set_color_back(color_back, &mut self.canvas);

        self
    }

    pub fn background2(mut self, background: Background<'b>) -> Self {
        self.background = background;
        self
    }

    pub fn build(self) -> TetrisView<'b> {
        return TetrisView {
            main_texture: self.main_texture,
            canvas: self.canvas,
            background: self.background,
            color_piece: self.color_piece,
            color_ghost: self.color_ghost,
        };
    }
}

impl<'b> TetrisView<'b> {
    pub fn builder(
        canvas: WindowCanvas,
        texture: Texture<'b>,
        background: Background<'b>,
    ) -> TetrisViewBuilder<'b> {
        TetrisViewBuilder::new(canvas, texture, background)
    }

    pub fn draw_board(&mut self, model: &TetrisModel) {
        //let back = Rc::clone(&self.background);
        // on 2015 edition this code will not work because in 2015 edition, closure capture the
        // whole structure even if you acceded only one member of your struct
        // here self.background is borrow as immutable -> so whole self is borrow as immutable in 2015 edition
        // and self.canvas capture canvas as
        // mutable (because with_texture_canvas take self as mutable) -> so capture whole self as immutable in 2015 edition
        self.canvas
            .with_texture_canvas(self.main_texture.borrow_mut(), |texture_canvas| {
                //Copy the background
                texture_canvas
                    .copy(&self.background.background_texture, None, None)
                    .expect("Cant copy");

                // Copy already present piece with our iterator
                texture_canvas.set_draw_color(self.color_piece);
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
                texture_canvas.set_draw_color(self.color_ghost);
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
