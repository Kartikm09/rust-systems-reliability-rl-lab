use candidate_task::process;
#[test]
fn integer_boundaries_remain_standard() {
    assert_eq!(process("0"), Ok(0)); assert!(process("18446744073709551616").is_err());
}
