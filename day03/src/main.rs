use std::{
    cmp::{max, min},
    fs,
    ops::Range,
};

use regex::Regex;

fn main() {
    let data = read_file("src/input.txt");
    let schematic = Schematic::parse(data.lines().collect());
    let sum_of_parts: i32 = schematic.parts().iter().map(|c| c.id).sum();
    println!("Part 1: The sum of the parts is: {sum_of_parts}");

    let sum_of_ratios: i32 = schematic.gear_ratios().iter().sum();
    println!("Part 2: The sum of the ratios is: {sum_of_ratios}");
}

struct Schematic<'a> {
    data: Vec<&'a str>,
    components: Vec<Component>,
}

pub fn overlaps(r1: &Range<usize>, r2: &Range<usize>) -> bool {
    r1.start <= r2.end - 1 && r2.start <= r1.end - 1
}

impl Schematic<'_> {
    pub fn parse(data: Vec<&str>) -> Schematic {
        let mut schematic = Schematic {
            data: data.clone(),
            components: Vec::new(),
        };

        for (line, text) in data.iter().enumerate() {
            let reg = Regex::new(r"(\d+)").unwrap();

            for item in reg.find_iter(text) {
                let range = item.range();
                let id = item.as_str().parse::<i32>().unwrap();

                schematic.components.push(Component { id, line, range });
            }
        }

        schematic
    }

    pub fn parts(&self) -> Vec<&Component> {
        self.components
            .iter()
            .filter(|c| self.is_part_number(c))
            .collect()
    }

    pub fn gear_ratios(&self) -> Vec<i32> {
        let mut ratios = Vec::new();
        for (line, text) in self.data.iter().enumerate() {
            let reg = Regex::new(r"(\*)").unwrap();

            for gear in reg.find_iter(text) {
                let range = max(gear.range().start as i32 - 1, 0) as usize
                    ..min(gear.range().end + 1, self.data[0].len() - 1);
                let lines = max(line as i32 - 1, 0) as usize..min(line + 2, self.data.len());
                let adjacent_components: Vec<&Component> = self
                    .components
                    .iter()
                    .filter(|c| lines.contains(&c.line) && overlaps(&range, &c.range))
                    .collect();

                if adjacent_components.len() == 2 {
                    ratios.push(adjacent_components.iter().map(|c| c.id).product());
                }
            }
        }

        ratios
    }

    pub fn is_part_number(&self, component: &&Component) -> bool {
        let symbols = ['+', '$', '#', '*', '=', '/', '%', '@', '&', '-'];

        let start = max(component.range.start as i32 - 1, 0) as usize;
        let end = min(component.range.end + 1, self.data[0].len() - 1);

        if component.line != 0 {
            let top_line = &(self.data[(component.line - 1) as usize])[start..end];
            if top_line.contains(symbols) {
                return true;
            }
        }

        if self.data[component.line][start..start + 1].contains(symbols)
            || self.data[component.line][end - 1..end].contains(symbols)
        {
            return true;
        }

        if component.line != (self.data.len() - 1) {
            let bottom_line = &(self.data[(component.line + 1) as usize])[start..end];
            if bottom_line.contains(symbols) {
                return true;
            }
        }

        false
    }
}

struct Component {
    id: i32,
    line: usize,
    range: Range<usize>,
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file!")
}

#[cfg(test)]
mod test {
    use crate::{read_file, Schematic};

    #[test]
    fn part1() {
        let data = read_file("src/test01.txt");

        let schematic = Schematic::parse(data.lines().collect());
        let sum_of_parts: i32 = schematic.parts().iter().map(|c| c.id).sum();
        assert_eq!(sum_of_parts, 4361);
    }

    #[test]
    fn part2() {
        let data = read_file("src/test01.txt");

        let schematic = Schematic::parse(data.lines().collect());
        let sum_of_ratios: i32 = schematic.gear_ratios().iter().sum();
        assert_eq!(sum_of_ratios, 467835);
    }
}
