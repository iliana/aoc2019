use std::io::prelude::*;

fn intcode(input: Vec<usize>) -> Vec<usize> {
    let mut input = input;
    let mut i = 0;
    loop {
        match input[i] {
            1 => {
                let a = input[i + 1];
                let b = input[i + 2];
                let x = input[i + 3];
                input[x] = input[a] + input[b];
                i += 4;
            }
            2 => {
                let a = input[i + 1];
                let b = input[i + 2];
                let x = input[i + 3];
                input[x] = input[a] * input[b];
                i += 4;
            }
            99 => return input,
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut values_str = String::new();
    std::io::stdin().read_to_string(&mut values_str)?;
    let values = values_str
        .trim()
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, _>>()?;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut values = values.clone();
            values[1] = noun;
            values[2] = verb;
            let values = intcode(values);
            if values[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(intcode(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    assert_eq!(intcode(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    assert_eq!(intcode(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    assert_eq!(
        intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
}
