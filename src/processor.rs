use display::Display;
use error::ProcessorError;
use failure::Error;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
pub(crate) struct Processor {
    pc: usize,
    i: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    sp: usize,
    stack: [usize; 16],
    keys: [bool; 16],
    display: Display,
}

impl Default for Processor {
    fn default() -> Self {
        Processor {
            memory: [0x0; 4096],
            pc: 0x200,
            i: 0x0,
            v: [0x0; 16],
            sp: 0x0,
            stack: [0x0; 16],
            keys: [false; 16],
            display: Display::default(),
        }
    }
}

impl Processor {
    pub fn load_rom(&mut self, path: &str) {
        let mut f = File::open(path).expect("ROM File not found.");
        f.read(&mut self.memory[0x200..])
            .expect("Failed to read ROM File.");
    }

    pub fn start(&mut self, scale: u16) {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let mut event_pump = context.event_pump().unwrap();

        let window_width = 64 * u32::from(scale);
        let window_height = 32 * u32::from(scale);

        let window = video
            .window("Chip8", window_width, window_height)
            .position_centered()
            .build()
            .expect("Failed to create window.");

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        'game: loop {
            'event: for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'game,
                    Event::KeyDown { keycode, .. } => match keycode {
                        Some(Keycode::Escape) => break 'game,
                        _ => println!("{:?}", keycode),
                    },
                    _ => {}
                }
            }

            if self.pc < 4096 {
                match self.execute_cycle() {
                    Ok(step) => self.pc += step,
                    Err(err) => panic!("{}", err),
                }
            }

            self.display.draw(&mut canvas, scale);
        }
    }

    fn execute_cycle(&mut self) -> Result<usize, Error> {
        self.display.draw_flag = false;
        let opcode = self.current_opcode();

        match opcode[0] {
            0x0 => self.misc_ops(opcode),
            0x1 | 0x2 | 0xB => self.control_flow_ops(opcode),
            0x6 | 0x7 => self.const_ops(opcode),
            0x8 => self.math_bit_ops(opcode),
            0xD => self.display_ops(opcode),
            _ => Ok(2),
        }
    }

    fn current_opcode(&self) -> [u8; 4] {
        let code = (u16::from(self.memory[self.pc]) << 8) | u16::from(self.memory[self.pc + 1]);

        [
            ((code & 0xF000) >> 12) as u8,
            ((code & 0x0F00) >> 8) as u8,
            ((code & 0x00F0) >> 4) as u8,
            (code & 0x000F) as u8,
        ]
    }

    fn full_opcode(opcode: [u8; 4]) -> u16 {
        opcode
            .into_iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (idx, x)| acc + (u16::from(*x) << (idx * 4)))
    }

    /// Implementation of the 0x0 opcodes. (Misc operations)
    fn misc_ops(&mut self, opcode: [u8; 4]) -> Result<usize, Error> {
        match Self::full_opcode(opcode) {
            0x00E0 => self.display.clear(),
            0x00EE => {
                if self.sp > 0 {
                    self.pc = self.stack[self.sp - 1];
                    self.sp -= 1;
                }
            }

            _ => bail!(ProcessorError::UnimplementedOpcode {
                opcode: Self::full_opcode(opcode,)
            }),
        }

        Ok(2)
    }

    /// Implementation of the 0x1, 0x2 and 0xB opcodes. (Control flow operations)
    fn control_flow_ops(&mut self, opcode: [u8; 4]) -> Result<usize, Error> {
        let n = usize::from(Self::full_opcode(opcode) & 0x0FFF);

        match opcode[0] {
            0x1 => self.pc = n,
            0x2 => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = n;
            }
            0xB => self.pc = usize::from(self.v[0x0]) + n,

            _ => bail!(ProcessorError::UnimplementedOpcode {
                opcode: Self::full_opcode(opcode,)
            }),
        }

        Ok(0)
    }

    /// Implementation of the 0x6 and 0x7 opcodes. (Constant operations)
    fn const_ops(&mut self, opcode: [u8; 4]) -> Result<usize, Error> {
        let x = usize::from(opcode[1]);
        let n = (Self::full_opcode(opcode) & 0x00FF) as u8;

        match opcode[0] {
            0x6 => self.v[x] = n,
            0x7 => self.v[x] = self.v[x].saturating_add(n),

            _ => bail!(ProcessorError::UnimplementedOpcode {
                opcode: Self::full_opcode(opcode,)
            }),
        }

        Ok(2)
    }

    /// Implementation of the 0x8 Opcodes. (Math and Bit operations)
    fn math_bit_ops(&mut self, opcode: [u8; 4]) -> Result<usize, Error> {
        let x = usize::from(opcode[1]);
        let y = usize::from(opcode[2]);

        match opcode[3] {
            0x0 => self.v[x] = self.v[y],
            0x1 => self.v[x] |= self.v[y],
            0x2 => self.v[x] &= self.v[y],
            0x3 => self.v[x] ^= self.v[y],
            0x4 => {
                let (result, overflow) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = result;

                if overflow {
                    self.v[0xF] = 0x1;
                }
            }
            0x5 => {
                let (result, overflow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = result;

                if overflow {
                    self.v[0xF] = 0x1;
                }
            }
            0x6 => {
                self.v[0xF] = self.v[y] & 0x0F;
                self.v[x] = self.v[y] >> 1;
            }
            0x7 => {
                let (result, overflow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = result;

                if overflow {
                    self.v[0xF] = 0x1;
                }
            }
            0xE => {
                self.v[0xF] = self.v[y] & 0xF0;
                self.v[y] <<= 1;
                self.v[x] = self.v[y];
            }

            _ => bail!(ProcessorError::UnimplementedOpcode {
                opcode: Self::full_opcode(opcode,)
            }),
        }

        Ok(2)
    }

    /// Implementation of the 0xD opcode. (Display operations)
    fn display_ops(&mut self, opcode: [u8; 4]) -> Result<usize, Error> {
        let collision;
        let x = opcode[1] as usize;
        let y = opcode[2] as usize;
        let n = opcode[3] as usize;

        match opcode[0] {
            0xD => collision = self.display.draw_sprite(x, y, n),

            _ => bail!(ProcessorError::UnimplementedOpcode {
                opcode: Self::full_opcode(opcode,)
            }),
        }

        self.v[0xF] = if collision { 1 } else { 0 };
        Ok(2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_const_ops_setting() {
        let opcode = [0x6, 0x5, 0x6, 0x6];
        let x = usize::from(opcode[1]);
        let n = (opcode[2] << 4) + opcode[3];

        let mut processor = Processor::default();
        processor.v[x] = 0xFF;
        processor.const_ops(opcode);

        assert_eq!(processor.v[x], n)
    }

    #[test]
    fn test_const_ops_addition() {
        let opcode = [0x7, 0x5, 0x6, 0x6];
        let x = usize::from(opcode[1]);
        let n = (opcode[2] << 4) + opcode[3];
        let starting = 0x1;

        let mut processor = Processor::default();
        processor.v[x] = starting;
        processor.const_ops(opcode);

        assert_eq!(processor.v[x], starting + n)
    }

    #[test]
    fn test_const_ops_adition_saturation() {
        let opcode = [0x7, 0x5, 0xF, 0xF];
        let x = usize::from(opcode[1]);

        let mut processor = Processor::default();
        processor.v[x] = 0xFF;
        processor.const_ops(opcode);

        assert_eq!(processor.v[x], 0xFF)
    }

    #[test]
    fn test_misc_ops_subroutine_return() {
        let opcode = [0x0, 0x0, 0xE, 0xE];

        let mut processor = Processor::default();
        processor.stack = [0x255; 16];
        processor.sp = 0x2;
        processor.misc_ops(opcode);

        assert_eq!(processor.sp, 0x1);
        assert_eq!(processor.pc, 0x255);
    }

    #[test]
    #[should_panic]
    fn test_non_implemented_opcode() {
        let opcode = [0x0, 0xE, 0xE, 0xE];

        let mut processor = Processor::default();
        processor.misc_ops(opcode);
    }

    #[test]
    fn test_control_flow_ops_jump() {
        let opcode = [0x1, 0xA, 0xB, 0xC];

        let mut processor = Processor::default();
        processor.control_flow_ops(opcode);

        assert_eq!(processor.pc, 0xABC);
    }

    #[test]
    fn test_control_flow_ops_jump_add() {
        let opcode = [0xB, 0xA, 0xB, 0xC];

        let mut processor = Processor::default();
        processor.v[0x0] = 0x1;
        processor.control_flow_ops(opcode);

        assert_eq!(processor.pc, 0xABC + 0x1);
    }

    #[test]
    fn test_control_flow_ops_subroutine_call() {
        let opcode = [0x2, 0xA, 0xB, 0xC];

        let mut processor = Processor::default();
        let starting_pos = processor.pc;
        processor.control_flow_ops(opcode);

        assert_eq!(processor.pc, 0xABC);
        assert_eq!(processor.sp, 0x1);
        assert_eq!(processor.stack[0], starting_pos);
    }

    #[test]
    fn test_subroutine() {
        let call_opcode = [0x2, 0xA, 0xB, 0xC];
        let return_opcode = [0x0, 0x0, 0xE, 0xE];

        let mut processor = Processor::default();
        let starting_pos = processor.pc;
        processor.control_flow_ops(call_opcode);
        processor.misc_ops(return_opcode);

        assert_eq!(processor.pc, starting_pos);
        assert_eq!(processor.sp, 0);
    }

}
