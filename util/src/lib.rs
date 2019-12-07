use std::env;
use std::path::PathBuf;

pub fn read_input() -> String {
    std::fs::read_to_string(
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("input.txt"),
    )
    .unwrap()
}

pub fn read_intcode() -> Vec<i64> {
    read_input()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}
