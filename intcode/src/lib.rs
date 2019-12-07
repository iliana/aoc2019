use std::io::{BufRead, Error, ErrorKind, Result};

fn invalid_data<E: std::error::Error + Send + Sync + 'static>(err: E) -> Error {
    Error::new(ErrorKind::InvalidData, err)
}

pub fn load<R: BufRead>(reader: &mut R) -> Result<Vec<i64>> {
    reader
        .split(b',')
        .map(|b| {
            b.and_then(|b| String::from_utf8(b).map_err(invalid_data))
                .and_then(|s| s.trim().parse().map_err(invalid_data))
        })
        .collect()
}

fn read(ip: usize, n: usize, program: &[i64]) -> Vec<i64> {
    let mut params = program[ip] / 100;
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(match params % 10 {
            0 => {
                // position
                let x = program[ip + 1 + i];
                program[x as usize]
            }
            1 => {
                // immediate
                program[ip + 1 + i]
            }
            _ => unimplemented!(),
        });
        params /= 10;
    }
    v
}

pub fn intcode(program: &mut [i64], input: impl IntoIterator<Item = i64>) -> Vec<i64> {
    let mut ip = 0;
    let mut input = input.into_iter();
    let mut output = Vec::new();
    loop {
        match program[ip] % 100 {
            1 => {
                // add
                let data = read(ip, 2, &program);
                let x = program[ip + 3] as usize;
                program[x] = data[0] + data[1];
                ip += 4;
            }
            2 => {
                // multiply
                let data = read(ip, 2, &program);
                let x = program[ip + 3] as usize;
                program[x] = data[0] * data[1];
                ip += 4;
            }
            3 => {
                // write input
                let x = program[ip + 1] as usize;
                program[x] = input.next().unwrap();
                ip += 2;
            }
            4 => {
                // read output
                let data = read(ip, 1, &program);
                output.push(data[0]);
                ip += 2;
            }
            5 => {
                // jump-if-true
                let data = read(ip, 2, &program);
                if data[0] != 0 {
                    ip = data[1] as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                // jump-if-false
                let data = read(ip, 2, &program);
                if data[0] == 0 {
                    ip = data[1] as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                // less than
                let data = read(ip, 2, &program);
                let x = program[ip + 3] as usize;
                program[x] = if data[0] < data[1] { 1 } else { 0 };
                ip += 4;
            }
            8 => {
                // equals
                let data = read(ip, 2, &program);
                let x = program[ip + 3] as usize;
                program[x] = if data[0] == data[1] { 1 } else { 0 };
                ip += 4;
            }
            99 => {
                // halt
                break output;
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    macro_rules! intcode_eq {
        ($in:expr, $input:expr, $out:expr, $output:expr) => {{
            let mut program = $in.to_vec();
            let output = intcode(&mut program, $input.to_vec());
            assert_eq!(program, $out.to_vec());
            assert_eq!(output, $output.to_vec());
        }};

        ($in:expr, $out:expr) => {
            intcode_eq!($in, [], $out, [])
        };

        ($in:expr, $input:expr, $output:expr) => {
            let mut program = $in.to_vec();
            let output = intcode(&mut program, $input.to_vec());
            assert_eq!(output, $output.to_vec());
        };
    }

    // day 2
    intcode_eq!([1, 0, 0, 0, 99], [2, 0, 0, 0, 99]);
    intcode_eq!([2, 3, 0, 3, 99], [2, 3, 0, 6, 99]);
    intcode_eq!([2, 4, 4, 5, 99, 0], [2, 4, 4, 5, 99, 9801]);
    intcode_eq!([1, 1, 1, 4, 99, 5, 6, 0, 99], [30, 1, 1, 4, 2, 5, 6, 0, 99]);

    // day 5
    intcode_eq!([3, 0, 4, 0, 99], [42], [42]);
    intcode_eq!([1002, 4, 3, 4, 33], [1002, 4, 3, 4, 99]);

    intcode_eq!([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], [8], [1]);
    intcode_eq!([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], [7], [0]);
    intcode_eq!([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], [7], [1]);
    intcode_eq!([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], [8], [0]);
    intcode_eq!([3, 3, 1108, -1, 8, 3, 4, 3, 99], [8], [1]);
    intcode_eq!([3, 3, 1108, -1, 8, 3, 4, 3, 99], [7], [0]);
    intcode_eq!([3, 3, 1107, -1, 8, 3, 4, 3, 99], [7], [1]);
    intcode_eq!([3, 3, 1107, -1, 8, 3, 4, 3, 99], [8], [0]);

    intcode_eq!(
        [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        [0],
        [0]
    );
    intcode_eq!(
        [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        [4],
        [1]
    );
    intcode_eq!([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], [0], [0]);
    intcode_eq!([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], [4], [1]);

    intcode_eq!(
        [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99
        ],
        [7],
        [999]
    );
    intcode_eq!(
        [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99
        ],
        [8],
        [1000]
    );
    intcode_eq!(
        [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99
        ],
        [9],
        [1001]
    );
}
