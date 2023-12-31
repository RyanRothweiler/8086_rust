use crate::*;

#[allow(unreachable_patterns)]
pub fn pull_command(asm: &mut Asm) -> Option<Command> {
    let first_byte: u8 = match asm.pull_byte() {
        Some(v) => *v,
        None => return None,
    };

    let mut ret: Command = get_command(first_byte);

    match ret.encoding {
        Encoding::RegMemToRegMem(ref mut data) => {
            let second_byte: u8 = match asm.pull_byte() {
                Some(v) => *v,
                None => return None,
            };

            // first byte
            let w_mask: u8 = 0b0000_0001;
            let d_mask: u8 = 0b0000_0010;

            let w_val: bool = (first_byte & w_mask) == 1;
            let d_val: bool = ((first_byte & d_mask) >> 1) == 1;

            // second byte
            let mod_mask: u8 = 0b1100_0000;
            let reg_mask: u8 = 0b0011_1000;
            let rm_mask: u8 = 0b0000_0111;

            let mod_val: u8 = second_byte & mod_mask;
            let reg_val: u8 = (second_byte & reg_mask) << 2;
            let rm_val: u8 = (second_byte & rm_mask) << 5;

            /*
            println!("");
            println!("mod {mod_val:#8b}");
            println!("source {source_val:#8b}");
            println!("dest_val {dest_val:#8b}");
            println!("first_byte {first_byte:#8b}");
            println!("second_byte {second_byte:#8b}");
            */

            let mod_results = match handle_mod(mod_val, reg_val, rm_val, w_val, asm) {
                Some(v) => v,
                None => return None,
            };
            
            // handle source / dest swapping based on the d_val
            if d_val {
                data.source = mod_results.rm_address;
                data.dest = mod_results.reg_address;
            } else {
                data.source = mod_results.reg_address;
                data.dest = mod_results.rm_address;
            }
        }

        Encoding::ImmediateToReg(ref mut data) => {
            let second_byte: u8 = match asm.pull_byte() {
                Some(v) => *v,
                None => return None,
            };

            let w_mask: u8 = 0b0000_1000;
            let reg_mask: u8 = 0b0000_0111;

            let w_val: bool = ((first_byte & w_mask) >> 3) == 1;
            let reg_val: u8 = (first_byte & reg_mask) << 5;

            data.dest = decode_register(reg_val, w_val);

            if w_val {
                let third_byte: u8 = match asm.pull_byte() {
                    Some(v) => *v,
                    None => return None,
                };

                data.immediate = combine(second_byte, third_byte);
            } else {
                data.immediate = second_byte as u16;
            }
        }

        Encoding::ImmediateToRegMem(ref mut data) => {
            let second_byte: u8 = match asm.pull_byte() {
                Some(v) => *v,
                None => return None,
            };

            // first byte
            let w_mask: u8 = 0b0000_0001;
            let s_mask: u8 = 0b0000_0010;

            let w_val: bool = (first_byte & w_mask) == 1;
            let s_val: bool = ((first_byte & s_mask) >> 1) == 1;

            assert!(s_val, "Invalid s_val");

            // second byte
            let reg_mask: u8 = 0b0000_0111;
            let mod_mask: u8 = 0b1100_0000;

            let reg_val: u8 = (second_byte & reg_mask) << 5;
            let mod_val: u8 = first_byte & reg_mask;

            // get results
            let mod_results = match handle_mod(mod_val, reg_val, 0, w_val, asm) {
                Some(v) => v,
                None => return None,
            };

            //data.dest = mod_results.
        }

        _ => {
            panic!("Unknown instruction");
        }
    }

    Some(ret)
}

// combine two 8 bits into one 16 bit 
fn combine(low: u8, high: u8) -> u16 {
    let low: u16 = low as u16;
    let high: u16 = (high as u16) << 8;
    let num: u16 = high | low;

    return num;
}

struct ModResults {
    reg_address: Address,
    rm_address: Address,
}

// extract addresses based on the mod tables
fn handle_mod(
    mod_val: u8,
    reg_val: u8,
    rm_val: u8,
    w_val: bool,
    asm: &mut Asm,
) -> Option<ModResults> {
    let mut results = ModResults {
        reg_address: Address::Register(Register::None),
        rm_address: Address::Register(Register::None),
    };

    match mod_val {
        // register to register
        // mov si, bx
        0b1100_0000 => {
            results.reg_address = Address::Register(decode_register(reg_val, w_val));
            results.rm_address = Address::Register(decode_register(rm_val, w_val));
        }

        // effective address calculation
        0b0000_0000 => {
            results.reg_address = Address::Register(decode_register(reg_val, w_val));

            match rm_val {
                0b0000_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bx,
                        second_operand: Register::Si,
                        offset: 0,
                    });
                }

                0b0110_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bp,
                        second_operand: Register::Di,
                        offset: 0,
                    });
                }

                0b0010_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bx,
                        second_operand: Register::Di,
                        offset: 0,
                    });
                }

                0b0100_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bp,
                        second_operand: Register::Si,
                        offset: 0,
                    });
                }

                _ => {
                    panic!("Unknown effective address source val");
                }
            };
        }

        // effective address calculation
        0b0100_0000 => {
            results.reg_address = Address::Register(decode_register(reg_val, w_val));

            match rm_val {
                0b0000_0000 => {
                    let low: u8 = match asm.pull_byte() {
                        Some(v) => *v,
                        None => return None,
                    };

                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bx,
                        second_operand: Register::Si,
                        offset: low as u16,
                    });
                }

                0b1100_0000 => {
                    let offset: u8 = match asm.pull_byte() {
                        Some(v) => *v,
                        None => return None,
                    };

                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bp,
                        second_operand: Register::None,
                        offset: offset as u16,
                    });
                }

                _ => {
                    panic!("Unknown effective address source val");
                }
            };
        }

        // effective address calculation
        0b1000_0000 => {
            results.reg_address = Address::Register(decode_register(reg_val, w_val));

            match rm_val {
                0b0000_0000 => {
                    let low: u8 = match asm.pull_byte() {
                        Some(v) => *v,
                        None => return None,
                    };
                    let high: u8 = match asm.pull_byte() {
                        Some(v) => *v,
                        None => return None,
                    };

                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bx,
                        second_operand: Register::Si,
                        offset: combine(low, high),
                    });
                }

                _ => {
                    panic!("Unknown effective address source val");
                }
            };
        }
        _ => {
            panic!("Unknown mod value");
        }
    };

    Some(results)
}

fn get_command(byte: u8) -> Command {
    const MOV_REG_MEM: u8 = 0b_1000_1000;
    const ADD_REG_MEM: u8 = 0b_0000_0000;
    const REG_MEM_TO_REG_MEM_MASK: u8 = 0b_1111_1100;

    const MOV_IMMEDIATE_REG: u8 = 0b_1011_0000;
    const IMMEDIATE_REG_MASK: u8 = 0b_1111_0000;

    const ADD_IMMEDIATE_REG_MEM: u8 = 0b_1000_0000;
    const IMMEDIATE_REG_MEM_MASK: u8 = 0b_1111_0000;

    if (byte & REG_MEM_TO_REG_MEM_MASK) == MOV_REG_MEM {
        return Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::None),
                dest: Address::Register(Register::None),
            }),
        };
    } else if (byte & REG_MEM_TO_REG_MEM_MASK) == ADD_REG_MEM {
        return Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::None),
                dest: Address::Register(Register::None),
            }),
        };
    } else if (byte & IMMEDIATE_REG_MEM_MASK) == ADD_IMMEDIATE_REG_MEM {
        return Command {
            instruction: Instruction::Add,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                immediate: 0,
                dest: Address::Register(Register::None),
            }),
        };
    } else if (byte & IMMEDIATE_REG_MASK) == MOV_IMMEDIATE_REG {
        return Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                immediate: 0,
                dest: Register::None,
            }),
        };
    }

    panic!("Unkown instruction");
}

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
