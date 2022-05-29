pub mod arg;
pub mod args;
pub mod block;
pub mod blocks;
pub mod data;
pub mod logic;
pub mod logics;
pub mod program;
pub mod type_;

pub use arg::*;
pub use args::*;
pub use block::*;
pub use blocks::*;
pub use data::*;
pub use logic::*;
pub use logics::*;
pub use program::*;
pub use type_::*;

use super::*;

fn error(place: &str, expect: &str, found: Option<&Token>) -> ! {
    if let Some(n) = found {
        panic!(
            "{}",
            format!(
                "[Parser error] {} : {} expected, but '{:?}' found.",
                place, expect, n
            )
        );
    } else {
        panic!(
            "{}",
            format!(
                "[Parser error] {} : {} expected, but no token found.",
                place, expect
            )
        );
    }
}

pub fn parse(tokens: &mut Tokens) -> program::Program {
    program::Program::parse(tokens)
}
