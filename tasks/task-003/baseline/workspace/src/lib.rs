#![forbid(unsafe_code)]

pub fn process(value: &str) -> Result<u64, String> {
    if value.is_empty() { return Err("input is empty".to_owned()); }
    value.parse::<u64>().map_err(|_| "input is not an integer".to_owned())
}
pub fn exit_code(message: &str) -> i32 {
    if message == "input is empty" { 64 } else { 65 }
}
