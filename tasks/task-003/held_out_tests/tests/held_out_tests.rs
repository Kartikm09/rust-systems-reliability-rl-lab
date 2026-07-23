use candidate_task::{DomainError, exit_code, process};
#[test]
fn success_and_exit_codes_are_compatible() {
    assert_eq!(process("42"), Ok(42));
    assert_eq!(exit_code(&DomainError::InvalidInteger), 65);
}
#[test]
fn source_has_no_string_error_result() {
    let source = std::fs::read_to_string("src/lib.rs").unwrap();
    assert!(!source.contains("Result<u64, String>"));
}
