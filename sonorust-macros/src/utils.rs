use syn::{
    Result,
    parse::{Parse, ParseBuffer},
};

pub fn parse_<T: Parse, Sep: Parse>(input: &ParseBuffer<'_>) -> Result<T> {
    let name = input.parse()?;
    // Optional sep after individual item
    let _ = input.parse::<Sep>();
    Ok(name)
}
