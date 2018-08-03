use sdl2;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::video::{DisplayMode, Window};
use std::mem;

pub(crate) struct Display {
    pub(crate) draw_flag: bool,
    gfx: [[bool; 64]; 32],
    scale: Option<u16>,
    window: Option<Window>,
}

impl Default for Display {
    fn default() -> Self {
        Display {
            draw_flag: false,
            gfx: [[false; 64]; 32],
            scale: None,
            window: None,
        }
    }
}

impl Display {
    pub(crate) fn start(
        &mut self,
        video: sdl2::VideoSubsystem,
        width: i32,
        height: i32,
        scale: u16,
        refresh_rate: i32,
    ) {
        self.scale = Some(scale);
        let window_width = 64 * u32::from(self.scale.unwrap());
        let window_height = 32 * u32::from(self.scale.unwrap());

        self.window = Some(
            video
                .window("Chip8", window_width, window_height)
                .position_centered()
                .build()
                .expect("Failed to create window."),
        );

        let display = Some(DisplayMode::new(
            PixelFormatEnum::RGB24,
            width * i32::from(scale),
            height * i32::from(scale),
            refresh_rate,
        ));

        self.window.as_mut().and_then(move |window| {
            window.set_display_mode(display).unwrap();

            Some(window)
        });
    }

    pub(crate) fn clear(&mut self) {
        self.draw_flag = true;
        mem::replace(&mut self.gfx, [[false; 64]; 32]);
    }

    pub(crate) fn draw(&mut self, event_pump: &sdl2::EventPump) {
        if self.draw_flag {
            let mut surface = self
                .window
                .as_mut()
                .map(|window| window.surface(&event_pump).ok())
                .unwrap()
                .unwrap();

            for x in 0..64 {
                for y in 0..32 {
                    let pixel = if self.gfx[y][x] { 255 } else { 0 };

                    let real_x = (x as i32) * (self.scale.unwrap() as i32);
                    let real_y = (y as i32) * (self.scale.unwrap() as i32);

                    surface
                        .fill_rect(
                            Some(Rect::new(
                                real_x,
                                real_y,
                                u32::from(self.scale.unwrap()),
                                u32::from(self.scale.unwrap()),
                            )),
                            Color::RGB(pixel, pixel, pixel),
                        )
                        .unwrap();
                }
            }

            surface.finish().unwrap();
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
