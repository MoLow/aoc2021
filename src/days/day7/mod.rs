use cached::proc_macro::cached;

fn parse_input(input: &str) -> Vec<usize> {
    input.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect()
}

fn find_cheapest_position<F: Fn(usize) -> usize>(crabs: &Vec<usize>, calculate_cost: F) -> (usize, usize) {
    let max = *crabs.iter().max().unwrap();
    let min = *crabs.iter().min().unwrap();
    let range = min..max;

    let deltas = range
        .map(|x| crabs.iter().map(|y| calculate_cost((x as isize - *y as isize).abs() as usize)).sum())
        .collect::<Vec<usize>>();

    let minimum_cost = *deltas.iter().min().unwrap();
    let minimum_cost_index = deltas.iter().position(|&x| x == minimum_cost).unwrap() as usize;

    return (minimum_cost, minimum_cost_index);
}

#[cached]
fn calculate_sum(size: usize) -> usize {
    (1..size + 1).sum()
} 

static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (u64, u64) {
    let crabs = parse_input(INPUT);
    let (part1, _) = find_cheapest_position(&crabs, |x| x);
    let (part2, _) = find_cheapest_position(&crabs, calculate_sum);

    return (part1 as u64, part2 as u64);
}