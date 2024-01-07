use crate::simulator::tests::*;

#[test]
fn listing_0046() {
    let mut computer = Computer::new("resources/listings/listing_0046_add_sub_cmp");

    sim_one(&mut computer);
    assert_eq!(computer.cpu.b, -4093);
    assert_eq!(computer.cpu.signed_flag, false);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.c, 3841);
    assert_eq!(computer.cpu.signed_flag, false);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.b, -7934);
    assert_eq!(computer.cpu.signed_flag, true);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.sp, 998);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.bp, 999);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.sp, 998);
    assert_eq!(computer.cpu.bp, 999);
    assert_eq!(computer.cpu.signed_flag, false);
    assert_eq!(computer.cpu.zero_flag, false);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.bp, 2026);
    assert_eq!(computer.cpu.signed_flag, false);
    assert_eq!(computer.cpu.zero_flag, false);

    sim_one(&mut computer);
    assert_eq!(computer.cpu.bp, 0);
    assert_eq!(computer.cpu.signed_flag, false);
    assert_eq!(computer.cpu.zero_flag, true);
}
