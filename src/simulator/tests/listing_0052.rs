use crate::simulator::tests::*;

#[test]
fn listing_0052() {
    let mut computer = Computer::new("resources/listings/listing_0052_memory_add_loop");

    full_simulate(&mut computer);

    assert_eq!(computer.cpu.b, 6);
    assert_eq!(computer.cpu.c, 4);
    assert_eq!(computer.cpu.d, 6);
    assert_eq!(computer.cpu.bp, 1000);
    assert_eq!(computer.cpu.si, 6);
    assert_eq!(computer.cpu.instruction_pointer, 36);

    assert_eq!(computer.cpu.zero_flag, true);
}
