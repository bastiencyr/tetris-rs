pub mod background {
    extern crate sdl2;

    use sdl2::rect::Point;
    use sdl2::render::Canvas;
    use sdl2::render::Texture;
    use sdl2::render::TextureCreator;
    use sdl2::video::Window;
    use sdl2::video::WindowContext;

// on le déclare public pour quil soit accessible de l'extérieur

    pub struct Background<'a> {
        pub background_texture: Texture<'a>,
        color: [u8; 4],
    }

    impl Background<'_> {
        pub fn new<'a>(
            canvas: &mut Canvas<Window>,
            texture_creator: &'a TextureCreator<WindowContext>,
        ) -> Background<'a> {
            let texture = texture_creator
                .create_texture_target(texture_creator.default_pixel_format(), 300, 600)
                .unwrap();
            let mut back = Background {
                background_texture: texture,
                color: [0, 0, 63, 255], // la couleur de fond + gamma
            };
            back.draw(canvas);
            back
        }

        fn draw(&mut self, canvas: &mut Canvas<Window>) {
            let (x, y) = canvas.window().size();
            //let color = self.color;
            canvas
                .with_texture_canvas(&mut self.background_texture, |texture_canvas| {
                    //initialiser  la couleur de fond ici avec color

                    texture_canvas.set_draw_color(sdl2::pixels::Color::RGBA(63, 63, 63, 255));

                    let x = x as i32;
                    let y = y as i32;
                    let size_carre = x / 10;

                    //verticale
                    for number in 1..10 {
                        let p1 = Point::from((number * size_carre, 0));
                        let p2 = Point::from((number * size_carre, y));
                        texture_canvas
                            .draw_line(p1, p2)
                            .expect("Failed to draw line");
                    }

                    //horizontale
                    for number in 1..20 {
                        let p1 = Point::from((0, number * size_carre));
                        let p2 = Point::from((x, number * size_carre));
                        texture_canvas
                            .draw_line(p1, p2)
                            .expect("Failed to draw line");
                    }

                    texture_canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
                })
                .unwrap();
        }

        pub fn copy_back_to_texture(
            &self,
            canvas: &mut Canvas<Window>,
            main_texture: &mut Texture,
        ) {
            canvas
                .with_texture_canvas(main_texture, |texture_canvas| {
                    texture_canvas
                        .copy(&self.background_texture, None, None)
                        .expect("Cant copy");
                })
                .unwrap();
        }
    }
}
