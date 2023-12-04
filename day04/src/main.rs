use std::fs;

fn main() {
    let data = read_file("src/input.txt");
    let game = CardGame::parse(data.lines());

    println!("Part 1: The cards are worth: {}", game.value());
}

struct Card {
    id: i32,
    winners: Vec<i32>,
    numbers: Vec<i32>,
}

impl Card {
    pub fn parse(id: i32, data: &str) -> Card {
        let data_items: Vec<_> = data.split("|").collect();
        Card {
            id,
            winners: Self::parse_numbers(data_items[0]),
            numbers: Self::parse_numbers(data_items[1]),
        }
    }

    fn parse_numbers(data: &str) -> Vec<i32> {
        data.split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect()
    }

    pub fn value(&self) -> i32 {
        let count = self
            .numbers
            .iter()
            .filter(|n| self.winners.contains(n))
            .count() as i32;

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
        let mut game = CardGame { cards: Vec::new() };

        for line in lines {
            let data: Vec<_> = line.split(":").collect();
            let id_text: Vec<_> = data[0].split_whitespace().collect();
            let id: i32 = id_text[1].parse().unwrap();

            game.cards.push(Card::parse(id, data[1]));
        }

        game
    }

    pub fn value(&self) -> i32 {
        self.cards.iter().map(|c| c.value()).sum()
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
}
