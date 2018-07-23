extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::Read;

const WIDTH: u8 = 64;
const HEIGHT: u8 = 32;
const GFX_SIZE: usize = (HEIGHT as usize) * (WIDTH as usize);

fn main() {
    let mut buf = [0; 4096];

    let mut f = File::open("./roms/PONG").expect("File not found.");
    f.read(&mut buf[0x200..]).expect("Unable to read file.");

    let context = sdl2::init().expect("Failed to initialize SDL2.");
    let video_subsystem = context.video().expect("Failed to aquire video subsystem.");

    let scale = 5u32;
    let window_width = (WIDTH as u32) * scale;
    let window_height = (HEIGHT as u32) * scale;

    let _window = video_subsystem
        .window("Chip8", window_width, window_height)
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
struct Processor {
    pub pc: usize,
    i: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    stack: [u16; 16],
    sp: usize,
    key: [u8; 16],
    gfx: [u8; GFX_SIZE],
}

impl Processor {
    fn new(memory: [u8; 4096]) -> Self {
        Processor {
            memory,
            pc: 0x200,
            i: 0x0,
            v: [0x0; 16],
            stack: [0x0; 16],
            sp: 0x0,
            key: [0x0; 16],
            gfx: [0x0; GFX_SIZE],
        }
    }

    fn execute_cycle(&mut self) {
        let opcode = self.current_opcode();

        match opcode[0] {
            0x8 => self.eight(opcode),
            _ => {}
        }
    }

    /// Implementation of the 0x8 Opcodes.
    fn eight(&mut self, opcode: [u8; 4]) {
        let x = usize::from(opcode[1]);
        let y = usize::from(opcode[2]);

        match opcode[3] {
            0x0 => self.v[x] = self.v[y],
            0x1 => self.v[x] |= self.v[y],
            0x2 => self.v[x] &= self.v[y],
            0x3 => self.v[x] ^= self.v[y],
            0x4 => {
                self.v[x] = match self.v[x].overflowing_add(self.v[y]) {
                    (result, true) => {
                        self.v[0xF] = 0x1;
                        result
                    }
                    (result, false) => result,
                }
            }
            0x5 => {
                self.v[x] = match self.v[x].overflowing_sub(self.v[y]) {
                    (result, true) => {
                        self.v[0xF] = 0x1;
                        result
                    }
                    (result, false) => result,
                }
            }
            0x6 => {
                self.v[0xF] = self.v[y] & 0x0F;
                self.v[x] = self.v[y] >> 1;
            }
            0x7 => {
                self.v[x] = match self.v[y].overflowing_sub(self.v[x]) {
                    (result, true) => {
                        self.v[0xF] = 0x1;
                        result
                    }
                    (result, false) => result,
                }
            }
            0xE => {
                self.v[0xF] = self.v[y] & 0xF0;
                self.v[y] <<= 1;
                self.v[x] = self.v[y];
            }

            _ => unreachable!(),
        }
    }

    fn current_opcode(&self) -> [u8; 4] {
        let code = (u16::from(self.memory[self.pc]) << 8) | u16::from(self.memory[self.pc + 1]);

        [
            ((code & 0xF000) >> 12) as u8,
            ((code & 0x0F00) >> 8) as u8,
            ((code & 0x00F0) >> 4) as u8,
            ((code & 0x000F)) as u8,
        ]
    }
}
