mod disassembler;
mod cpu;

use std::env;
use cpu::Cpu;
use disassembler::disassemble;
use raylib::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("rom path not provided");
    }

    let instructions = disassemble(&args[1]);

    let mut cpu = Cpu { memory: [0; 4096],
        registers: [0; 16],
        stack: [0; 16],
        i: 0,
        vf: 0,
        delay_timer: 0,
        sound_timer: 0,
        pc: 0,
        sp: 0 };

    cpu.load_program(&instructions);

    let (mut rl, thread) = raylib::init()
        .size(640, 320)
        .title("chipmyst")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        let x: i32 = 10;
        let mut y: i32 = 10;

        for instr in &instructions {
            d.draw_text(&instr.asm, x, y, 8, Color::BLACK);

            y += 10;
        }
    }
}
