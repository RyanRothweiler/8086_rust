#![allow(unused_variables, dead_code, unused_imports)]

use super::*;

mod listing_0043;
mod listing_0044;
mod listing_0046;

fn full_simulate(file: &str, cpu: &mut Cpu) {
    let mut asm = Asm::new(file);

    // parse instructions
    loop {
        let command = parser::pull_command(&mut asm);
        let command = match command {
            Some(v) => v,
            None => break,
        };

        cpu.simulate(command);
    }
}

fn sim_one(cpu: &mut Cpu, asm: &mut Asm) {
    let command = parser::pull_command(asm);
    let command = match command {
        Some(v) => v,
        None => panic!("Ran out of commands"),
    };

    cpu.simulate(command);
}
