use candidate_task::{ParseError, parse};
#[test]
fn malformed_and_length_mismatched_inputs_do_not_panic() {
    for input in ["", "x|a", "2|a", "0|x"] { let _ = parse(input); }
    assert_eq!(parse("2|a"), Err(ParseError::LengthMismatch));
}
