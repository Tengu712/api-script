#[macro_use]
pub mod cell;
pub mod grammer;
#[cfg(test)]
pub mod test;

pub use cell::*;
pub use grammer::*;

use super::lexer::*;

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
