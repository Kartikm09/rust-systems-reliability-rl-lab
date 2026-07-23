use candidate_task::{ParseError, parse, MAX_PAYLOAD};

#[test]
fn boundary_and_escape_cases_are_deterministic() {
    let exact = format!("{}|{}", MAX_PAYLOAD, "x".repeat(MAX_PAYLOAD));
    assert_eq!(parse(&exact).unwrap().len(), MAX_PAYLOAD);
    assert_eq!(parse(r"1|\\"), Ok("\\".to_owned()));
    assert_eq!(parse(r"1|\"), Err(ParseError::DanglingEscape));
}
