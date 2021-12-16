use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    return input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect())
        .collect();
}

fn increase_adjacent(board: &mut Vec<Vec<usize>>, x: usize, y: usize) {
    if y > 0 && x > 0 { board[y - 1][x - 1] += 1; }
    if y > 0 { board[y - 1][x] += 1; }
    if y > 0 && x < board[y].len() - 1 { board[y - 1][x + 1] += 1; }
    if x > 0 { board[y][x - 1] += 1; }
    if x < board[y].len() - 1 { board[y][x + 1] += 1; }
    if y < board.len() - 1 && x > 0 { board[y + 1][x - 1] += 1; }
    if y < board.len() - 1 { board[y + 1][x] += 1; }
    if y < board.len() - 1 && x < board[y].len() - 1 { board[y + 1][x + 1] += 1; }
}

fn run_step(board: &mut Vec<Vec<usize>>) -> usize {
    board.iter_mut().for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));
    
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let mut has_flashed = false;
        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[y][x] > 9 && !flashed.contains(&(x, y)) {
                    increase_adjacent(board, x, y);
                    has_flashed = true;
                    flashed.insert((x, y));
                } 
            }
        }
        if !has_flashed {
            break;
        }
    }
    

    board.iter_mut().for_each(|row| row.iter_mut().for_each(|cell| {
        if *cell > 9 {
            *cell = 0;
        }
    }));

    return flashed.len();
}

fn run_steps(board: &mut Vec<Vec<usize>>, steps: usize) -> usize {
    let flashes: usize = (0..steps).map(|_i| run_step(board)).sum();
    return flashes;
}

fn run_until_all_flash(board: &mut Vec<Vec<usize>>) -> usize {
    let mut counter = 0;
    loop {
        counter += 1;
        let flashes = run_step(board);
        if flashes == board.len() * board[0].len() {
            break;
        }
    }
    return counter;
}


static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (u64, u64) { 
    let mut entries = parse_input(INPUT);
    
    let part1 = run_steps(&mut entries.clone(), 100) as u64;
    let part2 = run_until_all_flash(&mut entries) as u64;
    
    return (part1, part2);
}