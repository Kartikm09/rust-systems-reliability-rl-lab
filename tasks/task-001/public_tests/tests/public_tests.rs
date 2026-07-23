use candidate_task::{ParseError, parse};

#[test]
fn escaped_separator_uses_decoded_length() {
    assert_eq!(parse(r"4|a\|bc"), Ok("a|bc".to_owned()));
}

#[test]
fn oversized_frame_is_rejected() {
    let input = format!("5000|{}", "a".repeat(16));
    assert_eq!(parse(&input), Err(ParseError::TooLarge));
}
