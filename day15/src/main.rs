use intcode::{PollExt, Runner};
use rand::Rng;
use std::collections::HashMap;

#[allow(unused)]
fn flail(program: &mut [i64]) -> HashMap<(i64, i64), i64> {
    let mut runner = Runner::new(program);
    let mut map = HashMap::new();
    let mut pos = (0, 0);
    map.insert(pos, 1);

    let mut rng = rand::thread_rng();
    loop {
        let dir = rng.gen_range(1, 5);
        runner.input(dir);
        let new_pos = match dir {
            1 => (pos.0 - 1, pos.1),
            2 => (pos.0 + 1, pos.1),
            3 => (pos.0, pos.1 - 1),
            4 => (pos.0, pos.1 + 1),
            _ => unreachable!(),
        };
        let output = runner.next().unwrap().unwrap();
        map.insert(new_pos, output);
        match output {
            1 => {
                pos = new_pos;
            }
            2 => {
                break;
            }
            _ => {}
        }
    }

    map
}

#[allow(unused)]
fn print_map(map: &HashMap<(i64, i64), i64>) {
    let x_min = *map.keys().map(|(x, _)| x).min().unwrap();
    let x_max = *map.keys().map(|(x, _)| x).max().unwrap();
    let y_min = *map.keys().map(|(_, y)| y).min().unwrap();
    let y_max = *map.keys().map(|(_, y)| y).max().unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if (x, y) == (0, 0) {
                print!("x");
            } else {
                print!(
                    "{}",
                    match map.get(&(x, y)) {
                        Some(0) => '#',
                        Some(1) => ' ',
                        Some(2) => 'D',
                        Some(3) => '*',
                        None => '.',
                        _ => unreachable!(),
                    }
                );
            }
        }
        println!();
    }
}

fn parse_map(s: &str) -> [[u8; 41]; 41] {
    let mut map = [[0; 41]; 41];
    for y in 0..41 {
        for x in 0..41 {
            map[x][y] = match &s[(y * 42 + x)..=(y * 42 + x)] {
                "#" => 0,
                " " => 1,
                "O" => 2,
                _ => unreachable!(),
            };
        }
    }
    map
}

fn disperse(mut map: [[u8; 41]; 41]) -> usize {
    let mut i = 0;
    loop {
        //print_final_map(&map);
        if coord_iter().all(|(x, y)| map[x][y] == 0 || map[x][y] == 2) {
            break;
        }

        i += 1;

        let o = coord_iter()
            .filter(|(x, y)| map[*x][*y] == 2)
            .collect::<Vec<_>>();
        for coord in o {
            for (adj_x, adj_y) in adjacent(coord) {
                let v = &mut map[adj_x][adj_y];
                if *v == 1 {
                    *v = 2;
                }
            }
        }
    }
    i
}

#[allow(unused)]
fn print_final_map(map: &[[u8; 41]; 41]) {
    for y in 0..41 {
        for x in 0..41 {
            print!(
                "{}",
                match map[x][y] {
                    0 => '#',
                    1 => ' ',
                    2 => 'O',
                    _ => unreachable!(),
                }
            );
        }
        println!();
    }
}

fn coord_iter() -> impl Iterator<Item = (usize, usize)> {
    (0..41 * 41).map(|c| (c % 41, c / 41))
}

fn adjacent(coord: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = coord;
    let mut v = Vec::new();
    if x > 0 {
        v.push((x - 1, y));
    }
    if x < 40 {
        v.push((x + 1, y));
    }
    if y > 0 {
        v.push((x, y - 1));
    }
    if y < 40 {
        v.push((x, y + 1));
    }
    v
}

fn main() {
    let input = util::read_intcode();

    let mut program = [0; 2048];
    program[..input.len()].copy_from_slice(&input);
    // So turns out I didn't actually want to do this problem so I flailed and printed the map and
    // did the maze by hand
    //print_map(&flail(&mut program));

    // Now I'm just going to re-parse the map with all the holes filled in by hand and do the awful
    // dispersion logic
    println!(
        "part 2: {}",
        disperse(parse_map(&std::fs::read_to_string("map.txt").unwrap()))
    );
}
