use std::fs;

fn main() {
    let data = read_file("src/input.txt");
    let games = Game::parse(data.lines());

    let sum_of_ids = Game::possible_games(&games, 12, 13, 14)
        .iter()
        .map(|g| g.id)
        .sum::<i32>();
    println!("Part 1: The sum of the ids is: {sum_of_ids}");
}

struct Game {
    id: i32,
    cubes: Vec<Color>,
}

enum Color {
    Red(i32),
    Green(i32),
    Blue(i32),
}

impl Color {
    pub fn parse(color: &str, count: i32) -> Color {
        match color {
            "red" => Color::Red(count),
            "green" => Color::Green(count),
            "blue" => Color::Blue(count),
            _ => panic!("Invalid color!"),
        }
    }
}

impl Game {
    pub fn parse(lines: std::str::Lines<'_>) -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();

        for line in lines {
            let data: Vec<_> = line.split(":").collect();
            let id_text: Vec<_> = data[0].split_whitespace().collect();
            let id: i32 = id_text[1].parse().unwrap();

            let mut game = Game {
                id,
                cubes: Vec::new(),
            };

            let game_text: Vec<_> = data[1].split(";").collect();

            for games in game_text {
                let cubes_text: Vec<_> = games.split(",").collect();
                for cube_info in cubes_text {
                    let cube: Vec<_> = cube_info.split_whitespace().collect();
                    game.cubes
                        .push(Color::parse(cube[1], cube[0].parse().unwrap()));
                }
            }
            games.push(game);
        }

        games
    }

    pub fn possible_games(games: &Vec<Game>, red: i32, green: i32, blue: i32) -> Vec<&Game> {
        games.iter().filter(|g| g.contains(red, green, blue)).collect()
    }

    pub fn contains(&self, red: i32, green: i32, blue: i32) -> bool {
        self.red_cubes() <= red && self.green_cubes() <= green && self.blue_cubes() <= blue
    }

    pub fn red_cubes(&self) -> i32 {
        *self
            .cubes
            .iter()
            .map(|cube| match cube {
                Color::Red(c) => c,
                _ => &0,
            })
            .max()
            .unwrap()
    }

    pub fn green_cubes(&self) -> i32 {
        *self
            .cubes
            .iter()
            .map(|cube| match cube {
                Color::Green(c) => c,
                _ => &0,
            })
            .max()
            .unwrap()
    }

    pub fn blue_cubes(&self) -> i32 {
        *self
            .cubes
            .iter()
            .map(|cube| match cube {
                Color::Blue(c) => c,
                _ => &0,
            })
            .max()
            .unwrap()
    }
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file!")
}

#[cfg(test)]
mod test {
    use crate::{read_file, Game};

    #[test]
    fn part1() {
        let data = read_file("src/test01.txt");
        let sum_of_ids = Game::possible_games(&Game::parse(data.lines()), 12, 13, 14)
            .iter()
            .map(|g| g.id)
            .sum::<i32>();
        assert_eq!(sum_of_ids, 8);
    }
}
