#![no_std]
#![warn(clippy::pedantic)]
#![allow(clippy::use_self)]

use core::convert::TryFrom;
use core::fmt::{self, Debug};
use core::task::Poll;

pub trait PollExt<T> {
    fn unwrap(self) -> T;
}

impl<T> PollExt<T> for Poll<T> {
    fn unwrap(self) -> T {
        match self {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("called `Poll::unwrap()` on a `Pending` value"),
        }
    }
}

pub struct Runner<'a> {
    program: &'a mut [i64],
    ip: usize,
    base: i64,
    halted: bool,
    input: Option<i64>,
    register: [i64; 2],
}

impl<'a> Runner<'a> {
    pub fn new(program: &'a mut [i64]) -> Runner<'a> {
        Runner {
            program,
            ip: 0,
            base: 0,
            halted: false,
            input: None,
            register: [0; 2],
        }
    }

    pub fn input(&mut self, input: i64) {
        self.input = Some(input);
    }

    pub fn full_input<I, T>(self, input: I) -> FullRunner<'a, I::IntoIter>
    where
        I: IntoIterator<Item = T>,
        T: Into<i64>,
    {
        FullRunner {
            runner: self,
            iter: input.into_iter(),
        }
    }

    fn panic(&self, msg: &str, ip_offset: usize) -> ! {
        panic!(
            "{} (ip={} mem={})",
            msg,
            self.ip - ip_offset,
            self.program[self.ip - ip_offset]
        )
    }

    fn usize(&self, value: i64, msg: &str, ip_offset: usize) -> usize {
        if let Ok(value) = usize::try_from(value) {
            value
        } else {
            panic!(
                "{} {} (ip={} mem={} base={})",
                msg,
                value,
                self.ip - ip_offset,
                self.program[self.ip - ip_offset],
                self.base
            );
        }
    }

    pub fn run(&mut self) {
        if !loop {
            match self.next() {
                Some(Poll::Ready(_)) => {}
                Some(Poll::Pending) => break false,
                None => break true,
            }
        } {
            self.panic("program blocked on input", 0)
        }
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
            self.register[i] = match params % 10 {
                0 => {
                    // position
                    self.program[self.usize(value, "illegal position value", 1)]
                }
                1 => {
                    // immediate
                    value
                }
                2 => {
                    // relative
                    self.program[self.usize(self.base + value, "illegal relative value", 1)]
                }
                _ => self.panic("illegal value parameter mode", 1),
            };
            params /= 10;
        }
    }

    fn addr(&mut self, mode: i64) -> &mut i64 {
        let x = match mode % 10 {
            0 => self.pop(),
            2 => self.pop() + self.base,
            _ => self.panic("illegal address parameter mode", 0),
        };
        &mut self.program[self.usize(x, "illegal address", 1)]
    }
}

impl Iterator for Runner<'_> {
    type Item = Poll<i64>;

    fn next(&mut self) -> Option<Poll<i64>> {
        if self.halted {
            return None;
        }

        loop {
            let opcode = self.pop();
            match opcode % 100 {
                1 => {
                    // add
                    self.read(opcode, 2);
                    *self.addr(opcode / 10000) = self.register[0] + self.register[1];
                }
                2 => {
                    // multiply
                    self.read(opcode, 2);
                    *self.addr(opcode / 10000) = self.register[0] * self.register[1];
                }
                3 => {
                    // write input
                    if let Some(input) = core::mem::replace(&mut self.input, None) {
                        *self.addr(opcode / 100) = input;
                    } else {
                        self.ip -= 1;
                        break Some(Poll::Pending);
                    }
                }
                4 => {
                    // read output
                    self.read(opcode, 1);
                    break Some(Poll::Ready(self.register[0]));
                }
                5 => {
                    // jump-if-true
                    self.read(opcode, 2);
                    if self.register[0] != 0 {
                        self.ip = self.usize(self.register[1], "illegal address", 1);
                    }
                }
                6 => {
                    // jump-if-false
                    self.read(opcode, 2);
                    if self.register[0] == 0 {
                        self.ip = self.usize(self.register[1], "illegal address", 1);
                    }
                }
                7 => {
                    // less than
                    self.read(opcode, 2);
                    *self.addr(opcode / 10000) = if self.register[0] < self.register[1] {
                        1
                    } else {
                        0
                    };
                }
                8 => {
                    // equals
                    self.read(opcode, 2);
                    *self.addr(opcode / 10000) = if self.register[0] == self.register[1] {
                        1
                    } else {
                        0
                    };
                }
                9 => {
                    // adjust relative base
                    self.read(opcode, 1);
                    self.base += self.register[0];
                }
                99 => {
                    // halt
                    self.halted = true;
                    break None;
                }
                _ => self.panic("illegal instruction", 1),
            }
        }
    }
}

impl Debug for Runner<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Runner")
            .field("ip", &self.ip)
            .field("base", &self.base)
            .field("halted", &self.halted)
            .field("input", &self.input)
            .field("register", &self.register)
            .finish()
    }
}

#[derive(Debug)]
pub struct FullRunner<'a, I> {
    runner: Runner<'a>,
    iter: I,
}

impl<'a, I, T> FullRunner<'a, I>
where
    I: Iterator<Item = T>,
    T: Into<i64>,
{
    pub fn run(&mut self) {
        self.last();
    }
}

impl<'a, I, T> Iterator for FullRunner<'_, I>
where
    I: Iterator<Item = T>,
    T: Into<i64>,
{
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        loop {
            match self.runner.next() {
                Some(Poll::Ready(v)) => break Some(v),
                Some(Poll::Pending) => {
                    if let Some(input) = self.iter.next() {
                        self.runner.input(input.into());
                    } else {
                        self.runner.panic("program blocked on input", 0);
                    }
                }
                None => break None,
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
            let input: &[i64] = &$input[..];
            let runner = Runner::new(&mut program[..]).full_input(input.iter().cloned());
            let output: &[i64] = &$output[..];
            assert_eq!(runner.eq(output.iter().cloned()), true);
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

    // day 9
    let mut a = [0; 102];
    &a[..16].copy_from_slice(&[
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ]);
    intcode_eq!(
        a,
        [],
        [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    );

    intcode_eq!(
        [1102, 34915192, 34915192, 7, 4, 7, 99, 0],
        [],
        [1219070632396864]
    );
    intcode_eq!([104, 1125899906842624, 99], [], [1125899906842624]);
}
