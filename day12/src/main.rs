use itertools::Itertools;
use num_integer::Integer;

type System = [[i64; 3]; 4];

fn get_coord(system: System, coord: usize) -> [i64; 4] {
    [
        system[0][coord],
        system[1][coord],
        system[2][coord],
        system[3][coord],
    ]
}

fn step_coord(pos: &mut System, vel: &mut System, coord: usize) {
    for (a, b) in (0..4).tuple_combinations() {
        if pos[a][coord] < pos[b][coord] {
            vel[a][coord] += 1;
            vel[b][coord] -= 1;
        } else if pos[a][coord] > pos[b][coord] {
            vel[a][coord] -= 1;
            vel[b][coord] += 1;
        }
    }
    for i in 0..4 {
        pos[i][coord] += vel[i][coord];
    }
}

fn step(pos: &mut System, vel: &mut System) {
    for coord in 0..3 {
        step_coord(pos, vel, coord);
    }
}

fn energy(pos: System, vel: System) -> u64 {
    (0..4)
        .map(|i| {
            pos[i].iter().map(|n| n.abs() as u64).sum::<u64>()
                * vel[i].iter().map(|n| n.abs() as u64).sum::<u64>()
        })
        .sum()
}

fn cycle(pos_init: System, vel_init: System, coord: usize) -> usize {
    let mut pos = pos_init;
    let mut vel = vel_init;
    (1..)
        .find(|_| {
            step_coord(&mut pos, &mut vel, coord);
            get_coord(pos, coord) == get_coord(pos_init, coord)
                && get_coord(vel, coord) == get_coord(vel_init, coord)
        })
        .unwrap()
}

fn main() {
    let input = [[-6, -5, -8], [0, -3, -13], [-15, 10, -11], [-3, -8, 3]];

    let mut pos = input;
    let mut vel = System::default();
    for _ in 0..1000 {
        step(&mut pos, &mut vel);
    }
    println!("part 1: {}", energy(pos, vel));

    let x = cycle(input, System::default(), 0);
    let y = cycle(input, System::default(), 1);
    let z = cycle(input, System::default(), 2);
    println!("part 2: {}", x.lcm(&y).lcm(&z));
}

#[cfg(test)]
#[test]
fn test() {
    let input = [[-1, 0, 2], [2, -10, -7], [4, -8, 8], [3, 5, -1]];

    let mut pos = input;
    let mut vel = System::default();
    step(&mut pos, &mut vel);
    assert_eq!(pos, [[2, -1, 1], [3, -7, -4], [1, -7, 5], [2, 2, 0]]);
    step(&mut pos, &mut vel);
    assert_eq!(pos, [[5, -3, -1], [1, -2, 2], [1, -4, -1], [1, -4, 2]]);

    let pos = [[2, 1, -3], [1, -8, 0], [3, -6, 1], [2, 0, 4]];
    let vel = [[-3, -2, 1], [-1, 1, 3], [3, 2, -3], [1, -1, -1]];
    assert_eq!(energy(pos, vel), 179);
}
