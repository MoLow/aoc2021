#[derive(Debug)]
#[derive(Clone)]
struct BingoCard {
    number: i32,
    is_marked: bool,
}

#[derive(Debug)]
#[derive(Clone)]
struct BingoBoard {
    cards: Vec<Vec<BingoCard>>,
}
impl BingoBoard {
    fn new(lines: &[&str]) -> BingoBoard {
        let mut cards = Vec::new();
        for line in lines {
            let mut card = Vec::new();
            for num in line.split_whitespace() {
                card.push(BingoCard {
                    number: num.parse().unwrap(),
                    is_marked: false,
                });
            }
            cards.push(card);
        }
        BingoBoard { cards }
    }

    fn rest(&mut self) {
        for card in &mut self.cards {
            for bingo in card {
                bingo.is_marked = false;
            }
        }
    }

    fn mark(&mut self, num: i32) {
        for card in &mut self.cards {
            for bingo in card {
                if bingo.number == num {
                    bingo.is_marked = true;
                }
            }
        }
    }

    fn score (&self) -> i32 {
        let mut score = 0;
        for line in &self.cards {
            for card in line {
                if !card.is_marked {
                    score += card.number;
                }
            }
        }
        score
    }

    fn has_bingo (&self) -> bool {
        let line_length = self.cards[0].len();
        let lines = self.cards.iter().any(|line| line.iter().all(|card| card.is_marked));
        let rows = (0..line_length).any(|row| {
            return self.cards.iter().all(|line| line[row].is_marked);
        });

        return lines || rows;
    }
}

fn get_winning_board(boards: &mut Vec<BingoBoard>, numbers: &Vec<i32>) -> Option<(BingoBoard, i32, usize)> {
    for num in numbers {
        for (index, board) in (&mut *boards).iter_mut().enumerate() {
            board.mark(*num);
            if board.has_bingo() {
                return Some((board.clone(), *num, index));
            }
        }
    }
    return None;
}

fn get_last_winning_board(boards: &mut Vec<BingoBoard>, numbers: &Vec<i32>) -> Option<(BingoBoard, i32)> {
    let mut last_win: Option<(BingoBoard, i32)> = None;
    boards.iter_mut().for_each(|board| board.rest());
    
    while boards.len() > 0 {
        let win = get_winning_board(boards, numbers);
        if win.is_some() {
            let (board, num, index) = win.unwrap();
            last_win = Some((board, num));
            boards.remove(index);
        }
    }

    return last_win;
}

fn parse_input(input: &str) -> (Vec<i32>,Vec<BingoBoard>) {
    let lines = input.lines().filter(|line| !line.is_empty()).collect::<Vec<&str>>();
    let numbers = lines[0].split(',').map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let boards = lines[1..]
        .chunks(5)
        .map(|board| BingoBoard::new(board))
        .collect::<Vec<BingoBoard>>();

    return (numbers, boards);
}

pub fn run(input: String) {
    let (numbers,mut boards) = parse_input(&input);
    let winner = get_winning_board(&mut boards, &numbers);

    if winner.is_some() {
        let ( board, num, ..) = winner.unwrap();
        let score = board.score();
        println!("first winner score {} X num {} = {}", score, num, score * num);
    }


    let last_winner = get_last_winning_board(&mut boards, &numbers);
    if last_winner.is_some() {
        let ( board, num) = last_winner.unwrap();
        let score = board.score();
        println!("last winner score {} X num {} = {}", score, num, score * num);
    }
}