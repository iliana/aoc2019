use std::io::prelude::*;

fn fuel_required(input: i64) -> i64 {
    (input / 3) - 2
}

fn fuel_required_incl_fuel(input: i64) -> i64 {
    let mut prev = fuel_required(input);
    let mut req = prev;
    loop {
        let new = fuel_required(prev);
        if new <= 0 {
            break req;
        }
        prev = new;
        req += new;
    }
}

fn main() {
    let modules = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(|v| v.ok().and_then(|v| v.parse().ok()))
        .collect::<Vec<_>>();
    println!(
        "part 1: {}",
        modules.iter().cloned().map(fuel_required).sum::<i64>()
    );
    println!(
        "part 2: {}",
        modules
            .iter()
            .cloned()
            .map(fuel_required_incl_fuel)
            .sum::<i64>()
    );
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(fuel_required(12), 2);
    assert_eq!(fuel_required(14), 2);
    assert_eq!(fuel_required(1969), 654);
    assert_eq!(fuel_required(100756), 33583);

    assert_eq!(fuel_required_incl_fuel(14), 2);
    assert_eq!(fuel_required_incl_fuel(1969), 966);
    assert_eq!(fuel_required_incl_fuel(100756), 50346);
}
