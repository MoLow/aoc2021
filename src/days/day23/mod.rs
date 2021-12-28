use std::{collections::{HashMap, BinaryHeap, HashSet}, cmp::Reverse};

static INPUT: &str = include_str!("./input.txt");
static INPUT2: &str = include_str!("./input2.txt");

const ROOMS: [i32; 4] = [3, 5, 7, 9];

type Coordinate = (i32, i32);

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
enum Amphipod {
    Empty,
    Amber,
    Bronze,
    Copper,
    Desert,
}
impl Amphipod {
    fn from_char(c: char) -> Option<Self> {
        return match c {
            'A' => Some(Amphipod::Amber),
            'B' => Some(Amphipod::Bronze),
            'C' => Some(Amphipod::Copper),
            'D' => Some(Amphipod::Desert),
            '.' => Some(Amphipod::Empty),
            _ => None,
        }
    }
    fn cost(&self) -> i32 {
        match self {
            Amphipod::Empty => 0,
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
    fn get_x_destination(&self) -> i32 {
        match self {
            Amphipod::Amber => ROOMS[0],
            Amphipod::Bronze => ROOMS[1],
            Amphipod::Copper => ROOMS[2],
            Amphipod::Desert => ROOMS[3],
            _ => panic!("Invalid destination"),
        }
    }
    fn from_x_destination(dest: i32) -> usize {
        match dest {
            3 => 0,
            5 => 1,
            7 => 2,
            9 => 3,
            _ => panic!("Invalid destination"),
        }
    }
    fn print(&self) {
        match self {
            Amphipod::Amber => print!("A"),
            Amphipod::Bronze => print!("B"),
            Amphipod::Copper => print!("C"),
            Amphipod::Desert => print!("D"),
            _ => print!("."),
        }
    }
}

#[derive(Clone, Debug, Hash)]
struct Burrow {
    rooms: [Vec<Amphipod>; 4],
    hallway: [Amphipod; 11],
    score: i32
}
impl Ord for Burrow {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}
impl PartialOrd for Burrow {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Burrow {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl Eq for Burrow {}

impl std::str::FromStr for Burrow {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rooms = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let mut hallway = [Amphipod::Empty; 11];

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(amphipod) = Amphipod::from_char(c) {
                    if y == 1 {
                        hallway[x as usize - 1] = amphipod;
                    } else {
                        rooms[Amphipod::from_x_destination(x as i32)].push(amphipod);
                    }
                }
            }
        }

        Ok(Burrow { score: 0, rooms, hallway })
    }
}
impl Burrow {
    fn print(&self) {        
        println!("#############");
        print!("#");
        for amphipod in self.hallway {
            amphipod.print()
        }
        print!("#");
        println!();
        let room_size = self.rooms[0].len();
        for line in 0..room_size {
            if line == 0 { print!("###") } else { print!("  #") };
            for room in 0..4 {
                self.rooms[room][line].print();
                print!("#");
            }
            if line == 0 { print!("##") } else { print!("  ") };
            println!();
        }
        println!("  #########  ");
    }

    fn is_hallway_free(&self, start: i32, end: i32) -> bool {
        let from = start.min(end)-1;
        let to = start.max(end);
        return self.hallway[from as usize..to as usize].iter().all(|amphipod| *amphipod == Amphipod::Empty);
    }

    fn manhattan_distance(from: Coordinate, to: Coordinate) -> i32 {
        return (from.0 - to.0).abs() + (from.1 - to.1).abs();
    }

    fn next_from_hallway(&self) -> Vec<Self> {
        let mut moves = Vec::new();
        for (i, &item) in self.hallway.iter().enumerate() {
            if item == Amphipod::Empty {
                continue;
            }
            let origin_x = i as i32 + 1;
            let dest_x = item.get_x_destination();

            let dx = if origin_x < dest_x { 1 } else { -1 };
            let hallway_free = self.is_hallway_free(origin_x + dx, dest_x);
            if !hallway_free {
                continue;
            }

            let room = Amphipod::from_x_destination(dest_x);
            let mut lowest_room = 0;
            loop {
                if lowest_room >= self.rooms[room].len() || self.rooms[room][lowest_room] != Amphipod::Empty {
                    break;
                }
                lowest_room += 1;
            }

            if self.rooms[room][lowest_room..].iter().any(|amphipod| amphipod.get_x_destination() != dest_x) {
                continue;
            }

            if lowest_room > 0 {
                let mut new = self.clone();
                let cost = Burrow::manhattan_distance((origin_x, 1), (dest_x, lowest_room as i32 + 1)) * item.cost();
                new.hallway[i] = Amphipod::Empty;
                new.rooms[room][lowest_room - 1] = item;
                new.score += cost;
                moves.push(new);
            }

        }
        return moves;
    }

    fn next_from_rooms(&self) -> Vec<Self> {
        let mut moves = Vec::new();
        for (index, room) in self.rooms.iter().enumerate() {
            let mut lowest_room = 0;
            loop {
                if lowest_room >= room.len() || room[lowest_room] != Amphipod::Empty {
                    break;
                }
                lowest_room += 1;
            }
            if lowest_room >= room.len() {
                continue;
            }

            let position = ROOMS[index];
            
            if room[lowest_room..].iter().all(|amphipod| amphipod.get_x_destination() == position) {
                continue;
            }
            
            
            for x in 1..12 {
                if !ROOMS.contains(&x) && self.is_hallway_free(position, x) {
                    let mut new = self.clone();
                    let cost = Burrow::manhattan_distance((position, lowest_room as i32 + 2), (x, 1)) * room[lowest_room].cost();
                    new.hallway[x as usize - 1] = room[lowest_room];
                    new.rooms[index][lowest_room] = Amphipod::Empty;
                    new.score += cost;
                    moves.push(new);
                }
            }
        }
        return moves;
    }

    fn next_moves(&self) -> Vec<Self> {
        let mut moves = self.next_from_hallway();
        moves.append(&mut self.next_from_rooms());
        return moves;
    }

    fn is_organized(&self) -> bool {
        return self.rooms
            .iter()
            .enumerate()
            .all(|(i,room)| room
                .iter()
                .all(|amphipod| amphipod != &Amphipod::Empty && amphipod.get_x_destination() == ROOMS[i]));
    }
}

fn dijkstra(burrow: &Burrow) -> u64 {
    let mut queue = BinaryHeap::new();
    let mut lowest_costs = HashMap::new();
    let mut visited: HashSet<Burrow> = HashSet::default();

    let mut tester = vec![((10, 1), Amphipod::Amber), ((3, 3),  Amphipod::Amber), ((7, 2),  Amphipod::Copper), ((5, 3),  Amphipod::Bronze), ((5, 2),  Amphipod::Bronze), ((7, 3),  Amphipod::Copper), ((8, 1),  Amphipod::Desert), ((6, 1),  Amphipod::Desert)];
    tester.sort_by_key(|&(x, _y)| x);

    queue.push(Reverse(burrow.clone()));

    
    while let Some(Reverse(burrow)) = queue.pop() {
        if  burrow.is_organized() {
            println!("{}", burrow.score);
            burrow.print();

            return burrow.score as u64;
        }

        if burrow.score > *lowest_costs.get(&burrow).unwrap_or(&i32::MAX) {
            continue;
        }
        
        for next in burrow.next_moves() {
            if visited.insert(next.clone()) {
                let lowest = lowest_costs.entry(next.clone()).or_insert(i32::MAX);
                
                if next.score < *lowest {
                    *lowest = next.score;
                    queue.push(Reverse(next));
                }
            }
        }
    }
    return 0;
}


pub fn run() -> (u64, u64) {
    let burrow1 = INPUT.parse::<Burrow>().unwrap();
    let burrow2 = INPUT2.parse::<Burrow>().unwrap();

    let part1= dijkstra(&burrow1);
    let part2= dijkstra(&burrow2);
    return (part1, part2);
}