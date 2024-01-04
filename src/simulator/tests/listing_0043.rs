use crate::simulator::tests::*;

#[test]
fn listing_0043() {
    let mut cpu = Cpu::new();

    full_simulate("resources/listings/listing_0043_immediate_movs", &mut cpu);

    assert!(cpu.a == 1);
    assert!(cpu.b == 2);
    assert!(cpu.c == 3);
    assert!(cpu.d == 4);
    assert!(cpu.sp == 5);
    assert!(cpu.bp == 6);
    assert!(cpu.si == 7);
    assert!(cpu.di == 8);
}
