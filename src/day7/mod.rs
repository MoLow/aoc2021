use cached::proc_macro::cached;

fn parse_input(input: &str) -> Vec<i32> {
    input.trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

fn find_cheapest_position<F: Fn(i32) -> i32>(crabs: &Vec<i32>, calculate_cost: F) -> (i32, i32) {
    let max = *crabs.iter().max().unwrap();
    let min = *crabs.iter().min().unwrap();
    let range = min..max;

    let deltas = range
        .map(|x| crabs.iter().map(|y| calculate_cost((x - y).abs())).sum())
        .collect::<Vec<i32>>();

    let minimum_cost = *deltas.iter().min().unwrap();
    let minimum_cost_index = deltas.iter().position(|&x| x == minimum_cost).unwrap() as i32;

    return (minimum_cost, minimum_cost_index);
}

#[cached]
fn calculate_sum(size: i32) -> i32 {
    (1..size + 1).sum::<i32>()
} 

pub fn run(input: String) {
    let crabs = parse_input(&input);
    let cheapest_position = find_cheapest_position(&crabs, |x| x);
    println!("Part1: {:?}", cheapest_position);
    let cheapest_position_2 = find_cheapest_position(&crabs, calculate_sum);
    println!("Part2: {:?}", cheapest_position_2);
}