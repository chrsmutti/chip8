use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::mem;

pub(crate) struct Display {
    pub(crate) draw_flag: bool,
    gfx: [[bool; 64]; 32],
}

impl Default for Display {
    fn default() -> Self {
        Display {
            draw_flag: false,
            gfx: [[false; 64]; 32],
        }
    }
}

impl Display {
    pub(crate) fn clear(&mut self) {
        self.draw_flag = true;
        mem::replace(&mut self.gfx, [[false; 64]; 32]);
    }

    pub(crate) fn draw(&mut self, canvas: &mut WindowCanvas, scale: u16) {
        if self.draw_flag {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            for x in 0..64 {
                for y in 0..32 {
                    let pixel = if self.gfx[y][x] { 255 } else { 0 };
                    let sprite = Some(Rect::new(
                        (x as i32) * i32::from(scale),
                        (y as i32) * i32::from(scale),
                        u32::from(scale),
                        u32::from(scale),
                    ));

                    canvas.set_draw_color(Color::RGB(pixel, pixel, pixel));
                    canvas.fill_rect(sprite).unwrap();
                }
            }

            canvas.present();
        }
    }

    pub(crate) fn draw_sprite(&mut self, x: usize, y: usize, n: usize) -> bool {
        self.draw_flag = true;
        let mut collision = false;

        for sprite_x in 0..8 {
            for sprite_y in 0..n {
                collision |= self.gfx[x + sprite_x][y + sprite_y];
                self.gfx[x + sprite_x][y + sprite_y] ^= true;
            }
        }

        collision
    }
}
