use crate::simulator::tests::*;

#[test]
fn listing_0051() {
    let mut computer = Computer::new("resources/listings/listing_0051_memory_mov");

    sim_one(&mut computer);
    assert_eq!(computer.memory[1000], 1);

    sim_one(&mut computer);
    assert_eq!(computer.memory[1002], 2);

    sim_one(&mut computer);
    assert_eq!(computer.memory[1004], 3);

    sim_one(&mut computer);
    assert_eq!(computer.memory[1006], 4);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.b, 1000);

    sim_one(&mut computer);
    assert_eq!(computer.memory[1004], 10);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.b, 1);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.c, 2);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.d, 10);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.bp, 4);
}
