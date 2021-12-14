use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    return input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect())
        .collect();
}

fn increase_adjacent(board: &mut Vec<Vec<i32>>, x: usize, y: usize) {
    if y > 0 && x > 0 { board[y - 1][x - 1] += 1; }
    if y > 0 { board[y - 1][x] += 1; }
    if y > 0 && x < board[y].len() - 1 { board[y - 1][x + 1] += 1; }
    if x > 0 { board[y][x - 1] += 1; }
    if x < board[y].len() - 1 { board[y][x + 1] += 1; }
    if y < board.len() - 1 && x > 0 { board[y + 1][x - 1] += 1; }
    if y < board.len() - 1 { board[y + 1][x] += 1; }
    if y < board.len() - 1 && x < board[y].len() - 1 { board[y + 1][x + 1] += 1; }
}

fn run_step(board: &mut Vec<Vec<i32>>) -> i32 {
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

    return flashed.len() as i32;
}

fn run_steps(board: &mut Vec<Vec<i32>>, steps: i32) -> i32 {
    let flashes: i32 = (0..steps).map(|_i| run_step(board)).sum();
    return flashes;
}

fn run_until_all_flash(board: &mut Vec<Vec<i32>>) -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;
        let flashes = run_step(board);
        if flashes == board.len() as i32 * board[0].len() as i32 {
            break;
        }
    }
    return counter;
}


pub fn run(input: String) { 
    let mut entries = parse_input(&input);
    
    let flash_count = run_steps(&mut entries.clone(), 100);
    println!("Part1: {}", flash_count);
    let flash_count = run_until_all_flash(&mut entries);
    println!("Part2: {}", flash_count);
}