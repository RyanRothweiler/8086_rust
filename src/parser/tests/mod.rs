#![allow(unused_variables, dead_code)]

use super::*;

mod listing_0037;
mod listing_0038;
mod listing_0039;
mod listing_0041;
mod listing_0046;

fn validate_next_command(truth: Command, asm: &mut Asm) {
    let command = parser::pull_command(asm).expect("Error pulling command");
    assert_eq!(command, truth);
}
