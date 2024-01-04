use crate::parser::tests::*;

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

