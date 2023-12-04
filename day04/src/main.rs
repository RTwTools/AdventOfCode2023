use std::fs;

fn main() {
    let data = read_file("src/input.txt");
    let game = CardGame::parse(data.lines());

    println!("Part 1: The cards are worth: {}", game.value());
    println!("Part 2: The number of cards is: {}", game.count_cards());
}

struct Card {
    winners: Vec<i32>,
    numbers: Vec<i32>,
}

impl Card {
    pub fn parse(data: &str) -> Card {
        let data_items: Vec<_> = data.split("|").collect();
        Card {
            winners: Self::parse_numbers(data_items[0]),
            numbers: Self::parse_numbers(data_items[1]),
        }
    }

    fn parse_numbers(data: &str) -> Vec<i32> {
        data.split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect()
    }

    pub fn winning_count(&self) -> i32 {
        self.numbers
            .iter()
            .filter(|n| self.winners.contains(n))
            .count() as i32
    }

    pub fn value(&self) -> i32 {
        let count = self.winning_count();

        match count {
            0 => 0,
            _ => i32::pow(2, (count - 1) as u32),
        }
    }
}

struct CardGame {
    cards: Vec<Card>,
}

impl CardGame {
    pub fn parse(lines: std::str::Lines<'_>) -> CardGame {
        CardGame {
            cards: lines
                .map(|l| Card::parse(l.split(":").collect::<Vec<_>>()[1]))
                .collect(),
        }
    }

    pub fn value(&self) -> i32 {
        self.cards.iter().map(|c| c.value()).sum()
    }

    fn count_cards(&self) -> i32 {
        let mut copies = vec![1; self.cards.len()];

        for (i, card) in self.cards.iter().enumerate() {
            for c in 0..card.winning_count() {
                copies[i + 1 + c as usize] += copies[i];
            }
        }

        copies.iter().sum()
    }
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file!")
}

#[cfg(test)]
mod test {
    use crate::{read_file, CardGame};

    #[test]
    fn part1() {
        let data = read_file("src/test01.txt");
        let game = CardGame::parse(data.lines());

        assert_eq!(game.value(), 13);
    }

    #[test]
    fn part2() {
        let data = read_file("src/test01.txt");
        let game = CardGame::parse(data.lines());

        assert_eq!(game.count_cards(), 30);
    }
}
