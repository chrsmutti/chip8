#[macro_use]
extern crate failure;
extern crate rand;
extern crate sdl2;

mod display;
mod error;
mod processor;

use processor::Processor;

fn main() {
    let scale = 10;

    let mut processor = Processor::default();
    processor.load_rom("./roms/PONG");
    processor.start(scale);
}
