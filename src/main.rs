extern crate sdl2;

mod display;
mod processor;

fn main() {
    let scale = 10;

    let mut processor = processor::Processor::default();
    processor.load_rom("./roms/PONG");
    processor.start(scale);
}
