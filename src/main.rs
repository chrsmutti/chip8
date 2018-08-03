extern crate sdl2;

mod display;
mod processor;

fn main() {
    let width = 64;
    let height = 32;
    let scale = 10;
    let refresh_rate = 60;

    let mut processor = processor::Processor::default();
    processor.load_rom("./roms/PONG");
    processor.start(width, height, scale, refresh_rate);
}