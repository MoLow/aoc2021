static TOTAL_DAYS: i32 = 256;

pub fn run(input: String) {
    let mut counter: Vec<usize> = vec![0; 9];

    let all_fish = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
 
    all_fish.iter().for_each(|&f| {
        counter[f] += 1;
    });

    for _day in 0..TOTAL_DAYS {
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

    println!("{}", counter.iter().sum::<usize>());
}