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

    // todo remove this public?
    pub instruction_pointer: i64,

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

            instruction_pointer: 0,

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

    pub fn simulate(&mut self, command: Command, memory: &mut [i16; 2000]) {
        match command.encoding {
            Encoding::Jump(offset) => match command.instruction {
                Instruction::Jnz => {
                    if !self.zero_flag {
                        self.instruction_pointer += offset as i64;
                    }
                }

                _ => {
                    panic!("Unknown instruction");
                }
            },

            Encoding::ImmediateToReg(data) => {
                self.set_register(&data.dest, data.immediate as i16);
            }

            Encoding::ImmediateToRegMem(reg_mem) => match command.instruction {
                Instruction::Mov => {
                    self.mov_address_immediate(reg_mem.dest, reg_mem.immediate as i16, memory);
                }

                Instruction::Cmp => {
                    self.cmp(reg_mem.dest, reg_mem.immediate as i16);
                }

                Instruction::Sub => {
                    self.sub_address(reg_mem.dest, reg_mem.immediate as i16);
                }

                Instruction::Add => {
                    self.add_address(reg_mem.dest, reg_mem.immediate as i16);
                }

                _ => {
                    dbg!(command.instruction);
                    panic!("Unknown instruction.");
                }
            },

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

    fn sub_address(&mut self, dest: Address, value: i16) {
        match dest {
            Address::Register(reg) => {
                self.sub(reg, value);
            }

            _ => {
                panic!("Unimplemented address type");
            }
        }
    }

    fn add_address(&mut self, dest: Address, value: i16) {
        match dest {
            Address::Register(reg) => {
                self.add(reg, value);
            }

            _ => {
                panic!("Unimplemented address type");
            }
        }
    }

    fn mov_address_immediate(&mut self, dest: Address, value: i16, memory: &mut [i16; 2000]) {
        match dest {
            Address::Register(dest_reg) => {
                self.set_register(&dest_reg, value);
            }

            Address::EffectiveAddress(ea) => {

                let mut first_val: u16 = 0;
                if ea.first_operand != Register::None {
                    first_val = self.get_value(&ea.first_operand) as u16;
                }

                let mut second_val: u16 = 0;
                if ea.second_operand != Register::None {
                    second_val = self.get_value(&ea.second_operand) as u16;
                }

                let memory_address : u16 = first_val + second_val + ea.offset;
                memory[usize::from(memory_address)] = value;
                //self.update_flags();
            }
        }
    }

    fn cmp(&mut self, dest: Address, value: i16) {
        match dest {
            Address::Register(dest_reg) => {
                let dest_val = self.get_value(&dest_reg);
                self.update_flags(dest_val - value);
            }

            _ => {
                panic!("Unimplemented dest");
            }
        }
    }
}
