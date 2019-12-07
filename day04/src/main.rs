#![allow(clippy::trivially_copy_pass_by_ref)]

fn valid_password_part1(s: &u32) -> bool {
    let s = s.to_string().bytes().collect::<Vec<_>>();
    s.len() == 6 && s.windows(2).any(|w| w[0] == w[1]) && s.windows(2).all(|w| w[0] <= w[1])
}

fn valid_password_part2(s: &u32) -> bool {
    let mut last = b'A';
    let mut count = 0;
    for b in s.to_string().bytes() {
        if b == last {
            count += 1;
        } else {
            if count == 2 {
                return true;
            }
            last = b;
            count = 1;
        }
    }
    count == 2
}

#[test]
fn test_valid_password() {
    assert_eq!(valid_password_part1(&111111), true);
    assert_eq!(valid_password_part1(&223450), false);
    assert_eq!(valid_password_part1(&123789), false);

    assert_eq!(valid_password_part2(&112233), true);
    assert_eq!(valid_password_part2(&123444), false);
    assert_eq!(valid_password_part2(&111122), true);
}

fn main() {
    println!(
        "part 1: {}",
        (248_345..=746_315).filter(valid_password_part1).count()
    );
    println!(
        "part 2: {}",
        (248_345..=746_315).filter(|s| valid_password_part1(s) && valid_password_part2(s)).count()
    );
}
