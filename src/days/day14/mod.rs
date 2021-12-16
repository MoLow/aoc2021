use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut split = input.split("\n\n");
    let polymer = split.next().unwrap().chars().collect::<Vec<char>>();
    let rules = split.next().unwrap().lines().map(|line| {
        let mut iter = line.split(" -> ");
        let pair = iter.next().unwrap().chars().collect::<Vec<_>>();
        let insertion = iter.next().unwrap().chars().next().unwrap();
        ((pair[0], pair[1]), insertion)
    }).collect();

    return (polymer, rules);
}

fn run_insertion_rules(polymer: &Vec<char>, rules: &HashMap<(char, char), char>, times: i32) -> u64 {
    let mut chars_count = polymer
        .iter()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            return acc;
        });
    
    let mut pairs_count = polymer
        .windows(2)
        .fold(HashMap::new(), |mut acc, w| {
            *acc.entry((w[0], w[1])).or_insert(0) += 1;
            return acc;
        });
    
    for _ in 0..times {
        for (pair, count) in pairs_count.clone() {
            let insert = rules.get(&pair).unwrap();
            let (a, b) = pair;
            *pairs_count.entry(pair).or_insert(0) -= count;
            *pairs_count.entry((a, *insert)).or_insert(0) += count;
            *pairs_count.entry((*insert, b)).or_insert(0) += count;
            *chars_count.entry(insert).or_insert(0) += count;
        }
    }

    let (_, most_common_char) = chars_count.iter().max_by_key(|(_, v)| *v).unwrap();
    let (_, least_common_char) = chars_count.iter().min_by_key(|(_, v)| *v).unwrap();

    return most_common_char - least_common_char;
}

static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (u64, u64) { 
    let (polymer, rules) = parse_input(INPUT);
    let part1 = run_insertion_rules(&polymer, &rules, 10);
    let part2 = run_insertion_rules(&polymer, &rules, 40);
    
    return (part1, part2);
}