use super::*;

fn validate_next_command(truth: Command, asm: &mut Asm) {
    let command = parser::pull_command(asm).expect("Error pulling command");
    assert_eq!(command, truth);
}

#[test]
fn listing_0037() {
    let mut asm = Asm::new("resources/listings/listing_0037_single_register_mov");
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Bx),
                dest: Address::Register(Register::Cx),
            }),
        },
        &mut asm,
    );
}

#[test]
fn listing_0038() {
    let mut asm = Asm::new("resources/listings/listing_0038_many_register_mov");
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Bx),
                dest: Address::Register(Register::Cx),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Ah),
                dest: Address::Register(Register::Ch),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Bx),
                dest: Address::Register(Register::Dx),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Bx),
                dest: Address::Register(Register::Si),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Di),
                dest: Address::Register(Register::Bx),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Cl),
                dest: Address::Register(Register::Al),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Ch),
                dest: Address::Register(Register::Ch),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Ax),
                dest: Address::Register(Register::Bx),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Si),
                dest: Address::Register(Register::Bx),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Di),
                dest: Address::Register(Register::Sp),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Ax),
                dest: Address::Register(Register::Bp),
            }),
        },
        &mut asm,
    );
}

#[test]
fn listing_0039() {
    let mut asm = Asm::new("resources/listings/listing_0039_more_movs");
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Bx),
                dest: Address::Register(Register::Si),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::Register(Register::Al),
                dest: Address::Register(Register::Dh),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                dest: Register::Cl,
                immediate: 12,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                dest: Register::Ch,
                immediate: 244,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                dest: Register::Cx,
                immediate: 12,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                dest: Register::Cx,
                immediate: 65524,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                dest: Register::Dx,
                immediate: 3948,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                dest: Register::Dx,
                immediate: 61588,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Al),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Si,
                    offset: 0,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bx),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Di,
                    offset: 0,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Dx),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Ah),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Si,
                    offset: 4,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Al),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Si,
                    offset: 4999,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Di,
                    offset: 0,
                }),
                source: Address::Register(Register::Cx),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Si,
                    offset: 0,
                }),
                source: Address::Register(Register::Cl),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                }),
                source: Address::Register(Register::Ch),
            }),
        },
        &mut asm,
    );
}

#[test]
fn listing_0041() {
    let mut asm = Asm::new("resources/listings/listing_0041_add_sub_cmp_jnz");
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bx),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Si,
                    offset: 0,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bx),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::Register(Register::Si),
                immediate: 2,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::Register(Register::Bp),
                immediate: 2,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::Register(Register::Cx),
                immediate: 8,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bx),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Cx),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::None,
                    offset: 2,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bh),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Si,
                    offset: 4,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Di),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Di,
                    offset: 6,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Si,
                    offset: 0,
                }),
                source: Address::Register(Register::Bx),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                }),
                source: Address::Register(Register::Bx),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                }),
                source: Address::Register(Register::Bx),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::None,
                    offset: 2,
                }),
                source: Address::Register(Register::Cx),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Si,
                    offset: 4,
                }),
                source: Address::Register(Register::Bh),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Di,
                    offset: 6,
                }),
                source: Address::Register(Register::Di),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::None,
                    offset: 0,
                }),
                immediate: 34,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Si,
                    offset: 1000,
                }),
                immediate: 29,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Ax),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Al),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Si,
                    offset: 0,
                }),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Ax),
                source: Address::Register(Register::Bx),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Al),
                source: Address::Register(Register::Ah),
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                dest: Register::Ax,
                immediate: 1000,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                dest: Register::Al,
                immediate: 226,
            }),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                dest: Register::Al,
                immediate: 9,
            }),
        },
        &mut asm,
    );

}
