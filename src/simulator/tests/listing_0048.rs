use crate::simulator::tests::*;

#[test]
fn listing_0048() {
    let mut computer = Computer::new("resources/listings/listing_0048_ip_register");

    sim_one(&mut computer);
    assert_eq!(computer.cpu.c, 200);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.b, 200);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.c, 1200);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.b, 2000);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.c, -800);
}
