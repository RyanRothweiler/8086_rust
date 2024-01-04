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
    pub fn new() -> Cpu {
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

    pub fn print(&self) {
        println!("--------------------");
        println!("a  {}", self.a);
        println!("b  {}", self.b);
        println!("c  {}", self.c);
        println!("d  {}", self.d);
        println!("sp {}", self.sp);
        println!("bp {}", self.bp);
        println!("si {}", self.si);
        println!("di {}", self.di);
        println!("--------------------");
    }

    pub fn simulate(&mut self, command: Command) {
        match command.instruction {
            Instruction::Mov => match command.encoding {
                Encoding::ImmediateToReg(data) => {
                    self.set_register(data.dest, data.immediate);
                }

                Encoding::RegMemToRegMem(reg_mem) => {
                    let dest: Register = match reg_mem.dest {
                        Address::Register(d) => d,
                        _ => {
                            panic!("Unimplemented address")
                        }
                    };

                    let source: Register = match reg_mem.source {
                        Address::Register(d) => d,
                        _ => {
                            panic!("Unimplemented address")
                        }
                    };

                    let val = self.get_value(source);
                    self.set_register(dest, val);
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

    fn set_register(&mut self, reg: Register, val: u16) {
        match reg {
            Register::Ax => self.a = val,
            Register::Bx => self.b = val,
            Register::Cx => self.c = val,
            Register::Dx => self.d = val,
            Register::Sp => self.sp = val,
            Register::Bp => self.bp = val,
            Register::Si => self.si = val,
            Register::Di => self.di = val,
            _ => {
                panic! {"Unknown register. Remove this eventually. Should be a compile error."};
            }
        }
    }

    fn get_value(&self, reg: Register) -> u16 {
        match reg {
            Register::Ax => self.a,
            Register::Bx => self.b,
            Register::Cx => self.c,
            Register::Dx => self.d,
            Register::Sp => self.sp,
            Register::Bp => self.bp,
            Register::Si => self.si,
            Register::Di => self.di,
            _ => {
                panic! {"Unknown register. Remove this eventually. Should be a compile error."};
            }
        }
    }
}
