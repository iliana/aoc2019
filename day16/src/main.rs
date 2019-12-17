use std::str::FromStr;

fn fft_partial(input: &[u32], output: &mut [u32]) {
    output[input.len() - 1] = input[input.len() - 1];
    for i in (0..(input.len() - 1)).rev() {
        output[i] = input[i] + output[i + 1];
    }
    for i in 0..input.len() {
        output[i] %= 10;
    }
}

fn fft_inner(input: &[u32], output: &mut [u32]) {
    let start = std::time::Instant::now();
    for i in 0..input.len() {
        if i > 0 && i % 1000 == 0 {
            println!(
                "{}/{} @ {}ms",
                i,
                input.len(),
                (std::time::Instant::now() - start).as_millis()
            );
        }
        let mut sums = [0u32; 2];
        for (j, chunk) in input[i..].chunks(i + 1).step_by(2).enumerate() {
            sums[j % 2] += chunk.iter().sum::<u32>();
        }
        let sum = std::cmp::max(sums[0], sums[1]) - std::cmp::min(sums[0], sums[1]);
        output[i] = sum % 10;
    }
}

fn fft(input: &str, phases: usize, offset: usize) -> String {
    let mut input = input.bytes().map(|c| (c & 0xf) as u32).collect::<Vec<_>>();
    if offset >= input.len() / 2 {
        let mut output = vec![0; input.len() - offset];
        for _ in 0..phases {
            fft_partial(&input[offset..], &mut output);
            input[offset..].copy_from_slice(&output);
        }
    } else {
        let mut output = vec![0; input.len()];
        for _ in 0..phases {
            fft_inner(&input, &mut output);
            input.copy_from_slice(&output);
        }
    }
    input[offset..offset + 8]
        .iter()
        .map(|v| char::from(*v as u8 + b'0'))
        .collect()
}

fn repeat(s: &str, n: usize) -> String {
    let mut out = String::with_capacity(s.len() * n);
    for _ in 0..10000 {
        out.push_str(s);
    }
    out
}

#[cfg(test)]
#[test]
fn test_ffi() {
    assert_eq!(fft("12345678", 1, 0), "48226158");
    assert_eq!(fft("12345678", 2, 0), "34040438");
    assert_eq!(fft("12345678", 3, 0), "03415518");
    assert_eq!(fft("12345678", 4, 0), "01029498");

    assert_eq!(fft("80871224585914546619083218645595", 100, 0), "24176176",);

    assert_eq!(
        fft(
            &repeat("03036732577212944063491565474664", 10000),
            100,
            303673
        ),
        "84462026"
    );
}

fn main() {
    let input = util::read_input();
    let input = input.trim();
    println!("part 1: {}", fft(input, 100, 0));

    let offset = usize::from_str(&input[..7]).unwrap();
    println!("part 2: {}", fft(&repeat(input, 10000), 100, offset));
}
