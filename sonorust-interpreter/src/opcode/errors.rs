use std::fmt;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

macro_rules! error {
    [$name:ty = ($self:tt, $f:tt) $item:block] => {
        impl std::error::Error for $name {}
        impl fmt::Display for $name {
            fn fmt(&$self, $f: &mut fmt::Formatter<'_>) -> fmt::Result $item
        }
    };
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct UnknownOpCode(pub String);

error! {
    UnknownOpCode = (self, f) {
        write!(f, "unknown opcode: {}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidArgumentCount {
    pub function: &'static str,
    pub expected_min: usize,
    pub actual: usize,
}

error! {
    InvalidArgumentCount = (self, f) {
        let Self { function, expected_min, actual } = self;
        write!(f, "incorrect number of arguments for {function}. expected at least {expected_min}, got {actual}.")
    }
}
