#![forbid(unsafe_code)]

pub const MAX_PAYLOAD: usize = 4096;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError { Header, InvalidLength, TooLarge, LengthMismatch, DanglingEscape }

pub fn parse(input: &str) -> Result<String, ParseError> {
    let (length, encoded) = input.split_once('|').ok_or(ParseError::Header)?;
    let length: usize = length.parse().map_err(|_| ParseError::InvalidLength)?;
    let mut payload = String::with_capacity(length);
    if length > MAX_PAYLOAD { return Err(ParseError::TooLarge); }
    payload.push_str(&encoded.replace("\\|", "|"));
    if payload.len() != length { return Err(ParseError::LengthMismatch); }
    Ok(payload)
}
