use crate::tests::*;

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


