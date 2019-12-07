#![no_std]

use core::task::{Poll, Poll::*};

pub trait PollExt<T> {
    fn unwrap(self) -> T;
}

impl<T> PollExt<T> for Poll<T> {
    fn unwrap(self) -> T {
        match self {
            Ready(v) => v,
            Pending => panic!("program blocked on input"),
        }
    }
}

pub struct Runner<'a> {
    program: &'a mut [i64],
    input: Option<i64>,
    data: [i64; 2],
    ip: usize,
}

impl Runner<'_> {
    pub fn new(program: &mut [i64]) -> Runner<'_> {
        Runner {
            program,
            input: None,
            data: [0; 2],
            ip: 0,
        }
    }

    pub fn input(&mut self, input: i64) {
        self.input = Some(input);
    }

    pub fn run(&mut self) {
        self.for_each(|x| {
            x.unwrap();
        })
    }

    fn pop(&mut self) -> i64 {
        let x = self.program[self.ip];
        self.ip += 1;
        x
    }

    fn read(&mut self, opcode: i64, n: usize) {
        let mut params = opcode / 100;
        for i in 0..n {
            let value = self.pop();
            self.data[i] = match params % 10 {
                0 => {
                    // position
                    self.program[value as usize]
                }
                1 => {
                    // immediate
                    value
                }
                _ => unimplemented!(),
            };
            params /= 10;
        }
    }

    fn addr(&mut self) -> &mut i64 {
        let x = self.pop() as usize;
        &mut self.program[x]
    }
}

impl Iterator for Runner<'_> {
    type Item = Poll<i64>;

    fn next(&mut self) -> Option<Poll<i64>> {
        loop {
            let opcode = self.pop();
            match opcode % 100 {
                1 => {
                    // add
                    self.read(opcode, 2);
                    *self.addr() = self.data[0] + self.data[1];
                }
                2 => {
                    // multiply
                    self.read(opcode, 2);
                    *self.addr() = self.data[0] * self.data[1];
                }
                3 => {
                    // write input
                    if let Some(input) = core::mem::replace(&mut self.input, None) {
                        *self.addr() = input;
                    } else {
                        self.ip -= 1;
                        break Some(Pending);
                    }
                }
                4 => {
                    // read output
                    self.read(opcode, 1);
                    break Some(Ready(self.data[0]));
                }
                5 => {
                    // jump-if-true
                    self.read(opcode, 2);
                    if self.data[0] != 0 {
                        self.ip = self.data[1] as usize;
                    }
                }
                6 => {
                    // jump-if-false
                    self.read(opcode, 2);
                    if self.data[0] == 0 {
                        self.ip = self.data[1] as usize;
                    }
                }
                7 => {
                    // less than
                    self.read(opcode, 2);
                    *self.addr() = if self.data[0] < self.data[1] { 1 } else { 0 };
                }
                8 => {
                    // equals
                    self.read(opcode, 2);
                    *self.addr() = if self.data[0] == self.data[1] { 1 } else { 0 };
                }
                99 => {
                    // halt
                    break None;
                }
                _ => unimplemented!(
                    "opcode {} (ip={} program={:?})",
                    opcode,
                    self.ip,
                    self.program
                ),
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    macro_rules! intcode_eq {
        ($in:expr, $out:expr) => {{
            let mut program = $in;
            let mut runner = Runner::new(&mut program[..]);
            assert!(runner.next().is_none());
            assert_eq!(&program[..], &$out[..]);
        }};

        ($in:expr, $input:expr, $output:expr) => {{
            let mut program = $in;
            let mut runner = Runner::new(&mut program[..]);
            let input_orig = $input;
            let mut input = &input_orig[..];
            let output = $output;
            let mut i = 0;
            loop {
                match runner.next() {
                    Some(Ready(v)) => {
                        assert_eq!(v, output[i]);
                        i += 1;
                    }
                    Some(Pending) => {
                        runner.input(input[0]);
                        input = &input[1..];
                    }
                    None => {
                        break;
                    }
                }
            }
            assert_eq!(i, output.len());
        }};
    }

    // day 2
    intcode_eq!([1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
    intcode_eq!([2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
    intcode_eq!([2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
    intcode_eq!(
        [1, 1, 1, 4, 99, 5, 6, 0, 99],
        &[30, 1, 1, 4, 2, 5, 6, 0, 99]
    );

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
