use crate::parser::tests::*;

#[test]
fn listing_0051() {
    let mut computer = Computer::new("resources/listings/listing_0051_memory_mov");

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::None,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 1000,
                }),
                immediate: 1,
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::None,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 1002,
                }),
                immediate: 2,
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::None,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 1004,
                }),
                immediate: 3,
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::None,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 1006,
                }),
                immediate: 4,
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                immediate: 1000,
                dest: Register::Bx,
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::Bx,
                    second_operand: Register::None,
                    offset: 4,
                    direct: 0,
                }),
                immediate: 10,
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::None,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 1000,
                }),
                dest: Address::Register(Register::Bx),
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::None,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 1002,
                }),
                dest: Address::Register(Register::Cx),
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::None,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 1004,
                }),
                dest: Address::Register(Register::Dx),
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                source: Address::EffectiveAddress(EffectiveAddress {
                    first_operand: Register::None,
                    second_operand: Register::None,
                    offset: 0,
                    direct: 1006,
                }),
                dest: Address::Register(Register::Bp),
            }),
        },
        &mut computer,
    );
}
