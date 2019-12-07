use std::io::{stdin, BufRead, BufReader, Error, ErrorKind, Result};

fn invalid_data<E: std::error::Error + Send + Sync + 'static>(err: E) -> Error {
    Error::new(ErrorKind::InvalidData, err)
}

pub fn load_stdin() -> Result<Vec<i64>> {
    BufReader::new(stdin())
        .split(b',')
        .map(|b| {
            b.and_then(|b| String::from_utf8(b).map_err(invalid_data))
                .and_then(|s| s.trim().parse().map_err(invalid_data))
        })
        .collect()
}

pub fn intcode(program: impl AsRef<[i64]>) -> Runner {
    Runner {
        program: program.as_ref().to_vec(),
        ip: 0,
        input: Vec::new(),
    }
}

pub struct Runner {
    program: Vec<i64>,
    input: Vec<i64>,
    ip: usize,
}

impl Runner {
    pub fn input(&mut self, input: i64) {
        self.input.push(input);
    }

    pub fn run(&mut self) -> Vec<i64> {
        self.collect()
    }

    pub fn program(&self) -> &[i64] {
        &self.program
    }

    fn read(&self, n: usize) -> Vec<i64> {
        let mut params = self.program[self.ip] / 100;
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            let value = self.program[self.ip + 1 + i];
            v.push(match params % 10 {
                0 => {
                    // position
                    self.program[value as usize]
                }
                1 => {
                    // immediate
                    value
                }
                _ => unimplemented!(),
            });
            params /= 10;
        }
        v
    }

    fn addr(&mut self, offset: usize) -> &mut i64 {
        let x = self.program[self.ip + offset] as usize;
        &mut self.program[x]
    }
}

impl Iterator for Runner {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        loop {
            match self.program[self.ip] % 100 {
                1 => {
                    // add
                    let data = self.read(2);
                    *self.addr(3) = data[0] + data[1];
                    self.ip += 4;
                }
                2 => {
                    // multiply
                    let data = self.read(2);
                    *self.addr(3) = data[0] * data[1];
                    self.ip += 4;
                }
                3 => {
                    // write input
                    *self.addr(1) = self.input.remove(0);
                    self.ip += 2;
                }
                4 => {
                    // read output
                    let data = self.read(1);
                    self.ip += 2;
                    break Some(data[0]);
                }
                5 => {
                    // jump-if-true
                    let data = self.read(2);
                    if data[0] != 0 {
                        self.ip = data[1] as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    // jump-if-false
                    let data = self.read(2);
                    if data[0] == 0 {
                        self.ip = data[1] as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    // less than
                    let data = self.read(2);
                    *self.addr(3) = if data[0] < data[1] { 1 } else { 0 };
                    self.ip += 4;
                }
                8 => {
                    // equals
                    let data = self.read(2);
                    *self.addr(3) = if data[0] == data[1] { 1 } else { 0 };
                    self.ip += 4;
                }
                99 => {
                    // halt
                    break None;
                }
                _ => unimplemented!(),
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    macro_rules! intcode_eq {
        ($in:expr, $out:expr) => {{
            let mut runner = intcode($in.to_vec());
            runner.run();
            assert_eq!(runner.program(), $out.to_vec().as_slice());
        }};

        ($in:expr, $input:expr, $output:expr) => {{
            let mut runner = intcode($in.to_vec());
            for input in $input.to_vec() {
                runner.input(input);
            }
            assert_eq!(runner.run(), $output.to_vec());
        }};
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
