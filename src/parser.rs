use crate::*;

const MOV_REG_MEM: u8 = 0b_1000_1000;
const MOV_IMMEDIATE_REG: u8 = 0b_1011_0000;

pub fn pull_command(asm: &mut Asm) -> Option<Command> {
    let mut ret = Command {
        instruction: Instruction::None,
        encoding: Encoding::None,
    };

    let first_byte: u8 = match asm.pull_byte() {
        Some(v) => *v,
        None => return None,
    };

    let instruction_mask: u8 = 0b_1111_1100;
    let instruction_val: u8 = first_byte & instruction_mask;

    match instruction_val {
        MOV_REG_MEM => {
            ret.instruction = Instruction::Mov;

            let second_byte: u8 = match asm.pull_byte() {
                Some(v) => *v,
                None => return None,
            };

            let w_mask: u8 = 0b_0000_0001;
            let mod_mask: u8 = 0b_1100_0000;
            let first_reg_mask: u8 = 0b_0011_1000;
            let second_reg_mask: u8 = 0b_0000_0111;

            let w_val: bool = (first_byte & w_mask) == 1;
            let mod_val: u8 = second_byte & mod_mask;
            let first_reg: u8 = (second_byte & first_reg_mask) << 2;
            let second_reg: u8 = (second_byte & second_reg_mask) << 5;

            if mod_val != 0b1100_0000 {
                panic!("Unknown mod");
            }

            let data = RegMemToRegMem {
                source: decode_register(first_reg, w_val),
                dest: decode_register(second_reg, w_val),
            };
            ret.encoding = Encoding::RegMemToRegMem(data);
        }

        MOV_IMMEDIATE_REG => {}

        _ => {
            panic!("Unknown instruction");
        }
    }

    Some(ret)
}

/*
fn decode_instruction(byte : u8) -> Instruction{
    if (byte & 0b_1111_1100) == MOV_REG_MEM {
        return Instruction::Mov
    }

    Instruction::None
}
*/

fn decode_register(input: u8, w_set: bool) -> Register {
    if w_set {
        match input {
            0b0000_0000 => Register::Ax,
            0b0010_0000 => Register::Cx,
            0b0100_0000 => Register::Dx,
            0b0110_0000 => Register::Bx,

            0b1000_0000 => Register::Sp,
            0b1010_0000 => Register::Bp,
            0b1100_0000 => Register::Si,
            0b1110_0000 => Register::Di,

            _ => panic!("Unknown register encoding"),
        }
    } else {
        match input {
            0b0000_0000 => Register::Al,
            0b0010_0000 => Register::Cl,
            0b0100_0000 => Register::Dl,
            0b0110_0000 => Register::Bl,

            0b1000_0000 => Register::Ah,
            0b1010_0000 => Register::Ch,
            0b1100_0000 => Register::Dh,
            0b1110_0000 => Register::Bh,

            _ => panic!("Unknown register encoding"),
        }
    }
}
