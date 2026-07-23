use candidate_task::{DomainError, exit_code, process};
#[test]
fn errors_are_structured_with_stable_text() {
    let empty = process("").unwrap_err(); assert_eq!(empty, DomainError::EmptyInput); assert_eq!(empty.to_string(), "input is empty"); assert_eq!(exit_code(&empty), 64);
    let invalid = process("x").unwrap_err(); assert_eq!(invalid, DomainError::InvalidInteger); assert_eq!(invalid.to_string(), "input is not an integer");
}
