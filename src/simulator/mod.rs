#![allow(dead_code, unused_variables)]

use crate::*;

mod tests;

pub struct Cpu {
    a: u16,
    b: u16,
    c: u16,
    d: u16,
    sp: u16,
    bp: u16,
    si: u16,
    di: u16,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            sp: 0,
            bp: 0,
            si: 0,
            di: 0,
        }
    }
}

fn set_register(val: u16, reg: Register, cpu: &mut Cpu) {
    match reg {
        Register::Ax => cpu.a = val,
        Register::Bx => cpu.b = val,
        Register::Cx => cpu.c = val,
        Register::Dx => cpu.d = val,
        Register::Sp => cpu.sp = val,
        Register::Bp => cpu.bp = val,
        Register::Si => cpu.si = val,
        Register::Di => cpu.di = val,
        _ => {
            panic! {"Unknown register. Remove this eventually. Should be a compile error."};
        }
    }
}

pub fn simulate(cpu: &mut Cpu, command: Command) {
    match command.instruction {
        Instruction::Mov => match command.encoding {
            Encoding::ImmediateToReg(data) => {
                set_register(data.immediate, data.dest, cpu);
            }

            _ => {
                panic!("Unknwon mov encoding");
            }
        },

        _ => {
            panic!("Unknown instruction");
        }
    }
}
