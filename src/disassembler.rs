use std::fs;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: u16,
    pub asm: String,
}

pub fn disassemble(rom_path: &String) -> Vec<Instruction> {
    let rom_contents = fs::read(rom_path).expect("can't read rom");

    let mut instructions: Vec<Instruction> = Vec::new();

    let mut i: usize = 0;
    while i < rom_contents.len() {
        let opcode = ((rom_contents[i] as u16) << 8) | (rom_contents[i+1] as u16);

        let nnn = opcode & 0x0fff;
        let x = (opcode & 0x0f00) >> 8;
        let y = (opcode & 0x00f0) >> 4;
        let n = opcode & 0x000f;
        let kk = opcode & 0x00ff;

        let op_1 = (opcode & 0xf000) >> 12;
        let op_2 = (opcode & 0x0f00) >> 8;
        let op_3 = (opcode & 0x00f0) >> 4;
        let op_4 = opcode & 0x000f;

        let asm: String = match (op_1, op_2, op_3, op_4) {
            (0x0, 0x0, 0xe, 0x0) => "CLS".to_string(),

            (0x0, 0x0, 0xe, 0xe) => "RET".to_string(),

            (0x0, _, _, _) => format!("SYS {:#06x} (ignored)", nnn),

            (0x1, _, _, _) => format!("JP {:#06x}", nnn),

            (0x2, _, _, _) => format!("CALL {:#06x}", nnn),

            (0x3, _, _, _) => format!("SE V{:?}, {:#04x}", x, kk),

            (0x4, _, _, _) => format!("SNE V{:?}, {:#04x}", x, kk),

            (0x5, _, _, 0x0) => format!("SE V{:?}, V{:?}", x, y),

            (0x6, _, _, _) => format!("LD V{:?}, {:#04x}", x, kk),

            (0x7, _, _, _) => format!("ADD V{:?}, {:#04x}", x, kk),

            (0x8, _, _, 0x0) => format!("LD V{:?}, V{:?}", x, y),

            (0x8, _, _, 0x1) => format!("OR V{:?}, V{:?}", x, y),

            (0x8, _, _, 0x2) => format!("AND V{:?}, V{:?}", x, y),

            (0x8, _, _, 0x3) => format!("XOR V{:?}, V{:?}", x, y),

            (0x8, _, _, 0x4) => format!("ADD V{:?}, V{:?}", x, y),

            (0x8, _, _, 0x5) => format!("SUB V{:?}, V{:?}", x, y),

            (0x8, _, _, 0x6) => format!("SHR V{:?}, V{:?}", x, y),

            (0x8, _, _, 0x7) => format!("SUBN V{:?}, V{:?}", x, y),

            (0x8, _, _, 0xe) => format!("SHL V{:?}, V{:?}", x, y),

            (0x9, _, _, 0x0) => format!("SNE V{:?}, V{:?}", x, y),

            (0xa, _, _, _) => format!("LD I, {:#06x}", nnn),

            (0xb, _, _, _) => format!("JP V0, {:#06x}", nnn),

            (0xc, _, _, _) => format!("RDN V{:?}, {:#04x}", x, kk),

            (0xd, _, _, _) => format!("DRW V{:?}, V{:?}, {:#03x}", x, y, n),

            (0xe, _, 0x9, 0xe) => format!("SKP V{:?}", x),

            (0xe, _, 0xa, 0x1) => format!("SKNP V{:?}", x),

            (0xf, _, 0x0, 0x7) => format!("LD V{:?}, DT", x),

            (0xf, _, 0x0, 0xa) => format!("LD V{:?}, K", x),

            (0xf, _, 0x1, 0x5) => format!("LD DT, V{:?}", x),

            (0xf, _, 0x1, 0x8) => format!("LD ST, V{:?}", x),

            (0xf, _, 0x1, 0xe) => format!("ADD I, V{:?}", x),

            (0xf, _, 0x2, 0x9) => format!("LD F, V{:?}", x),

            (0xf, _, 0x3, 0x3) => format!("LD B, V{:?}", x),

            (0xf, _, 0x5, 0x5) => format!("LD [I], V{:?}", x),

            (0xf, _, 0x6, 0x5) => format!("LD V{:?}, [I]", x),

            _ => format!("unknown instruction")
        };

        let instruction = Instruction { opcode, asm };

        instructions.push(instruction);

        i += 2;
    }

    return instructions;
}
