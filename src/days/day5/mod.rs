use std::cmp::{max};

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}
impl std::str::FromStr for Line {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.replace(" -> ",",");
        let mut iter = normalized.split(",");
        let x1 = iter.next().unwrap().parse::<usize>().unwrap();
        let y1 = iter.next().unwrap().parse::<usize>().unwrap();
        let x2 = iter.next().unwrap().parse::<usize>().unwrap();
        let y2 = iter.next().unwrap().parse::<usize>().unwrap();
        return Ok(Line { x1, y1, x2, y2 });
    }
}

impl Line {
    fn direction_x (&self) -> i32 {
        return (self.x2 as i32 - self.x1 as i32).signum();
    }
    fn direction_y (&self) -> i32 {
        return (self.y2 as i32 - self.y1 as i32).signum();
    }
}
    

fn parse_input(input: &str) -> Vec<Line> {
    return input.lines().map(|l| l.parse::<Line>().unwrap()).collect();
}

fn build_map(lines: Vec<&Line>) -> Vec<Vec<usize>> {
    let max_x = lines.iter().map(|l| max(l.x1,l.x2)).max().unwrap() + 1;
    let max_y = lines.iter().map(|l| max(l.y1, l.y2)).max().unwrap() + 1;
    let mut map = vec![vec![0; max_x]; max_y];

    for line in lines {
        let mut x = line.x1;
        let mut y = line.y1;
        let dx = line.direction_x();
        let dy = line.direction_y();
        loop {
            map[y][x] += 1;
            x = (x as i32 + dx) as usize;
            y = (y as i32+ dy) as usize;

            if x == line.x2 && y == line.y2 {
                map[y][x] += 1;
                break;
            }
        }
    }

    return map;
}


static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (usize, usize) {
    let lines = parse_input(INPUT);
    let straight_lines = lines.iter()
        .filter(|line| line.x1 == line.x2 || line.y1 == line.y2)
        .collect::<Vec<&Line>>();

    let all_lines = lines.iter().collect::<Vec<&Line>>();
    
    let part1 = build_map(straight_lines).iter().map(|line| line.iter().filter(|&&x| x > 1).count()).sum::<usize>();
    let part2 = build_map(all_lines).iter().map(|line| line.iter().filter(|&&x| x > 1).count()).sum::<usize>();

    return (part1, part2);
}