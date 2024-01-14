use crate::parser::tests::*;

// the same commands are used for add / sub / cmp
fn test_list(instruction: Instruction, computer: &mut Computer) {
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bx),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Si,
                    offset: 0,
                    direct: 0,
                }),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bx),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 0,
                }),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::Register(Register::Si),
                immediate: 2,
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::Register(Register::Bp),
                immediate: 2,
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::Register(Register::Cx),
                immediate: 8,
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bx),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 0,
                }),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Cx),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::None,
                    offset: 2,
                    direct: 0,
                }),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bh),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Si,
                    offset: 4,
                    direct: 0,
                }),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Di),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Di,
                    offset: 6,
                    direct: 0,
                }),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Si,
                    offset: 0,
                    direct: 0,
                }),
                source: Address::Register(Register::Bx),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 0,
                }),
                source: Address::Register(Register::Bx),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 0,
                }),
                source: Address::Register(Register::Bx),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::None,
                    offset: 2,
                    direct: 0,
                }),
                source: Address::Register(Register::Cx),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Si,
                    offset: 4,
                    direct: 0,
                }),
                source: Address::Register(Register::Bh),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Di,
                    offset: 6,
                    direct: 0,
                }),
                source: Address::Register(Register::Di),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 0,
                }),
                immediate: 34,
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Si,
                    offset: 1000,
                    direct: 0,
                }),
                immediate: 29,
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Ax),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 0,
                }),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Al),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Si,
                    offset: 0,
                    direct: 0,
                }),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Ax),
                source: Address::Register(Register::Bx),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Al),
                source: Address::Register(Register::Ah),
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                dest: Register::Ax,
                immediate: 1000,
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                dest: Register::Al,
                immediate: 226,
            }),
        },
        computer,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                dest: Register::Al,
                immediate: 9,
            }),
        },
        computer,
    );
}
#[test]
fn listing_0041() {
    let mut computer = Computer::new("resources/listings/listing_0041_add_sub_cmp_jnz");
    test_list(Instruction::Add, &mut computer);
    test_list(Instruction::Sub, &mut computer);
    test_list(Instruction::Cmp, &mut computer);

    validate_next_command(
        Command {
            instruction: Instruction::Jnz,
            encoding: Encoding::Jump(2),
        },
        &mut computer,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jnz,
            encoding: Encoding::Jump(-4),
        },
        &mut computer,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jnz,
            encoding: Encoding::Jump(-6),
        },
        &mut computer,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jnz,
            encoding: Encoding::Jump(-4),
        },
        &mut computer,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Je,
            encoding: Encoding::Jump(-2),
        },
        &mut computer,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jl,
            encoding: Encoding::Jump(-4),
        },
        &mut computer,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jle,
            encoding: Encoding::Jump(-6),
        },
        &mut computer,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jb,
            encoding: Encoding::Jump(-8),
        },
        &mut computer,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jbe,
            encoding: Encoding::Jump(-10),
        },
        &mut computer,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jp,
            encoding: Encoding::Jump(-12),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jo,
            encoding: Encoding::Jump(-14),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Js,
            encoding: Encoding::Jump(-16),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jnz,
            encoding: Encoding::Jump(-18),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jnl,
            encoding: Encoding::Jump(-20),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jg,
            encoding: Encoding::Jump(-22),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jnb,
            encoding: Encoding::Jump(-24),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Ja,
            encoding: Encoding::Jump(-26),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jnp,
            encoding: Encoding::Jump(-28),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jno,
            encoding: Encoding::Jump(-30),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jns,
            encoding: Encoding::Jump(-32),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Loop,
            encoding: Encoding::Jump(-34),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Loopz,
            encoding: Encoding::Jump(-36),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Loopnz,
            encoding: Encoding::Jump(-38),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jcxz,
            encoding: Encoding::Jump(-40),
        },
        &mut computer,
    );
}
