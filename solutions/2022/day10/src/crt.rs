use std::{fmt::Write, sync::mpsc::Receiver};

use crate::cpu::Cpu;

pub struct Crt<'a> {
    cycle: usize,
    position: usize,
    /// CRT receives sprite position from the cpu on this channel.
    cpu_channel: Receiver<i64>,

    /// The CRT will draw into this Writeable handle
    out: &'a mut (dyn Write + 'a),
}

impl<'a> Crt<'a> {
    /// Creates a new [`Crt`].
    pub fn new(rx: Receiver<i64>, out: &'a mut (dyn Write + 'a)) -> Self {
        Crt {
            cycle: 0,
            position: 0,
            cpu_channel: rx,
            out,
        }
    }

    pub fn install_sprite_hook(cpu: &mut Cpu) {
        cpu.add_hook("X", |_, value| value);
    }

    pub fn step(&mut self) {
        self.cycle += 1;
        let at_end_of_line = self.at_end_of_line();

        // read sprite position from cpu
        if let Ok(sprite_position) = self.cpu_channel.try_recv() {
            self.draw(sprite_position as usize, at_end_of_line);
        }
        if at_end_of_line {
            self.position = 0;
        } else {
            self.position += 1;
        }
    }

    fn draw(&mut self, spr_pos: usize, at_end_of_line: bool) {
        // sprite is 3 pixels wide
        let pixel_on = self.position >= spr_pos.saturating_sub(1) && self.position <= spr_pos + 1;
        let pixel = if pixel_on { "#" } else { "." };
        write!(self.out, "{}", pixel).expect("crt write");
        if at_end_of_line {
            writeln!(self.out).expect("crt write");
        }
    }

    fn at_end_of_line(&self) -> bool {
        self.cycle % 40 == 0
    }
}
