use std::{collections::HashSet};



fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<(i32, i32)>) {
    let dots = input
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("fold"))
        .map(|line| line.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|line| (line[0], line[1]))
        .collect::<HashSet<(i32, i32)>>();

    let folds = input
        .lines()
        .filter(|line| line.starts_with("fold"))
        .map(|line| {
            let l = line.split("=").collect::<Vec<&str>>();
            let cmd = l[0].chars();
            return match cmd.last().unwrap() {
                'y' => (0, l[1].parse::<i32>().unwrap()),
                'x' => (l[1].parse::<i32>().unwrap(), 0),
                _ => (0, 0),
            };
        })
        .collect::<Vec<(i32, i32)>>();

    return (dots, folds);
}

fn fold_dots(dots: &HashSet<(i32, i32)>, fold: &(i32, i32)) -> HashSet<(i32, i32)> {
    let (x, y) = *fold;
    let mut new_dots = HashSet::new();
    for dot in dots {
        let (dot_x, dot_y) = *dot;
        let mut new_dot = (dot_x, dot_y);
        if dot_x >= x {
            new_dot.0 = (x + (x - dot_x)).abs();
        }
        if dot_y >= y {
            new_dot.1 = (y + (y - dot_y)).abs();
        }
        new_dots.insert(new_dot);
    }
    return new_dots;
}

fn fold_all_dots(dots: &HashSet<(i32, i32)>, folds: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_dots = dots.clone();
    for fold in folds {
        new_dots = fold_dots(&new_dots, fold);
    }
    return new_dots;
}

pub fn draw(board: &HashSet<(i32, i32)>) -> String {
    let max_x = *board.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *board.iter().map(|(_, y)| y).max().unwrap();
    let mut result = String::new();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if board.contains(&(x, y)) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }

    return result;
}

static INPUT: &str = include_str!("./input.txt");
pub fn run<P: FnMut(&str) -> ()>(mut printer: P) -> (usize, usize) { 
    let (dots, folds) = parse_input(INPUT);
    let part1 = fold_dots(&dots, &folds[0]).len();
    
    printer(&draw(&fold_all_dots(&dots, &folds)));
    return (part1, 0);
}