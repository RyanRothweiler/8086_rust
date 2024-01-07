use crate::parser::tests::*;

#[test]
fn listing_0046() {
    let mut computer = Computer::new("resources/listings/listing_0046_add_sub_cmp");

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                dest: Register::Bx,
                immediate: (4093 * -1) as u16,
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                dest: Register::Cx,
                immediate: 3841,
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Sub,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bx),
                source: Address::Register(Register::Cx),
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                dest: Register::Sp,
                immediate: 998,
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            encoding: Encoding::ImmediateToReg(ImmediateToReg {
                dest: Register::Bp,
                immediate: 999,
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Cmp,
            encoding: Encoding::RegMemToRegMem(RegMemToRegMem {
                dest: Address::Register(Register::Bp),
                source: Address::Register(Register::Sp),
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Add,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::Register(Register::Bp),
                immediate: 1027,
            }),
        },
        &mut computer,
    );

    validate_next_command(
        Command {
            instruction: Instruction::Sub,
            encoding: Encoding::ImmediateToRegMem(ImmediateToRegMem {
                dest: Address::Register(Register::Bp),
                immediate: 2026,
            }),
        },
        &mut computer,
    );

}
