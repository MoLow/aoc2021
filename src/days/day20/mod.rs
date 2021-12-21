use std::collections::HashMap;

static INPUT: &str = include_str!("./input.txt");

struct Image {
    algorithm: Vec<bool>,
    image: HashMap<(i64, i64), bool>,
    infinity: bool,
}
impl std::str::FromStr for Image {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split("\n\n");
        let algorithm: Vec<bool> = parts.next().unwrap().chars().map(|c| c == '#').collect();
        let image_lines = parts.next().unwrap().lines().collect::<Vec<&str>>();
        let image = image_lines.iter().enumerate()
            .map(|(y, line)| line.chars().enumerate().map(|(x, c)| ((x as i64, y as i64), c == '#')).collect::<Vec<((i64, i64), bool)>>())
            .flatten()
            .collect();
        

        return Ok(Image{ algorithm, image, infinity: false });
    }
}

impl Image {
    fn print(&self) {
        let (min_x, min_y) = *self.image.iter().min().unwrap().0;
        let (max_x, max_y) = *self.image.iter().max().unwrap().0;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match self.image.get(&(x, y)) {
                    Some(true) => print!("#"),
                    Some(false) => print!("."),
                    None => print!(" "),
                }
            }
            println!()
        }
        println!();
        println!();
    }

    fn get_neighbors(&(x, y): &(i64, i64)) -> Vec<(i64, i64)> {
        return vec![
            (x-1, y-1), (x, y-1),   (x+1, y-1),
            (x-1, y),   (x, y),     (x+1, y),
            (x-1, y+1), (x, y+1),   (x+1, y+1)
        ];
    }
    fn bool_vec_to_byte(bits: Vec<bool>) -> u16 {
        let mut byte = 0;
        for bit in bits {
            byte <<= 1;
            byte |= bit as u16;
        }
        return byte;
    }
    fn enhance(&mut self) {
        let (min_x, min_y) = *self.image.iter().min().unwrap().0;
        let (max_x, max_y) = *self.image.iter().max().unwrap().0;
        let mut new_image = HashMap::new();

        for y in min_y-1..=max_y+1 {
            for x in min_x-1..=max_x+1 {
                let pixel = (x, y);
                if new_image.contains_key(&pixel) {
                    continue;
                }
                let alg_index = Image::bool_vec_to_byte(Image::get_neighbors(&pixel).iter()
                    .map(|c| match self.image.get(c) { Some(x) => *x, None => self.infinity })
                    .collect::<Vec<bool>>());
                new_image.insert(pixel, self.algorithm[alg_index as usize]);
            }
        }
        self.infinity = match self.infinity {
            false => *self.algorithm.first().unwrap(),
            true => *self.algorithm.last().unwrap(),
        };
        self.image = new_image;
    }

    fn count_lights(&self) -> u64 {
        return self.image.values().filter(|x| **x).count() as u64;
    }
}

pub fn run() -> (u64, u64) {
    let mut image = INPUT.parse::<Image>().unwrap();
    let mut part1 = 0;
    for i in 0..50 {
        if i == 2 {
            part1 = image.count_lights();
        }
        image.enhance();
    }
    image.print();

    return (part1, image.count_lights());
}