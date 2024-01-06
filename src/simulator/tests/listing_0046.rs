use crate::simulator::tests::*;

#[test]
fn listing_0046() {
    let mut cpu = Cpu::new();
    let mut asm = Asm::new("resources/listings/listing_0046_add_sub_cmp");

    sim_one(&mut cpu, &mut asm);
    assert_eq!(cpu.b, -4093);
    assert_eq!(cpu.signed_flag, false);

    sim_one(&mut cpu, &mut asm);
    assert_eq!(cpu.c, 3841);
    assert_eq!(cpu.signed_flag, false);

    sim_one(&mut cpu, &mut asm);
    assert_eq!(cpu.b, -7934);
    assert_eq!(cpu.signed_flag, true);

    sim_one(&mut cpu, &mut asm);
    assert_eq!(cpu.sp, 998);

    sim_one(&mut cpu, &mut asm);
    assert_eq!(cpu.bp, 999);

    sim_one(&mut cpu, &mut asm);
    assert_eq!(cpu.sp, 998);
    assert_eq!(cpu.bp, 999);
    assert_eq!(cpu.signed_flag, false);
    assert_eq!(cpu.zero_flag, false);

    sim_one(&mut cpu, &mut asm);
    assert_eq!(cpu.bp, 2026);
    assert_eq!(cpu.signed_flag, false);
    assert_eq!(cpu.zero_flag, false);

    sim_one(&mut cpu, &mut asm);
    assert_eq!(cpu.bp, 0);
    assert_eq!(cpu.signed_flag, false);
    assert_eq!(cpu.zero_flag, true);
}
