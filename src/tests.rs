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
    let truth = Command {
        instruction: Instruction::Mov,
        source: Register::Bx,
        dest: Register::Cx,
    };
    assert_eq!(command, truth);
}

