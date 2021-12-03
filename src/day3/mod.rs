fn get_lines(input: &String) -> Vec<Vec<char>> {
    return input
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();
}

fn most_common_at_index(input: &Vec<Vec<char>>, index: usize) -> (u32, u32) {
    let total_size: u32 = input.len().try_into().unwrap();
    let half_size: u32 = total_size / 2;
    let zeros: u32 = input.iter().filter(|&x| x[index] == '0').count().try_into().unwrap();

    if zeros > half_size {
        return (0, 1) 
    } else { 
        return (1, 0)
    }
}

fn calculate_gamma_and_epsilon_rate(input: &String) -> (u32, u32) {
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    let lines = get_lines(&input);
    let line_length = lines[0].len();
    for i in 0..line_length {
        let (g, e) = most_common_at_index(&lines, i);
        let shift = line_length - i - 1;
        gamma += g << shift;
        epsilon += e << shift;
    }

    return (gamma, epsilon);
}

enum RatingMode {
    MostCommon,
    LeastCommon,
}

fn calculate_rating(input: &String, mode: RatingMode) -> isize {
    let mut lines = get_lines(&input);
    let mut index = 0;
    let line_length = lines[0].len();

    while lines.len() > 1 {
        let (most_common, least_common) = most_common_at_index(&lines, index);
        let rating = match mode {
            RatingMode::MostCommon => most_common,
            RatingMode::LeastCommon => least_common,
        };
        lines = lines
            .iter()
            .filter(|&x| {
                let my_int: u32 = x[index].to_digit(10).unwrap().try_into().unwrap();
                return my_int == rating
            })
            .map(|x| x.to_vec())
            .collect();
        
        index = (index + 1) % line_length;
    }

    let final_line: String = lines[0].iter().collect();
    return isize::from_str_radix(&final_line, 2).unwrap();
}

pub fn run(input: String) {
    let (gamma, epsilon) = calculate_gamma_and_epsilon_rate(&input);
    println!("gamma: {:b} X epsilon: {:b} = {}", gamma, epsilon, gamma * epsilon);

    let oxygen_generator_rating = calculate_rating(&input, RatingMode::MostCommon);
    let co2_scrub_rating = calculate_rating(&input, RatingMode::LeastCommon);
    println!("oxygen generator rating: {} X co2 scrub rating: {} = {}", oxygen_generator_rating, co2_scrub_rating, oxygen_generator_rating * co2_scrub_rating);
}