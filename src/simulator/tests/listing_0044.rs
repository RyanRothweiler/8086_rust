use crate::simulator::tests::*;

#[test]
fn listing_0044() {
    let mut computer = Computer::new("resources/listings/listing_0044_register_movs");

    full_simulate(&mut computer);

    assert_eq!(computer.cpu.a, 4);
    assert_eq!(computer.cpu.b, 3);
    assert_eq!(computer.cpu.c, 2);
    assert_eq!(computer.cpu.d, 1);
    assert_eq!(computer.cpu.sp, 1);
    assert_eq!(computer.cpu.bp, 2);
    assert_eq!(computer.cpu.si, 3);
    assert_eq!(computer.cpu.di, 4);
}
