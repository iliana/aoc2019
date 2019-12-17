use intcode::Runner;
use std::fmt;
use std::ops::Index;
use std::task::Poll;

#[derive(Debug)]
struct Map(String);

impl Map {
    fn read(runner: &mut Runner) -> Map {
        let mut s = String::new();
        while let Some(Poll::Ready(n)) = runner.next() {
            s.push(char::from(n as u8));
        }
        Map(s)
    }

    fn width(&self) -> usize {
        self.0.lines().next().unwrap().len()
    }

    fn height(&self) -> usize {
        self.0.trim().lines().count()
    }

    fn coord_to_idx(&self, coord: (usize, usize)) -> usize {
        let w = self.width() + 1;
        coord.1 * w + coord.0
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.trim())
    }
}

impl Index<(usize, usize)> for Map {
    type Output = str;

    fn index(&self, coord: (usize, usize)) -> &str {
        let idx = self.coord_to_idx(coord);
        &self.0[idx..=idx]
    }
}

fn main() {
    let input = util::read_intcode();

    let mut program = [0; 8192];
    program[..input.len()].copy_from_slice(&input);
    let mut runner = Runner::new(&mut program);
    let map = Map::read(&mut runner);
    println!("{}", map);
    let mut sum = 0;
    for x in 1..(map.width() - 1) {
        for y in 1..(map.height() - 1) {
            if &map[(x, y)] == "#"
                && &map[(x - 1, y)] == "#"
                && &map[(x + 1, y)] == "#"
                && &map[(x, y - 1)] == "#"
                && &map[(x, y + 1)] == "#"
            {
                sum += x * y;
            }
        }
    }
    println!("part 1: {}", sum);

    // it'd be neat to figure this out programatically but my brain works just as well!
    let a = "L,8,R,10,L,10";
    let b = "R,10,L,8,L,8,L,10";
    let c = "L,4,L,6,L,8,L,8";
    let main = "A,B,A,C,B,C,A,C,B,C";

    let mut program = [0; 8192];
    program[..input.len()].copy_from_slice(&input);
    program[0] = 2;
    println!(
        "part 2: {}",
        Runner::new(&mut program)
            .full_input(
                format!("{}\n{}\n{}\n{}\nn\n", main, a, b, c)
                    .into_bytes()
                    .into_iter(),
            )
            .last()
            .unwrap()
    );
}
