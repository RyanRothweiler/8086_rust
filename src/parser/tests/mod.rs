#![allow(unused_variables, dead_code)]

use super::*;

mod listing_0037;
mod listing_0038;
mod listing_0039;
mod listing_0041;
mod listing_0046;
mod listing_0051;

fn validate_next_command(truth: Command, computer: &mut Computer) {
    let command = parser::pull_command(computer).expect("Error pulling command");
    assert_eq!(command, truth);
}
