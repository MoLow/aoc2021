pub fn count_increasing(input: &str, window_size: usize) -> usize {
    return input
        .split_whitespace()
        .map(|x| x.parse::<i32>()
        .expect("Not a number"))
        .collect::<Vec<i32>>()
        .windows(window_size)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count()
        .try_into()
        .unwrap()
}

static INPUT: &str = include_str!("./input.txt");

pub fn run() -> (u64, u64) {
    let part1 = count_increasing(INPUT, 1) as u64;
    let part2 = count_increasing(INPUT, 3) as u64;
    
    return (part1, part2);
}