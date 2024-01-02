use crate::*;

#[allow(unreachable_patterns)]
pub fn pull_command(asm: &mut Asm) -> Option<Command> {
    let first_byte: u8 = match asm.pull_byte() {
        Some(v) => *v,
        None => return None,
    };

    let mut ret: Command = get_command(first_byte);

    match ret.encoding {
        Encoding::Jump(ref mut data) => {
            let second_byte: u8 = match asm.pull_byte() {
                Some(v) => *v,
                None => return None,
            };

            *data = second_byte as i8;
        }

        Encoding::ImmediateToAccumulator(ref mut data) => {
            let w_mask: u8 = 0b0000_0001;
            let w_val: bool = (first_byte & w_mask) == 1;

            if !w_val {
                data.dest = Register::Al;

                let second_byte: u8 = match asm.pull_byte() {
                    Some(v) => *v,
                    None => return None,
                };
                data.immediate = second_byte as u16
            } else {
                data.dest = Register::Ax;

                let second_byte: u8 = match asm.pull_byte() {
                    Some(v) => *v,
                    None => return None,
                };
                let third_byte: u8 = match asm.pull_byte() {
                    Some(v) => *v,
                    None => return None,
                };
                data.immediate = combine(second_byte, third_byte);
            }
        }

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

            // get results
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

            // second byte
            let rm_mask: u8 = 0b0000_0111;
            let mod_mask: u8 = 0b1100_0000;
            let operation_mask: u8 = 0b00111_000;

            let rm_val: u8 = (second_byte & rm_mask) << 5;
            let mod_val: u8 = second_byte & mod_mask;
            let operation_val: u8 = (second_byte & operation_mask) << 2;

            // set operation
            match operation_val {
                0b0000_0000 => {
                    ret.instruction = Instruction::Add;
                }

                0b1010_0000 => {
                    ret.instruction = Instruction::Sub;
                }

                0b1110_0000 => {
                    ret.instruction = Instruction::Cmp;
                }

                _ => {
                    panic!("Unknown operation");
                }
            }

            // get results
            let mod_results = match handle_mod(mod_val, 0, rm_val, w_val, asm) {
                Some(v) => v,
                None => return None,
            };
            data.dest = mod_results.rm_address;

            let data_first: u8 = match asm.pull_byte() {
                Some(v) => *v,
                None => return None,
            };
            data.immediate = data_first as u16;
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

    /*
    println!("mod {mod_val:#8b}");
    println!("reg {reg_val:#8b}");
    println!("rm  {rm_val:#8b}");
    println!("");
    */

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

                0b0110_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bp,
                        second_operand: Register::Di,
                        offset: 0,
                    });
                }

                0b1000_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Si,
                        second_operand: Register::None,
                        offset: 0,
                    });
                }

                0b1010_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Di,
                        second_operand: Register::None,
                        offset: 0,
                    });
                }

                0b1110_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bx,
                        second_operand: Register::None,
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
            let offset: u8 = match asm.pull_byte() {
                Some(v) => *v,
                None => return None,
            };

            match rm_val {
                0b0000_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bx,
                        second_operand: Register::Si,
                        offset: offset as u16,
                    });
                }

                0b0010_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bx,
                        second_operand: Register::Di,
                        offset: offset as u16,
                    });
                }

                0b0100_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bp,
                        second_operand: Register::Si,
                        offset: offset as u16,
                    });
                }

                0b0110_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bp,
                        second_operand: Register::Di,
                        offset: offset as u16,
                    });
                }

                0b1000_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Si,
                        second_operand: Register::None,
                        offset: offset as u16,
                    });
                }

                0b1010_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Di,
                        second_operand: Register::None,
                        offset: offset as u16,
                    });
                }

                0b1100_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bp,
                        second_operand: Register::None,
                        offset: offset as u16,
                    });
                }

                0b1110_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bx,
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
            let low: u8 = match asm.pull_byte() {
                Some(v) => *v,
                None => return None,
            };
            let high: u8 = match asm.pull_byte() {
                Some(v) => *v,
                None => return None,
            };

            let offset = combine(low, high);

            match rm_val {
                0b0000_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bx,
                        second_operand: Register::Si,
                        offset: offset,
                    });
                }

                0b0010_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bx,
                        second_operand: Register::Di,
                        offset: offset,
                    });
                }

                0b0100_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bp,
                        second_operand: Register::Si,
                        offset: offset,
                    });
                }

                0b0110_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bp,
                        second_operand: Register::Di,
                        offset: offset,
                    });
                }

                0b1000_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Si,
                        second_operand: Register::None,
                        offset: offset,
                    });
                }

                0b1010_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Di,
                        second_operand: Register::None,
                        offset: offset,
                    });
                }

                0b1100_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bp,
                        second_operand: Register::None,
                        offset: offset,
                    });
                }

                0b1110_0000 => {
                    results.rm_address = Address::EffectiveAddress(EffectiveAddress {
                        first_operand: Register::Bx,
                        second_operand: Register::None,
                        offset: offset,
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

struct CommandMask {
    mask: u8,
    instruction_value: u8,
    command: Command,
}

const REG_MEM_TO_REG_MEM_MASK: u8 = 0b_1111_1100;
const IMMEDIATE_REG_MASK: u8 = 0b_1111_0000;
const ASC_IMMEDIATE_REG_MEM_MASK: u8 = 0b_1111_1100;
const ASC_IMMEDIATE_ACCUMULATOR_MASK: u8 = 0b_1111_1110;
const JMP_MASK: u8 = 0b_1111_1111;

static COMMAND_MASKS: [CommandMask; 29] = [
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_1110_0011,
        command: Command {
            instruction: Instruction::Jcxz,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_1110_0000,
        command: Command {
            instruction: Instruction::Loopnz,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_1110_0001,
        command: Command {
            instruction: Instruction::Loopz,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_1110_0010,
        command: Command {
            instruction: Instruction::Loop,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_1001,
        command: Command {
            instruction: Instruction::Jns,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_0001,
        command: Command {
            instruction: Instruction::Jno,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_1011,
        command: Command {
            instruction: Instruction::Jnp,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_0111,
        command: Command {
            instruction: Instruction::Ja,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_0011,
        command: Command {
            instruction: Instruction::Jnb,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_1111,
        command: Command {
            instruction: Instruction::Jg,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_1101,
        command: Command {
            instruction: Instruction::Jnl,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_1000,
        command: Command {
            instruction: Instruction::Js,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_0000,
        command: Command {
            instruction: Instruction::Jo,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_1010,
        command: Command {
            instruction: Instruction::Jp,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_0110,
        command: Command {
            instruction: Instruction::Jbe,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_0010,
        command: Command {
            instruction: Instruction::Jb,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_1110,
        command: Command {
            instruction: Instruction::Jle,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_1100,
        command: Command {
            instruction: Instruction::Jl,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_0100,
        command: Command {
            instruction: Instruction::Je,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: JMP_MASK,
        instruction_value: 0b_0111_0101,
        command: Command {
            instruction: Instruction::Jnz,
            encoding: Encoding::Jump(0),
        },
    },
    CommandMask {
        mask: ASC_IMMEDIATE_ACCUMULATOR_MASK,
        instruction_value: 0b_0000_0100,
        command: Command {
            instruction: Instruction::Add,
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                immediate: 0,
                dest: Register::None,
            }),
        },
    },
    CommandMask {
        mask: ASC_IMMEDIATE_ACCUMULATOR_MASK,
        instruction_value: 0b_0010_1100,
        command: Command {
            instruction: Instruction::Sub,
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                immediate: 0,
                dest: Register::None,
            }),
        },
    },
    CommandMask {
        mask: ASC_IMMEDIATE_ACCUMULATOR_MASK,
        instruction_value: 0b_0011_1100,
        command: Command {
            instruction: Instruction::Cmp,
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                immediate: 0,
                dest: Register::None,
            }),
        },
    },
    CommandMask {
        mask: IMMEDIATE_REG_MASK,
        instruction_value: 0b_1011_0000,
        command: Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                immediate: 0,
                dest: Register::None,
            }),
        },
    },
    CommandMask {
        mask: ASC_IMMEDIATE_REG_MEM_MASK,
        instruction_value: 0b_1000_0000,
        command: Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                immediate: 0,
                dest: Address::Register(Register::None),
            }),
        },
    },
    CommandMask {
        mask: REG_MEM_TO_REG_MEM_MASK,
        instruction_value: 0b_1000_1000,
        command: Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::None),
                dest: Address::Register(Register::None),
            }),
        },
    },
    CommandMask {
        mask: REG_MEM_TO_REG_MEM_MASK,
        instruction_value: 0b_0000_0000,
        command: Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::None),
                dest: Address::Register(Register::None),
            }),
        },
    },
    CommandMask {
        mask: REG_MEM_TO_REG_MEM_MASK,
        instruction_value: 0b_0010_1000,
        command: Command {
            instruction: Instruction::Sub,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::None),
                dest: Address::Register(Register::None),
            }),
        },
    },
    CommandMask {
        mask: REG_MEM_TO_REG_MEM_MASK,
        instruction_value: 0b_0011_1000,
        command: Command {
            instruction: Instruction::Cmp,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::None),
                dest: Address::Register(Register::None),
            }),
        },
    },
];

fn get_command(byte: u8) -> Command {
    for c_mask in &COMMAND_MASKS {
        if (byte & c_mask.mask) == c_mask.instruction_value {
            return c_mask.command.clone();
        }
    }
    panic!("Unkown instruction {byte:#8b}");
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
