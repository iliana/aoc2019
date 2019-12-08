fn part1(data: &[u8]) -> usize {
    let mut counts = Vec::new();
    for i in 0..(data.len() / 150) {
        let subdata = &data[(i * 150)..((i + 1) * 150)];
        let mut count = [0, 0, 0];
        for byte in subdata {
            count[(*byte as usize) - 48] += 1;
        }
        counts.push(count);
    }
    let min = counts.iter().min_by_key(|x| x[0]).unwrap();
    min[1] * min[2]
}

fn part2(data: &[u8], width: usize, height: usize) {
    let layer_size = width * height;
    let mut layers = Vec::new();
    for i in 0..(data.len() / layer_size) {
        layers.push(&data[(i * layer_size)..((i + 1) * layer_size)]);
    }

    let mut image = vec![2; layer_size];

    for i in 0..layer_size {
        for layer in &layers {
            match layer[i] {
                b'0' => {
                    image[i] = 0;
                    break;
                }
                b'1' => {
                    image[i] = 1;
                    break;
                }
                _ => continue,
            }
        }
    }

    println!("{:?}", image);

    for x in 0..height {
        for y in 0..width {
            print!(
                "{}",
                match image[(x * width) + y] {
                    0 => " ",
                    _ => "x",
                }
            );
        }
        println!();
    }
}

fn main() {
    let data = util::read_input();
    let data = data.trim().as_bytes();
    println!("part 1: {}", part1(&data));
    part2(&data, 25, 6);
}
