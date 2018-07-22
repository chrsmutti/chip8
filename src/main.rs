extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::Read;
use std::u16;

const SCALE: u32 = 5;
const WIDTH: u32 = 64 * SCALE;
const HEIGHT: u32 = 32 * SCALE;

fn main() {
    let mut buf = [0; 4096];

    let mut f = File::open("./roms/PONG").expect("File not found.");
    f.read(&mut buf[0x200..]).expect("Unable to read file.");

    let context = sdl2::init().expect("Failed to initialize SDL2.");
    let video_subsystem = context.video().expect("Failed to aquire video subsystem.");

    let _window = video_subsystem
        .window("Chip8", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("Failed to create window.");

    let mut event_pump = context.event_pump().unwrap();
    let mut processor = Processor::new(buf);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => break 'running,
                    _ => println!("{:?}", keycode),
                },
                _ => {}
            }
        }

        if processor.pc < 4096 {
            processor.execute_cycle();
        }

        processor.pc += 2;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Processor {
    pub pc: usize,
    i: u16,
    memory: Vec<u8>,
}

impl Processor {
    fn new(memory: [u8; 4096]) -> Self {
        Processor {
            pc: 0x200,
            i: 0x0,
            memory: memory.to_vec(),
        }
    }

    fn execute_cycle(&mut self) {
        let opcode = self.current_opcode();

        println!("Current Opcode: {:x}", opcode);
    }

    fn current_opcode(&self) -> u16 {
        (u16::from(self.memory[self.pc]) << 8) | u16::from(self.memory[self.pc + 1])
    }
}
