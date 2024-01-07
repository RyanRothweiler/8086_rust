use crate::simulator::tests::*;

#[test]
fn listing_0043() {
    let mut computer = Computer::new("resources/listings/listing_0043_immediate_movs");

    full_simulate(&mut computer);

    assert!(computer.cpu.a == 1);
    assert!(computer.cpu.b == 2);
    assert!(computer.cpu.c == 3);
    assert!(computer.cpu.d == 4);
    assert!(computer.cpu.sp == 5);
    assert!(computer.cpu.bp == 6);
    assert!(computer.cpu.si == 7);
    assert!(computer.cpu.di == 8);
}
