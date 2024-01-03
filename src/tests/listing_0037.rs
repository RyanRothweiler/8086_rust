use crate::*;
use crate::tests::*;

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
