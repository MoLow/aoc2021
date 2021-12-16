use std::{collections::{HashMap, BinaryHeap, HashSet}, cmp::Reverse};

struct RiskMap {
    risk_levels: HashMap<(usize, usize), usize>,
}
impl std::str::FromStr for RiskMap {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut risk_levels = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, num) in line.chars().enumerate() {
                risk_levels.insert((x as usize, y as usize), num.to_digit(10).unwrap() as usize);
            }
        }
        return Ok(RiskMap { risk_levels });
    }
}
impl RiskMap {
    fn size(&self) -> usize {
        return f64::sqrt(self.risk_levels.len() as f64) as usize - 1;
    }

    fn multiply(&self, times: usize) -> RiskMap {
        let mut risk_levels = HashMap::new();
        let (&(size_x, size_y), _) = self.risk_levels.iter().max_by_key(|((x, y), _)| x + y).unwrap();

        for time_x in 0..times {
            for time_y in 0..times {
                for &(x, y) in self.risk_levels.keys() {
                    let risk = (self.risk_levels.get(&(x, y)).unwrap() + time_x + time_y) % 9;
                    risk_levels.insert(((size_x + 1) * time_x + x, (size_y + 1) * time_y + y), match risk { 0 => 9, _ => risk });
                }
            }
        }

        return RiskMap { risk_levels };
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.size() {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.size() {
            neighbors.push((x, y + 1));
        }

        return neighbors;
    }

    fn dijkstra(&self) -> u16{
        let size = self.size();
        let mut lowest_costs= HashMap::new();
        let mut queue= BinaryHeap::new();

        lowest_costs.insert((0,0),0);
        queue.push(Reverse((0,(0,0))));

        let mut visited= HashSet::new();
        while let Some(Reverse((risk,(x,y)))) = queue.pop() {
            if visited.insert((x,y)){
                for (x,y) in self.get_neighbors(x,y){
                    let new_risk = self.risk_levels.get(&(x,y)).unwrap();
                    if risk + new_risk< *lowest_costs.get(&(x,y)).unwrap_or(&u16::MAX) as usize {
                        lowest_costs.insert((x,y), (risk + new_risk) as u16);
                        queue.push(Reverse((risk+new_risk,(x,y))))
                    }
                }
            }
        }
        return *lowest_costs.get(&(size,size)).unwrap();
    }
}

static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (u64, u64) { 
    let risk_map: RiskMap = INPUT.parse().unwrap();
    let part1 = risk_map.dijkstra() as u64;
    let risk_map = risk_map.multiply(5);
    let part2 = risk_map.dijkstra() as u64;

    return (part1, part2);
}