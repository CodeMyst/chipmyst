mod disassembler;

use std::env;
use disassembler::disassemble;
use raylib::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("rom path not provided");
    }

    let instructions = disassemble(&args[1]);

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
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
