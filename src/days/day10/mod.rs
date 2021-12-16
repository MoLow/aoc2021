fn parse_input(input: &str) -> Vec<Vec<char>> {
    return input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
}


#[derive(Debug)]
enum LineType {
    Valid,
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn parse_line(line: &Vec<char>) -> LineType {
    let mut stack: Vec<char> = Vec::new();
    for &c in line {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        }
        if c == ')' || c == ']' || c == '}' || c == '>' {
            let stack_end = stack.pop().unwrap();
            if stack_end != match c {
                ')' => '(',
                ']' => '[',
                '}' => '{',
                '>' => '<',
                _ => panic!("Invalid character")
            } {
               return LineType::Corrupted(c);
            }
        }
    }
    if stack.len() > 0 {
        stack.reverse();
        return LineType::Incomplete(stack);
    }

    return LineType::Valid;
}

fn calculate_corrupted_scores(lines: &Vec<LineType>) -> usize {
    return lines.iter().map(|line| match line {
        LineType::Corrupted(c) => match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0
        },
        _ => 0,
    })
    .sum::<usize>();
}

fn calculate_incomplete_scores(lines: &Vec<LineType>) -> usize {
    let mut scores = lines
        .iter()
        .map(|line| match line {
            LineType::Incomplete(stack) => {
                let mut score = 0;
                for &c in stack.iter() {
                    score *=5;
                    match c {
                        '(' => score += 1,
                        '[' => score += 2,
                        '{' => score += 3,
                        '<' => score += 4,
                        _ => panic!("Invalid token")
                    }
                }
                return score;
            },
            _ => 0,
        })
        .filter(|score| *score > 0)
        .collect::<Vec<usize>>();

    scores.sort();

    return *scores.get(scores.len() / 2).unwrap();
}

static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (usize, usize) { 
    let entries = parse_input(INPUT);
    let lines = entries.iter().map(|line| parse_line(line)).collect::<Vec<LineType>>();

    return (calculate_corrupted_scores(&lines), calculate_incomplete_scores(&lines));
}