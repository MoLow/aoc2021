use std::collections::{HashMap, HashSet};


static INPUT: &str = include_str!("./input.txt");

type Transformation = (i32, (i32, i32, i32));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}
impl std::str::FromStr for Beacon {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split(",").map(|x| x.parse::<i32>().unwrap());
        return Ok(Beacon {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
            z: parts.next().unwrap(),
        });
    }
}
impl Beacon {
    fn rotate(&self, i: i32) -> Beacon {

        return match i {
            0 => Beacon { x: self.x, y: self.y, z: self.z },
            1 => Beacon { x: self.y, y: self.z, z: self.x },
            2 => Beacon { x: self.z, y: self.x, z: self.y },
            3 => Beacon { x: -self.x, y: self.z, z: self.y },
            4 => Beacon { x: self.z, y: self.y, z: -self.x },
            5 => Beacon { x: self.y, y: -self.x, z: self.z },
            6 => Beacon { x: self.x, y: self.z, z: -self.y },
            7 => Beacon { x: self.z, y: -self.y, z: self.x },
            8 => Beacon { x: -self.y, y: self.x, z: self.z },
            9 => Beacon { x: self.x, y: -self.z, z: self.y },
            10 => Beacon { x: -self.z, y: self.y, z: self.x },
            11 => Beacon { x: self.y, y: self.x, z: -self.z },
            12 => Beacon { x: -self.x, y: -self.y, z: self.z },
            13 => Beacon { x: -self.y, y: self.z, z: -self.x },
            14 => Beacon { x: self.z, y: -self.x, z: -self.y },
            15 => Beacon { x: -self.x, y: self.y, z: -self.z },
            16 => Beacon { x: self.y, y: -self.z, z: -self.x },
            17 => Beacon { x: -self.z, y: -self.x, z: self.y },
            18 => Beacon { x: self.x, y: -self.y, z: -self.z },
            19 => Beacon { x: -self.y, y: -self.z, z: self.x },
            20 => Beacon { x: -self.z, y: self.x, z: -self.y },
            21 => Beacon { x: -self.x, y: -self.z, z: -self.y },
            22 => Beacon { x: -self.z, y: -self.y, z: -self.x },
            23 => Beacon { x: -self.y, y: -self.x, z: -self.z },
            _ => panic!("Invalid rotation {}", i),
        };
    }

    fn distance(&self, other: &Beacon) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs();
    }
}

#[derive(Debug)]
struct Scanner {
    position: Beacon,
    detected_beacons: Vec<Beacon>,
}
impl std::str::FromStr for Scanner {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let detected_beacons = input.lines().skip(1).map(|line| line.parse::<Beacon>().unwrap()).collect::<Vec<Beacon>>();
        return Ok(Scanner { position: Beacon { x: 0, y: 0, z: 0}, detected_beacons });
    }
}
impl Scanner {
    fn find_transformation(&self, other: &Scanner) -> Option<Transformation> {
        for rotation in 0..24 {
            let mut distances: HashMap<(i32, i32, i32), i32> = HashMap::new();
            for beacon in &self.detected_beacons {
                let rotated = beacon.rotate(rotation);
                for other_beacon in &other.detected_beacons {
                    let distance = (other_beacon.x - rotated.x, other_beacon.y - rotated.y, other_beacon.z - rotated.z);
                    *distances.entry(distance).or_insert(0) +=1;
                    if distances[&distance] == 12 {
                        return Some((rotation, distance));
                    }
                }
            }
        }
        return None;
    }

    fn transform(&mut self, transformation: &Transformation) {
        let rotation = self.position.rotate(transformation.0);
        self.position.x = rotation.x + transformation.1.0;
        self.position.y = rotation.y + transformation.1.1;
        self.position.z = rotation.z + transformation.1.2;
        for beacon in &mut self.detected_beacons {
            let rotated = &beacon.rotate(transformation.0);
            beacon.x = rotated.x + transformation.1.0;
            beacon.y = rotated.y + transformation.1.1;
            beacon.z = rotated.z + transformation.1.2;
        }
    }

    fn get_transformation(transformations: &mut HashMap<(i32, i32), Vec<Transformation>>, scanners: &Vec<Scanner>, from: i32, to: i32, seen: &mut HashSet<(i32, i32)>) -> Vec<Transformation> {
        if transformations.contains_key(&(from, to)) {
            return transformations[&(from, to)].clone();
        }
        if seen.contains(&(from, to)) {
            return vec![];
        }
        seen.insert((from, to));
        let scanner1 = &scanners[from as usize];
        if let Some(transformation) = scanner1.find_transformation(&scanners[to as usize]) {
            transformations.insert((from, to), vec![transformation]);
            return transformations[&(from, to)].clone();
        }

        for index in 0..scanners.len() as i32 {
            if from == index as i32 || to == index as i32 {
                continue;
            }
            let transformation1 = Scanner::get_transformation(transformations, scanners, from, index as i32, seen);
            let transformation2 = Scanner::get_transformation(transformations, scanners, index as i32, to, seen);
            if transformation1.len() > 0 && transformation2.len() > 0 {
                let mut vec = vec![];
                vec.extend(transformation1);
                vec.extend(transformation2);
                transformations.insert((from, to), vec);
                return transformations[&(from, to)].clone();
            }
        }


        return vec![];
    }

    fn transform_all(scanners: &mut Vec<Scanner>) -> HashMap<(i32, i32), Vec<Transformation>>{
        let mut transformations: HashMap<(i32, i32), Vec<Transformation>> = HashMap::new();
        transformations.insert((0, 0), vec![(0, (0, 0, 0))]);
        for (id, _) in scanners.iter().enumerate() {
             Scanner::get_transformation(&mut transformations, scanners, id as i32, 0, &mut HashSet::new());
        }
        for (id, scanner) in scanners.iter_mut().enumerate() {
            transformations[&(id as i32, 0)].iter().for_each(|t| scanner.transform(t));
        }
        return transformations;
    }
}

pub fn run() -> (u64, u64) {
    let mut scanners = INPUT.split("\n\n").map(|line| line.parse::<Scanner>().unwrap()).collect::<Vec<Scanner>>();
    Scanner::transform_all(&mut scanners);
    let all_beacons: HashSet<Beacon> = scanners.iter().flat_map(|scanner| scanner.detected_beacons.clone()).collect();

    let max_distance = scanners.iter().map(|scanner| scanners.iter().map(|scanner2| scanner2.position.distance(&scanner.position)).max().unwrap()).max().unwrap();

    return (all_beacons.len() as u64, max_distance as u64);
}