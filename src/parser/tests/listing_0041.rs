use crate::parser::tests::*;

// the same commands are used for add / sub / cmp
fn test_list(instruction: Instruction, asm: &mut Asm) {
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bx),
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Si,
                    offset: 0,
                }),
            }),
        },
        asm,
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
                }),
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::Register(Register::Si),
                immediate: 2,
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::Register(Register::Bp),
                immediate: 2,
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::Register(Register::Cx),
                immediate: 8,
            }),
        },
        asm,
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
                }),
            }),
        },
        asm,
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
                }),
            }),
        },
        asm,
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
                }),
            }),
        },
        asm,
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
                }),
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::Si,
                    offset: 0,
                }),
                source: Address::Register(Register::Bx),
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                }),
                source: Address::Register(Register::Bx),
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::None,
                    offset: 0,
                }),
                source: Address::Register(Register::Bx),
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::None,
                    offset: 2,
                }),
                source: Address::Register(Register::Cx),
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Si,
                    offset: 4,
                }),
                source: Address::Register(Register::Bh),
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Di,
                    offset: 6,
                }),
                source: Address::Register(Register::Di),
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::None,
                    offset: 0,
                }),
                immediate: 34,
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bp,
                    second_operand: Register::Si,
                    offset: 1000,
                }),
                immediate: 29,
            }),
        },
        asm,
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
                }),
            }),
        },
        asm,
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
                }),
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Ax),
                source: Address::Register(Register::Bx),
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Al),
                source: Address::Register(Register::Ah),
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                dest: Register::Ax,
                immediate: 1000,
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                dest: Register::Al,
                immediate: 226,
            }),
        },
        asm,
    );
    validate_next_command(
        Command {
            instruction: instruction.clone(),
            encoding: Encoding::ImmediateToAccumulator(ImmediateToAccumulator {
                dest: Register::Al,
                immediate: 9,
            }),
        },
        asm,
    );
}
#[test]
fn listing_0041() {
    let mut asm = Asm::new("resources/listings/listing_0041_add_sub_cmp_jnz");
    test_list(Instruction::Add, &mut asm);
    test_list(Instruction::Sub, &mut asm);
    test_list(Instruction::Cmp, &mut asm);

    validate_next_command(
        Command {
            instruction: Instruction::Jnz,
            encoding: Encoding::Jump(2),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jnz,
            encoding: Encoding::Jump(-4),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jnz,
            encoding: Encoding::Jump(-6),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jnz,
            encoding: Encoding::Jump(-4),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Je,
            encoding: Encoding::Jump(-2),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jl,
            encoding: Encoding::Jump(-4),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jle,
            encoding: Encoding::Jump(-6),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jb,
            encoding: Encoding::Jump(-8),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jbe,
            encoding: Encoding::Jump(-10),
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Jp,
            encoding: Encoding::Jump(-12),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jo,
            encoding: Encoding::Jump(-14),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Js,
            encoding: Encoding::Jump(-16),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jnz,
            encoding: Encoding::Jump(-18),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jnl,
            encoding: Encoding::Jump(-20),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jg,
            encoding: Encoding::Jump(-22),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jnb,
            encoding: Encoding::Jump(-24),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Ja,
            encoding: Encoding::Jump(-26),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jnp,
            encoding: Encoding::Jump(-28),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jno,
            encoding: Encoding::Jump(-30),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jns,
            encoding: Encoding::Jump(-32),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Loop,
            encoding: Encoding::Jump(-34),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Loopz,
            encoding: Encoding::Jump(-36),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Loopnz,
            encoding: Encoding::Jump(-38),
        },
        &mut asm,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Jcxz,
            encoding: Encoding::Jump(-40),
        },
        &mut asm,
    );
}
