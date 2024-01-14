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
                    self.effective_address_set(&reg_mem.dest, reg_mem.immediate as i16, memory);
                }

                Instruction::Cmp => {
                    self.cmp(reg_mem.dest, reg_mem.immediate as i16);
                }

                Instruction::Sub => {
                    let current: i16 = self.effective_address_get(&reg_mem.dest, memory);
                    self.effective_address_set(
                        &reg_mem.dest,
                        current - reg_mem.immediate as i16,
                        memory,
                    );
                }

                Instruction::Add => {
                    let dest_val: i16 = self.effective_address_get(&reg_mem.dest, memory);
                    self.effective_address_set(
                        &reg_mem.dest,
                        dest_val + reg_mem.immediate as i16,
                        memory,
                    );
                }

                _ => {
                    dbg!(command.instruction);
                    panic!("Unknown instruction.");
                }
            },

            Encoding::RegMemToRegMem(reg_mem) => match command.instruction {
                Instruction::Mov => {
                    let val: i16 = self.effective_address_get(&reg_mem.source, memory);
                    self.effective_address_set(&reg_mem.dest, val, memory);
                }

                Instruction::Cmp => {
                    let source_val: i16 = self.effective_address_get(&reg_mem.source, memory);
                    let dest_val: i16 = self.effective_address_get(&reg_mem.dest, memory);
                    self.update_flags(dest_val - source_val);
                }

                Instruction::Sub => {
                    let source_val: i16 = self.effective_address_get(&reg_mem.source, memory);
                    let dest_val: i16 = self.effective_address_get(&reg_mem.dest, memory);

                    self.effective_address_set(&reg_mem.dest, dest_val - source_val, memory);
                }

                Instruction::Add => {
                    let source_val: i16 = self.effective_address_get(&reg_mem.source, memory);
                    let dest_val: i16 = self.effective_address_get(&reg_mem.dest, memory);

                    self.effective_address_set(&reg_mem.dest, dest_val + source_val, memory);
                }

                _ => {
                    dbg!(command.instruction);
                    panic!("Unknown instruction.");
                }
            },

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

    fn effective_address_set(&mut self, dest: &Address, value: i16, memory: &mut [i16; 2000]) {
        match dest {
            Address::Register(dest_reg) => {
                self.set_register(&dest_reg, value);
            }

            Address::EffectiveAddress(ea) => {
                let address = self.effective_address_resolve(ea);
                memory[address as usize] = value;
                //self.update_flags();
            }
        }

        self.update_flags(value);
    }

    fn effective_address_get(&mut self, dest: &Address, memory: &mut [i16; 2000]) -> i16 {
        match dest {
            Address::Register(reg) => {
                return self.get_value(&reg);
            }

            Address::EffectiveAddress(ea) => {
                let address = self.effective_address_resolve(ea);
                return memory[address as usize];
            }
        }
    }

    fn effective_address_resolve(&self, ea: &EffectiveAddress) -> i16 {
        // If we have a direct address then use that
        if ea.direct != 0 {
            return ea.direct as i16;
        }

        // calculate operands
        let mut first_val: u16 = 0;
        if ea.first_operand != Register::None {
            first_val = self.get_value(&ea.first_operand) as u16;
        }

        let mut second_val: u16 = 0;
        if ea.second_operand != Register::None {
            second_val = self.get_value(&ea.second_operand) as u16;
        }

        return (first_val + second_val + ea.offset) as i16;
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
