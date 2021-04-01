use crate::disassembler::Instruction;

pub struct Cpu {
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub stack: [u16; 16],
    pub i: u16,
    pub vf: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub pc: u16,
    pub sp: u8,
}

impl Cpu {
    pub fn load_program(&mut self, program: &Vec<Instruction>) {
        let mut instructions: Vec<u8> = Vec::new();

        for i in program {
            instructions.push((i.opcode & 0xff00) as u8);
            instructions.push((i.opcode & 0x00ff) as u8);
        }

        self.memory[0x200..0x200+instructions.len()].copy_from_slice(&instructions[..]);

        println!("{:02x}", self.memory[0x205]);
    }
}
