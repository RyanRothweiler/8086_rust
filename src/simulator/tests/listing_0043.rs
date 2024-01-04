use crate::simulator::tests::*;

#[test]
fn listing_0043() {
    let mut asm = Asm::new("resources/listings/listing_0043_immediate_movs");
    let mut cpu = Cpu::new();

    // parse instructions
    loop {
        let command = parser::pull_command(&mut asm);
        let command = match command {
            Some(v) => v,
            None => break,
        };

        simulate(&mut cpu, command);
    }

    assert!(cpu.a == 1);
    assert!(cpu.b == 2);
    assert!(cpu.c == 3);
    assert!(cpu.d == 4);
    assert!(cpu.sp == 5);
    assert!(cpu.bp == 6);
    assert!(cpu.si == 7);
    assert!(cpu.di == 8);
}
