use std::ops::Range;

type Destination = (Range<i32>, Range<i32>);

#[derive(Debug)]
struct Probe {
    velocity: (i32, i32),
    position: (i32, i32),
}
impl Probe {
    fn new(velocity: (i32, i32)) -> Probe {
        return Probe {
            velocity,
            position: (0, 0),
        }
    }
    fn tick(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        self.velocity.0 -= self.velocity.0.signum(); 
        self.velocity.1 -= 1;
    }

    fn tick_until_in_destination(&mut self, destination: &Destination) -> Option<i32> {
        let mut max_y = self.position.1;

        loop {
            self.tick();
            if self.position.0 > destination.0.end || self.position.1 < destination.1.start {
                return None;
            }
            if destination.0.contains(&self.position.0) && destination.1.contains(&self.position.1) {
                break;
            }
            max_y = self.position.1.max(max_y);
        }

        return Some(max_y);
    }

    fn find_highest_velocity(destination: &Destination) -> i32 {
        let range = destination.1.start.abs();
        let mut max_y = 0;
        for x in 1..destination.0.end {
            for y in -range..range {
                let mut probe = Probe::new((x, y));
                let height = probe.tick_until_in_destination(destination).unwrap_or(0);
                if height > max_y {
                    max_y = height;
                }
            }
        }

        return max_y;
    }

    fn count_valid_velocities(destination: &Destination) -> i32 {
        let mut count = 0;
        for x in 1..destination.0.end {
            for y in -100..100 {
                let mut probe = Probe::new((x, y));
                if probe.tick_until_in_destination(destination).is_some() {
                    // println!("{:?}", (x, y));
                    count += 1;
                }
            }
        }

        return count;
    }
}

fn parse_input(input: &str) -> Destination {
    let x = input.split(", ").nth(0).unwrap().split("target area: x=").nth(1).unwrap().split("..").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let y = input.split(", ").nth(1).unwrap().split("y=").nth(1).unwrap().split("..").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    return ((x[0]..x[1]+1), (y[0]..y[1]+1));
}

static INPUT: &str = include_str!("./input.txt");
pub fn run() -> (u64, u64) { 
    let target = parse_input(INPUT);
    let part1 = Probe::find_highest_velocity(&target) as u64;
    let part2 = Probe::count_valid_velocities(&target) as u64;

    return (part1, part2);
}