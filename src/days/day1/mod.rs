pub fn count_increasing(input: &String, window_size: usize) -> i32 {
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

pub fn run(input: String) {
    let input_ref = &input;
    println!("total increasing: {}", count_increasing(input_ref, 1));
    println!("total increasing - part 2: {}", count_increasing(input_ref, 3));
}