use super::*;

fn validate_next_command(truth: Command, asm: &mut Asm) {
    let command = parser::pull_command(asm).expect("Error pulling command");
    assert_eq!(command, truth);
}

#[test]
fn register_to_register() {
    //mov cx, bx
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
