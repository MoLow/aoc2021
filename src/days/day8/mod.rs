use std::{collections::{HashMap, HashSet}};


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum DisplaySegment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}


static UNIQ_NUMBERS: [i32;4] = [1,4,7,8];


fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    return input
        .lines()
        .map(|line| {
            let mut parts = line.split('|').collect::<Vec<&str>>();
            let input = parts.remove(0).split_whitespace().collect::<Vec<&str>>();
            let output = parts.remove(0).split_whitespace().collect::<Vec<&str>>();
            return (input, output);
        })
        .collect();
}

fn count_output_by_lengths(entries: &Vec<(Vec<&str>, Vec<&str>)>, numbers: &Vec<Vec<DisplaySegment>>) -> usize {
    let uniq_numbers_lengths = UNIQ_NUMBERS.map(|x| numbers[x as usize].len());

    return entries
        .iter()
        .map(|entry| {
            let (_input, output) = entry;
            return output.iter().filter(|item| uniq_numbers_lengths.contains(&item.len())).count();
        })
        .sum();
}

fn calculate_mapping(input: &Vec<&str>, numbers: &Vec<Vec<DisplaySegment>>) -> HashMap<i32, HashSet<char>> {
    let mut possibilities: HashMap<i32, Vec<HashSet<char>>> = HashMap::new();

    (0..10).for_each(|number| input
            .iter()
            .filter(|x| x.len() == numbers[number as usize].len())
            .for_each(|item| possibilities.entry(number).or_insert(Vec::new()).push(item.chars().collect())));

    let one = possibilities.get(&1).unwrap().first().unwrap();
    let one_vec = one.iter().collect::<Vec<&char>>();
    let four = possibilities.get(&4).unwrap().first().unwrap();
    let three = possibilities.get(&3).unwrap().iter().find(|t| t.is_superset(one)).unwrap().clone();
    let six = possibilities.get(&6).unwrap().iter().find(|t| t.contains(one_vec[0]) ^ t.contains(one_vec[1])).unwrap().clone();
    let nine = possibilities.get(&9).unwrap().iter().find(|t| t.is_superset(four)).unwrap().clone();
    let zero = possibilities.get(&0).unwrap().iter().find(|t| !t.is_superset(&six) && !t.is_superset(&nine)).unwrap().clone();
    let one_six_intersection = *one.intersection(&six).collect::<Vec<&char>>().first().unwrap();
    let five = possibilities.get(&5).unwrap().iter().find(|t| !t.is_superset(&three) && t.contains(one_six_intersection)).unwrap().clone();
    let two = possibilities.get(&2).unwrap().iter().find(|t| !t.is_superset(&three) && !t.is_superset(&five)).unwrap().clone();
    possibilities.insert(3, vec![three]);
    possibilities.insert(6, vec![six]);
    possibilities.insert(9, vec![nine]);
    possibilities.insert(0, vec![zero]);
    possibilities.insert(5, vec![five]);
    possibilities.insert(2, vec![two]);
    
    return possibilities.iter().map(|(k, v)| (*k, v.first().unwrap().clone())).collect();
}

fn decode_outputs(mapping: &HashMap<i32, HashSet<char>>, output: &Vec<&str>) -> i32 {
    return output.iter().rev().enumerate().fold(0, |acc, (i, item)| {
        let chars = item.chars().collect::<HashSet<char>>();
        let num = mapping.iter().find(|(_, v)| v.is_subset(&chars) && v.is_superset(&chars)).unwrap().0;
        return  acc + num * 10_i32.pow(i as u32);
    });
}


static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (u64, u64) { 
    let numbers: Vec<Vec<DisplaySegment>> = vec![
        vec![DisplaySegment::A, DisplaySegment::B, DisplaySegment::C, DisplaySegment::E, DisplaySegment::F, DisplaySegment::G],                     // 0: 6
        vec![DisplaySegment::C, DisplaySegment::F],                                                                                                 // 1: 2 - unique
        vec![DisplaySegment::A, DisplaySegment::C, DisplaySegment::D, DisplaySegment::E, DisplaySegment::G],                                        // 2: 5
        vec![DisplaySegment::A, DisplaySegment::C, DisplaySegment::D, DisplaySegment::F, DisplaySegment::G],                                        // 3: 5
        vec![DisplaySegment::B, DisplaySegment::C, DisplaySegment::D, DisplaySegment::F],                                                           // 4: 4 - unique
        vec![DisplaySegment::A, DisplaySegment::B, DisplaySegment::D, DisplaySegment::F, DisplaySegment::G],                                        // 5: 5
        vec![DisplaySegment::A, DisplaySegment::B, DisplaySegment::D, DisplaySegment::E, DisplaySegment::F, DisplaySegment::G],                     // 6: 6
        vec![DisplaySegment::A, DisplaySegment::C, DisplaySegment::F],                                                                              // 7: 3 - unique
        vec![DisplaySegment::A, DisplaySegment::B, DisplaySegment::C, DisplaySegment::D, DisplaySegment::E, DisplaySegment::F, DisplaySegment::G],  // 8: 7 - unique
        vec![DisplaySegment::A, DisplaySegment::B, DisplaySegment::C, DisplaySegment::D, DisplaySegment::F, DisplaySegment::G],                     // 9: 6
    ];

    let entries = parse_input(&INPUT);

    let part1 = count_output_by_lengths(&entries, &numbers) as u64;
    let part2 = entries
        .iter()
        .map(|entry| {
            let (input, output) = entry;
            let mapping = calculate_mapping(input, &numbers);
            return decode_outputs(&mapping, output);
        })
        .sum::<i32>()  as u64;

    return (part1, part2);
}