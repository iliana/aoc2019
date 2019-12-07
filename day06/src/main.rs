use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn build_tree(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in s.lines() {
        let mut iter = line.trim().split(')');
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        map.insert(b.into(), a.into());
    }
    map
}

fn orbit_count(map: &HashMap<String, String>) -> usize {
    fn count(
        key: &str,
        counts: &mut HashMap<String, usize>,
        map: &HashMap<String, String>,
    ) -> usize {
        if let Some(count) = counts.get(key) {
            *count
        } else if let Some(parent) = map.get(key) {
            let count = 1 + count(parent, counts, map);
            counts.insert(key.to_string(), count);
            count
        } else {
            0
        }
    }

    let mut counts = HashMap::new();
    map.keys().map(|k| count(k, &mut counts, map)).sum()
}

fn path_to_top<'a>(key: &'a str, map: &'a HashMap<String, String>) -> Vec<&'a str> {
    let mut v = Vec::new();
    let mut key = key;
    loop {
        if let Some(parent) = map.get(key) {
            v.push(parent.as_str());
            key = parent;
        } else {
            break v;
        }
    }
}

fn hs<T: Eq + Hash + Clone>(v: &[T]) -> HashSet<T> {
    v.iter().cloned().collect()
}

fn transfer_count<'a>(a: &str, b: &str, map: &'a HashMap<String, String>) -> usize {
    let a = path_to_top(a, map);
    let b = path_to_top(b, map);
    let intersection = hs(&a)
        .intersection(&hs(&b))
        .cloned()
        .collect::<HashSet<_>>();
    let a_len = a
        .into_iter()
        .position(|k| intersection.contains(k))
        .unwrap();
    let b_len = b
        .into_iter()
        .position(|k| intersection.contains(k))
        .unwrap();
    a_len + b_len
}

#[test]
fn test_part1() {
    let data = &"COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L"[..];
    let tree = build_tree(data);
    assert_eq!(tree.len(), 11);
    assert_eq!(tree.get("C"), Some(&"B".to_string()));
    assert!(tree.get("COM").is_none());
    assert_eq!(orbit_count(&tree), 42);
}

#[test]
fn test_part2() {
    let data = &"COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN"[..];
    let tree = build_tree(data);
    assert_eq!(transfer_count("YOU", "SAN", &tree), 4);
}

fn main() {
    let tree = build_tree(&std::fs::read_to_string("input.txt").unwrap());
    println!("part 1: {}", orbit_count(&tree));
    println!("part 2: {}", transfer_count("YOU", "SAN", &tree));
}
