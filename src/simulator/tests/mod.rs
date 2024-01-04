#![allow(unused_variables, dead_code, unused_imports)]

use super::*;

mod listing_0043;
mod listing_0044;

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
