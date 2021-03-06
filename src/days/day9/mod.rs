use std::{collections::HashSet};

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    return input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect())
        .collect();
}

fn get_neighbors(x: usize, y: usize, grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if x > 0 { neighbors.push((x - 1, y)); }
    if x < grid[0].len() - 1 { neighbors.push((x + 1, y)); }
    if y > 0 { neighbors.push((x, y - 1)); }
    if y < grid.len() - 1 { neighbors.push((x, y + 1)); }
    
    return neighbors;
}

fn find_low_points(grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    return grid.iter().enumerate().fold(Vec::new(), |acc, (y, row)| {
        return row.iter().enumerate().fold(acc, |mut acc, (x, &cell)| {
            if get_neighbors(x, y, grid).iter().all(|(nx, ny)| grid[*ny][*nx] > cell) {
                acc.push((x, y));
            }
            return acc;
        })
    });
}

fn get_basin(grid: &Vec<Vec<i32>>, start: (usize, usize), basin: &mut HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let (x, y) = start;
    let neighbors = get_neighbors(x, y, grid);
    let relevant_neighbors = neighbors.iter()
        .filter(|n| grid[n.1][n.0] < 9 && !basin.contains(n))
        .collect::<HashSet<&(usize, usize)>>();
    
    basin.insert(start);
    basin.extend(relevant_neighbors.clone());

    let n = relevant_neighbors
        .iter()
        .map(|&n| get_basin(grid, *n, basin))
        .flatten()
        .collect::<HashSet<(usize, usize)>>();

    
    basin.extend(n);
    return basin.clone();
}

fn find_basins(grid: &Vec<Vec<i32>>, low_points: &Vec<(usize, usize)>) -> Vec<i32> {
    return low_points.iter()
        .map(|point| get_basin(grid, *point, &mut HashSet::new()).len() as i32)
        .collect();
}


static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (u64, u64) { 
    let entries = parse_input(INPUT);
    let low_points = find_low_points(&entries);
    let part1 = low_points.iter().map(|(x, y)| entries[*y][*x] + 1).sum::<i32>() as u64;
    
    let mut basins = find_basins(&entries, &low_points);
    basins.sort();
    let part2 = basins.iter().rev().take(3).product::<i32>() as u64;

    return (part1, part2);
}