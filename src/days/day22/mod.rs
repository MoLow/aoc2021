use std::{collections::{HashSet, HashMap}, ops::Range};

static INPUT: &str = include_str!("./input.txt");


type Position = (i32, i32, i32);
type Ranges = (Range<i32>, Range<i32>, Range<i32>);

struct Step {
    flag: bool,
    ranges: Ranges,
}
impl std::str::FromStr for Step {
    type Err = std::num::ParseIntError;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut x = input.split_whitespace();
        let flag = x.next().unwrap() == "on";
        let ranges = x.next().unwrap().split(",")
            .map(|val| {
                let values = val.split("=").last().unwrap().split("..").map(|val| val.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                return values[0]..values[1];
            }).collect::<Vec<Range<i32>>>();

        return Ok(Step { flag, ranges: (ranges[0].clone(), ranges[1].clone(), ranges[2].clone()) });
    }
}
impl Step {
    fn restrict(&self, min: i32, max: i32) -> Self {
        let (x, y, z) = self.ranges.clone();
        let new_x = x.start.max(min)..(x.end).min(max);
        let new_y = y.start.max(min)..(y.end).min(max);
        let new_z = z.start.max(min)..(z.end).min(max);
        return Step { flag: self.flag, ranges: (new_x, new_y, new_z) };
    }
    fn intersect(&self, other: Ranges) -> Option<Ranges> {
        let (x, y, z) = self.ranges.clone();
        let (x2, y2, z2) = other.clone();
        let new_x = x.start.max(x2.start)..x.end.min(x2.end);
        let new_y = y.start.max(y2.start)..y.end.min(y2.end);
        let new_z = z.start.max(z2.start)..z.end.min(z2.end);
        if new_x.start > new_x.end || new_y.start > new_y.end || new_z.start > new_z.end {
            return None;
        }
        return Some((new_x, new_y, new_z));
    }
    fn apply(&self, map: &mut HashSet<Position>) {
        let (x, y, z) = self.ranges.clone();
        for x in x.start..x.end+1 {
            for y in y.start..y.end+1 {
                for z in z.start..z.end+1 {
                    if self.flag {
                        map.insert((x, y, z));
                    } else {
                        map.remove(&(x, y, z));
                    }
                }
            }
        }
    }
}

struct Game {
    steps: Vec<Step>,
}
impl std::str::FromStr for Game {
    type Err = std::num::ParseIntError;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let steps = input.lines().map(|line| line.parse::<Step>().unwrap()).collect::<Vec<Step>>();

        return Ok(Game { steps });
    }
}

impl Game {
    fn part1(&mut self) -> u64 {
        let mut map = HashSet::new();
        for step in self.steps.iter() {
            step.restrict(-50, 50).apply(&mut map);
        }
        return map.len() as u64;
    }
    fn part2(&mut self) -> u64 {
        let mut map: HashMap<Ranges, i64> = HashMap::new();
        for step in self.steps.iter() {
            let mut new_map = map.clone();
            for (entry, count) in map.iter_mut() {
                if let Some(intersection) = step.intersect(entry.clone()) {
                    *new_map.entry(intersection).or_insert(0) -= *count;
                }
            }
            if step.flag {
                *new_map.entry(step.ranges.clone()).or_insert(0) += 1;
            }
            map = new_map;
        }

        return map
            .iter()
            .map(|(coords, &count)| {
                [coords.0.clone(), coords.1.clone(), coords.2.clone()]
                    .iter()
                    .map(|range| (range.end - range.start +1) as i64 * count)
                    .product::<i64>()
            })
            .sum::<i64>() as u64;
    }
}

pub fn run() -> (u64, u64) {
    let part1 = INPUT.parse::<Game>().unwrap().part1();
    let part2 = INPUT.parse::<Game>().unwrap().part2();


    return (part1, part2);
}