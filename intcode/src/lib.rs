use std::io::{BufRead, Error, ErrorKind, Result};

fn invalid_data<E: std::error::Error + Send + Sync + 'static>(err: E) -> Error {
    Error::new(ErrorKind::InvalidData, err)
}

pub fn load<R: BufRead>(reader: &mut R) -> Result<Vec<usize>> {
    reader
        .split(b',')
        .map(|b| {
            b.and_then(|b| String::from_utf8(b).map_err(invalid_data))
                .and_then(|s| s.trim().parse().map_err(invalid_data))
        })
        .collect()
}

pub fn intcode(input: &mut [usize]) -> &mut [usize] {
    let mut ip = 0;
    loop {
        match input[ip] {
            1 => {
                // add
                let a = input[ip + 1];
                let b = input[ip + 2];
                let x = input[ip + 3];
                input[x] = input[a] + input[b];
                ip += 4;
            }
            2 => {
                // multiply
                let a = input[ip + 1];
                let b = input[ip + 2];
                let x = input[ip + 3];
                input[x] = input[a] * input[b];
                ip += 4;
            }
            99 => {
                // halt
                break input;
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(intcode(&mut [1, 0, 0, 0, 99]), &[2, 0, 0, 0, 99]);
    assert_eq!(intcode(&mut [2, 3, 0, 3, 99]), &[2, 3, 0, 6, 99]);
    assert_eq!(intcode(&mut [2, 4, 4, 5, 99, 0]), &[2, 4, 4, 5, 99, 9801]);
    assert_eq!(
        intcode(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99]),
        &[30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
}
