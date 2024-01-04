use crate::simulator::tests::*;

#[test]
fn listing_0044() {
    let mut cpu = Cpu::new();

    full_simulate("resources/listings/listing_0044_register_movs", &mut cpu);

    assert_eq!(cpu.a, 4);
    assert_eq!(cpu.b, 3);
    assert_eq!(cpu.c, 2);
    assert_eq!(cpu.d, 1);
    assert_eq!(cpu.sp, 1);
    assert_eq!(cpu.bp, 2);
    assert_eq!(cpu.si, 3);
    assert_eq!(cpu.di, 4);
}
