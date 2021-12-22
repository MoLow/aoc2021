use std::{collections::HashMap, borrow::BorrowMut};

static INPUT: &str = include_str!("./input.txt");

struct Dice {
    max: i32,
    counter: i32,
}
impl Iterator for Dice {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        return Some(self.counter % self.max);
    }
}
impl Dice {
    fn new(max: i32) -> Dice {
        return Dice { max, counter: 0 };
    }
}


struct DiracDice {
    items: Vec<i32>,
}
impl Iterator for DiracDice {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        return self.items.pop();
    }
}
impl DiracDice {
    fn new() -> DiracDice {
        let mut items = Vec::new();
        for x in (1..4).rev() {
            for y in (1..4).rev() {
                for z in (1..4).rev() {
                    items.push(x + y + z);
                }
            }
        }
        return DiracDice { items };
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Player {
    position: i32,
    score: i32,
}
impl Player {
    fn new(position: i32) -> Player {
        return Player { position, score: 0 };
    }

    fn step(&mut self, steps: i32)  -> i32 {
        self.position = (self.position + steps) % 10;
        self.score += match self.position {
            0 => 10,
            x => x,
        };
        return self.score;
    }
}


struct Game;
impl Game {
    fn parse_input(input: &str) -> [Player; 2] {
        let positions = input.lines()
            .map(|line| line.split("starting position: ").nth(1).unwrap().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        return [Player::new(positions[0]), Player::new(positions[1])];
    }

    fn part1(mut players: [Player; 2]) -> u64 {
        let mut dice = Dice::new(100);
        loop {
            for player in players.iter_mut() {
                if player.step(dice.borrow_mut().take(3).sum::<i32>()) >= 1000 {
                    let looser = players.iter().min_by_key(|player| player.score).unwrap();
                    return (looser.score * dice.counter) as u64;
                }
            }
        }
    }

    fn play(players: [Player; 2], cache: &mut HashMap<[Player; 2], [u64; 2]>) -> [u64; 2] {
        if players[0].score >= 21 {
            return [1, 0];
        }
        if players[1].score >= 21 {
            return [0, 1];
        }

        if let Some(result) = cache.get(&players) {
            return *result;
        }

        let mut total_wins = [0, 0];
        for roll in DiracDice::new() {
            let mut p0 = players[0].clone();
            p0.step(roll);
            let next = Game::play([players[1].clone(), p0], cache);
            total_wins[0] += next[1];
            total_wins[1] += next[0];
        }
        cache.insert(players, total_wins);

        return total_wins;
    }

    fn part2(players: [Player; 2]) -> u64 {
        return *Game::play(players, &mut HashMap::new()).iter().max().unwrap();
    }
}

pub fn run() -> (u64, u64) {
    let part1 = Game::part1(Game::parse_input(INPUT));
    let part2 = Game::part2(Game::parse_input(INPUT));

    return (part1, part2);
}