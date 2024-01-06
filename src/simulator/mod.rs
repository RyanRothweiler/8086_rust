#![allow(dead_code, unused_variables)]

use crate::*;

mod tests;

pub struct Cpu {
    a: i16,
    b: i16,
    c: i16,
    d: i16,
    sp: i16,
    bp: i16,
    si: i16,
    di: i16,

    signed_flag: bool,
    zero_flag: bool,
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

            signed_flag: false,
            zero_flag: false,
        }
    }

    pub fn print(&self) {
        println!("--------------------");
        println!("REG-----------------");
        println!("a  {}", self.a);
        println!("b  {}", self.b);
        println!("c  {}", self.c);
        println!("d  {}", self.d);
        println!("sp {}", self.sp);
        println!("bp {}", self.bp);
        println!("si {}", self.si);
        println!("di {}", self.di);
        println!("FLAGS----------------");
        println!("signed {}", self.signed_flag);
        println!("zero   {}", self.zero_flag);
        println!("---------------------");
    }

    pub fn simulate(&mut self, command: Command) {
        match command.encoding {
            Encoding::ImmediateToReg(data) => {
                self.set_register(&data.dest, data.immediate as i16);
            }

            Encoding::ImmediateToRegMem(reg_mem) => {
                let dest: Register = match reg_mem.dest {
                    Address::Register(d) => d,
                    _ => {
                        panic!("Unimplemented address")
                    }
                };

                match command.instruction {
                    Instruction::Mov => {
                        self.set_register(&dest, reg_mem.immediate as i16);
                    }
                    Instruction::Cmp => {
                        let dest_val = self.get_value(&dest);
                        self.update_flags(dest_val - reg_mem.immediate as i16);
                    }
                    Instruction::Sub => {
                        self.sub(dest, reg_mem.immediate as i16);
                    }
                    Instruction::Add => {
                        self.add(dest, reg_mem.immediate as i16);
                    }
                    _ => {
                        dbg!(command.instruction);
                        panic!("Unknown instruction.");
                    }
                }
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

                match command.instruction {
                    Instruction::Mov => {
                        let val = self.get_value(&source);
                        self.set_register(&dest, val);
                    }
                    Instruction::Cmp => {
                        let source_val = self.get_value(&source);
                        let dest_val = self.get_value(&dest);
                        self.update_flags(dest_val - source_val);
                    }
                    Instruction::Sub => {
                        let source_val = self.get_value(&source);
                        self.sub(dest, source_val);
                    }
                    Instruction::Add => {
                        let source_val = self.get_value(&source);
                        self.add(dest, source_val);
                    }
                    _ => {
                        dbg!(command.instruction);
                        panic!("Unknown instruction.");
                    }
                }
            }
            _ => {
                dbg!(command.encoding);
                panic!("Unknwon encoding");
            }
        }
    }

    fn set_register(&mut self, reg: &Register, val: i16) {
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

    fn get_value(&self, reg: &Register) -> i16 {
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

    fn update_flags(&mut self, val: i16) {
        self.signed_flag = val < 0;
        self.zero_flag = val == 0;
    }

    fn add(&mut self, dest: Register, value: i16) {
        let curr: i16 = self.get_value(&dest);
        let new_val = curr + value;
        self.set_register(&dest, new_val);
        self.update_flags(new_val);
    }

    fn sub(&mut self, dest: Register, value: i16) {
        let curr: i16 = self.get_value(&dest);
        let new_val = curr - value;
        self.set_register(&dest, new_val);
        self.update_flags(new_val);
    }
}
