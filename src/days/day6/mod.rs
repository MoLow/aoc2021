pub fn count_fish(all_fish: &Vec<usize>, days: usize) -> Vec<u64> {
    let mut counter: Vec<u64> = vec![0; 9];

    all_fish.iter().for_each(|&f| {
        counter[f] += 1;
    });

    for _day in 0..days {
        let mut add_count = 0;
        for i in 0..9 {
            if i == 0 {
                counter[7] += counter[0];
                add_count += counter[0];
                counter[0] = 0;
            } else {
                counter[i - 1] += counter[i];
                counter[i] = 0;
            }
        }
        counter[8] += add_count;
    }

    return counter;
}


static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (u64, u64) {
    let all_fish = INPUT
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
 
    let part1 = count_fish(&all_fish, 80).iter().sum();
    let part2 = count_fish(&all_fish, 256).iter().sum();

    return (part1, part2);
}