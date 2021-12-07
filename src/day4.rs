use std::fmt::Display;

#[derive(PartialEq, Clone, Debug)]
enum BingoSpot {
    Marked(usize),
    Unmarked(usize),
}

impl Display for BingoSpot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BingoSpot::Marked(inner) => write!(f, "M{inner} "),
            BingoSpot::Unmarked(inner) => write!(f, "U{inner} "),
        }
    }
}

impl BingoSpot {
    fn mark(&self) -> Self {
        match self {
            BingoSpot::Marked(inner) => Self::Marked(*inner),
            BingoSpot::Unmarked(inner) => Self::Marked(*inner),
        }
    }

    fn is_marked(&self) -> bool {
        match self {
            BingoSpot::Marked(_) => true,
            BingoSpot::Unmarked(_) => false,
        }
    }

    fn get_num(&self) -> usize {
        match self {
            BingoSpot::Marked(inner) => *inner,
            BingoSpot::Unmarked(inner) => *inner,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct BingoBoard {
    spots: Vec<BingoSpot>,
    completed: bool,
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..25 {
            write!(f, "{}", self.spots[i])?;
            if i > 0 && (i + 1) % 5 == 0 {
                write!(f, "\n")?;
            }
            if i == 24 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl BingoBoard {
    fn mark_if_found(&mut self, number: usize) {
        if let Some((index, BingoSpot::Unmarked(_))) =
            self.spots.iter().enumerate().find(|(_, spot)| match spot {
                BingoSpot::Marked(_) => false,
                BingoSpot::Unmarked(inner) => *inner == number,
            })
        {
            self.mark_spot(index);
        }
    }

    fn mark_spot(&mut self, index: usize) {
        self.spots[index] = self.spots[index].mark()
    }

    fn has_won(&self) -> bool {
        // vertical
        for i in 0..5 {
            let mut marked = 0;
            for j in 0..5 {
                let spot = &self.spots[i + 5 * j];
                if spot.is_marked() {
                    marked += 1;
                }
            }
            if marked == 5 {
                return true;
            }
        }

        // horizontal
        for i in 0..5 {
            let mut marked = 0;
            for j in 0..5 {
                let spot = &self.spots[i * 5 + j];
                let is_marked = spot.is_marked();
                if is_marked {
                    marked += 1;
                }
            }
            if marked == 5 {
                return true;
            }
        }

        false
    }

    fn complete(&mut self) {
        self.completed = true;
    }

    fn score(&self, current_number: usize) -> usize {
        let s = self
            .spots
            .iter()
            .filter(|spot| !spot.is_marked())
            .map(|spot| spot.get_num())
            .sum::<usize>();
        s * current_number
    }
}

#[derive(Clone, Debug)]
struct BingoGame {
    numbers: Vec<usize>,
    number_index: usize,
    boards: Vec<BingoBoard>,
    last_completed: Option<usize>,
}

impl BingoGame {
    fn play_next(&mut self) {
        let played_number = self.numbers[self.number_index];
        self.number_index += 1;

        for i in 0..self.boards.len() {
            self.boards[i].mark_if_found(played_number)
        }
    }

    fn get_winning_board(&self) -> Option<&BingoBoard> {
        self.boards.iter().find(|&board| board.has_won())
    }

    // fn com(&mut self) {
    //     self.boards = self
    //         .boards
    //         .clone()
    //         .into_iter()
    //         .filter(|board| !board.has_won())
    //         .collect();
    // }

    fn complete_boards(&mut self) {
        for i in 0..self.boards.len() {
            let board = &mut self.boards[i];
            if !board.completed && board.has_won() {
                board.complete();
                self.last_completed = Some(i);
            }
        }
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> BingoGame {
    let numbers = input
        .clone()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let boards: Vec<BingoBoard> = input
        .split("\n\n")
        .skip(1)
        .map(|board_str| board_str.replace('\n', " "))
        .collect::<Vec<String>>()
        .iter()
        .map(|board_str| {
            let spots = board_str
                .split_whitespace()
                .map(|number| BingoSpot::Unmarked(number.parse().unwrap()))
                .collect();
            BingoBoard {
                spots,
                completed: false,
            }
        })
        .collect();

    BingoGame {
        numbers,
        number_index: 0,
        boards,
        last_completed: None,
    }
}

#[aoc(day4, part1)]
fn part1(game: &BingoGame) -> usize {
    let mut real_game = game.clone();
    let mut winning_board: Option<&BingoBoard> = None;

    while winning_board == None {
        real_game.play_next();
        winning_board = real_game.get_winning_board();
    }

    // we have a winning board by now
    winning_board
        .unwrap()
        .score(real_game.numbers[real_game.number_index - 1])
}

#[aoc(day4, part2)]
fn part2(game: &BingoGame) -> usize {
    let mut real_game = game.clone();
    let mut all_boards_completed = false;

    while !all_boards_completed {
        real_game.play_next();
        real_game.complete_boards();
        all_boards_completed = real_game.boards.iter().all(|board| board.completed);
    }

    let last_completed = real_game.last_completed.unwrap();
    let last_completed_board = &real_game.boards[last_completed];
    println!("{last_completed_board}");
    last_completed_board.score(real_game.numbers[real_game.number_index - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 4512)
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), 1924)
    }
}
