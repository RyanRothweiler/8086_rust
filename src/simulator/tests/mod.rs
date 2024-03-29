#![allow(unused_variables, dead_code, unused_imports)]

use super::*;

mod listing_0043;
mod listing_0044;
mod listing_0046;
mod listing_0048;
mod listing_0049;
mod listing_0051;
mod listing_0052;

fn full_simulate(computer: &mut Computer) {

    // parse instructions
    loop {
        let command = parser::pull_command(computer);
        let command = match command {
            Some(v) => v,
            None => break,
        };

        computer.simulate(command);
    }
}

fn sim_one(computer: &mut Computer) {
    let command = parser::pull_command(computer);
    let command = match command {
        Some(v) => v,
        None => panic!("Ran out of commands"),
    };

    computer.simulate(command);
}
