fn get_lines(input: &str) -> Vec<Vec<char>> {
    return input
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();
}

fn most_common_at_index(input: &Vec<Vec<char>>, index: usize) -> (usize, usize) {
    let total_size: usize = input.len().try_into().unwrap();
    let half_size: usize = total_size / 2;
    let zeros: usize = input.iter().filter(|&x| x[index] == '0').count().try_into().unwrap();

    if zeros > half_size {
        return (0, 1) 
    } else { 
        return (1, 0)
    }
}

fn calculate_gamma_and_epsilon_rate(input: &str) -> (usize, usize) {
    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    let lines = get_lines(input);
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

fn calculate_rating(input: &str, mode: RatingMode) -> usize {
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
                let my_int: usize = x[index].to_digit(10).unwrap().try_into().unwrap();
                return my_int == rating
            })
            .map(|x| x.to_vec())
            .collect();
        
        index = (index + 1) % line_length;
    }

    let final_line: String = lines[0].iter().collect();
    return usize::from_str_radix(&final_line, 2).unwrap();
}

static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (usize, usize) {
    let (gamma, epsilon) = calculate_gamma_and_epsilon_rate(INPUT);

    let oxygen_generator_rating = calculate_rating(INPUT, RatingMode::MostCommon);
    let co2_scrub_rating = calculate_rating(INPUT, RatingMode::LeastCommon);
    return (gamma * epsilon, oxygen_generator_rating * co2_scrub_rating);
}