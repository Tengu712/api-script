mod input;
mod output;

use super::automata::dfa::DFAutomata;
use super::*;

pub fn compile_lexer() -> Result<(), String> {
    let (dfa, labels) = input::input()?;
    output::output(dfa, labels)
}
