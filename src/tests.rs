use super::*;

fn validate_next_command(truth: Command, asm: &mut Asm) {
    let command = parser::pull_command(asm).expect("Error pulling command");
    assert_eq!(command, truth);
}

#[test]
fn register_to_register() {
    let mut asm = Asm::new("resources/listings/listing_0037_single_register_mov");
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Bx,
            dest: Register::Cx,
        },
        &mut asm,
    );
}

#[test]
fn register_to_register_all() {
    let mut asm = Asm::new("resources/listings/listing_0038_many_register_mov");
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Bx,
            dest: Register::Cx,
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Ah,
            dest: Register::Ch,
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Bx,
            dest: Register::Dx,
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Bx,
            dest: Register::Si,
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Di,
            dest: Register::Bx,
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Cl,
            dest: Register::Al,
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Ch,
            dest: Register::Ch,
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Ax,
            dest: Register::Bx,
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Si,
            dest: Register::Bx,
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Di,
            dest: Register::Sp,
        },
        &mut asm,
    );
    validate_next_command(
        Command {
            instruction: Instruction::Mov,
            source: Register::Ax,
            dest: Register::Bp,
        },
        &mut asm,
    );

}
