#![forbid(unsafe_code)]

//! Length-prefixed line protocol parser.

use protocol::Frame;
use thiserror::Error;

/// Maximum accepted payload bytes.
pub const MAX_PAYLOAD: usize = 4096;

/// Protocol decoding failures.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ParseError {
    /// Header does not contain `id:length:`.
    #[error("malformed frame header")]
    MalformedHeader,
    /// Numeric field is invalid.
    #[error("invalid numeric field")]
    InvalidNumber,
    /// Declared payload is too large.
    #[error("payload exceeds 4096 bytes")]
    PayloadTooLarge,
    /// Declared and actual payload lengths differ.
    #[error("payload length mismatch")]
    LengthMismatch,
    /// Payload is not valid UTF-8.
    #[error("payload is not valid UTF-8")]
    InvalidUtf8,
}

/// Parse an `id:length:payload` frame without lossy conversion.
pub fn parse_frame(input: &[u8]) -> Result<Frame, ParseError> {
    let first = input
        .iter()
        .position(|byte| *byte == b':')
        .ok_or(ParseError::MalformedHeader)?;
    let second_offset = input[first + 1..]
        .iter()
        .position(|byte| *byte == b':')
        .ok_or(ParseError::MalformedHeader)?;
    let second = first + 1 + second_offset;
    let id = std::str::from_utf8(&input[..first])
        .map_err(|_| ParseError::InvalidNumber)?
        .parse()
        .map_err(|_| ParseError::InvalidNumber)?;
    let length: usize = std::str::from_utf8(&input[first + 1..second])
        .map_err(|_| ParseError::InvalidNumber)?
        .parse()
        .map_err(|_| ParseError::InvalidNumber)?;
    if length > MAX_PAYLOAD {
        return Err(ParseError::PayloadTooLarge);
    }
    let payload = &input[second + 1..];
    if payload.len() != length {
        return Err(ParseError::LengthMismatch);
    }
    let payload = std::str::from_utf8(payload)
        .map_err(|_| ParseError::InvalidUtf8)?
        .to_owned();
    Ok(Frame { id, payload })
}

/// Encode a frame in the canonical wire representation.
#[must_use]
pub fn encode_frame(frame: &Frame) -> Vec<u8> {
    format!("{}:{}:{}", frame.id, frame.payload.len(), frame.payload).into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn valid_payloads_round_trip(id in any::<u32>(), payload in "[a-zA-Z0-9 :]{0,64}") {
            let frame = Frame { id: u64::from(id), payload };
            prop_assert_eq!(parse_frame(&encode_frame(&frame)), Ok(frame));
        }
    }

    #[test]
    fn malformed_input_never_panics() {
        for input in [b"".as_slice(), b"1", b"1:x:a", b"1:9:a", &[0xff, 0xfe]] {
            assert!(parse_frame(input).is_err());
        }
    }
}
