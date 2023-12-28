use super::*;

#[test]
fn register_to_register() {
    //mov cx, bx
    let mut asm: Vec<u8> = Vec::new();
    asm.push(137);
    asm.push(217);

    let mut asm = Asm {
        list: asm,
        index: 0,
    };

    let command = parser::pull_command(&mut asm).expect("Error pulling command");

    match command.instruction {
        Instruction::Mov => {}
        _ => panic!("Incorrect instruction"),
    }

    match command.source {
        Register::Bx => {}
        _ => panic!("Incorrect source register"),
    }

    match command.dest {
        Register::Cx => {}
        _ => panic!("Incorrect dest register"),
    }
}
