use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Error {
    UnknownOpCode(String),
    InvalidArgumentCount {
        function: &'static str,
        expected_min: usize,
        actual: usize,
    },
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::UnknownOpCode(opcode_name) => write!(f, "unknown opcode: {}", opcode_name),
            Error::InvalidArgumentCount {
                function,
                expected_min,
                actual,
            } => write!(
                f,
                "incorrect number of arguments for {function}. expected at least {expected_min}, got {actual}."
            ),
        }
    }
}
