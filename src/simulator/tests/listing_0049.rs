use crate::simulator::tests::*;

#[test]
fn listing_0049() {
    let mut computer = Computer::new("resources/listings/listing_0049_conditional_jumps");

    sim_one(&mut computer);
    assert_eq!(computer.cpu.c, 3);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.b, 1000);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.b, 1010);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.c, 2);

    // jump
    sim_one(&mut computer);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.b, 1020);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.c, 1);

    // jump
    sim_one(&mut computer);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.b, 1030);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.c, 0);
    assert_eq!(computer.cpu.zero_flag, true);

}
