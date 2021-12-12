use std::collections::{HashSet, HashMap};

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let connections = input
        .lines()
        .map(|line| line.split('-').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    for connection in connections {
        let from = connection[0];
        let to = connection[1];
        map.entry(from).or_insert(HashSet::new()).insert(to);
        map.entry(to).or_insert(HashSet::new()).insert(from);
    }

    return map;
}

fn find_paths<'a, F: Fn(&Vec<&str>, &str) -> bool>(connections: &HashMap<&str, HashSet<&'a str>>, from: &'a str, to: &str, current_path: &mut Vec<&'a str>, allow_recur: &F) -> Vec<Vec<&'a str>> {
    let mut paths = Vec::new();
    current_path.push(from);

    if from == to {
        paths.push(current_path.clone());
        return paths;
    }


    for next in connections.get(from).unwrap() {
        if allow_recur(current_path, next) {
            let mut curr = current_path.clone();
            let sub_paths = find_paths(connections, next, to, &mut curr, allow_recur);
            paths.extend(sub_paths);
        }
    }

    return paths;
}


pub fn run(input: String) { 
    let connections = parse_input(&input);

    let part1 = find_paths(&connections, "start","end", &mut Vec::new(), &|path, cave| {
        return !path.contains(&cave) || cave.chars().all(|c| c.is_uppercase());
    });
    println!("Part1: {:?}", part1.len());

    let part2 = find_paths(&connections, "start","end", &mut Vec::new(), &|path, cave: &str| {
        if cave.chars().all(|c| c.is_uppercase()) {
            return true;
        }
        if cave == "start" || cave == "end" {
            return !path.contains(&cave);
        }

        let small_caves = path.iter().filter(|cave| cave.chars().all(|c| c.is_lowercase())).collect::<Vec<&&str>>();
        let uniq_small_caves = small_caves.iter().collect::<HashSet<&&&str>>();

        if (small_caves.len() - uniq_small_caves.len()) < 1 {
            return true;
        }

        return !path.contains(&cave);
    });
    println!("Part2: {:?}", part2.len());
}