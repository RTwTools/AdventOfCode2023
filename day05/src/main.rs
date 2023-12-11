use std::fs;

fn main() {
    let file = read_file("src/input.txt");
    let alamnac = Alamnac::parse(file.lines().collect::<Vec<&str>>());

    let location = alamnac
        .seeds
        .iter()
        .map(|s| alamnac.to_location(s))
        .min()
        .unwrap();

    println!("Part 1: The closest location is: {}", location);
}

struct Alamnac {
    seeds: Vec<u64>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Alamnac {
    pub fn parse(lines: Vec<&str>) -> Alamnac {
        let items = lines.split(|s| s.is_empty()).collect::<Vec<_>>();

        Alamnac {
            seeds: items[0][0].split(":").collect::<Vec<_>>()[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>(),
            seed_to_soil: Self::parse_map(items[1]),
            soil_to_fertilizer: Self::parse_map(items[2]),
            fertilizer_to_water: Self::parse_map(items[3]),
            water_to_light: Self::parse_map(items[4]),
            light_to_temperature: Self::parse_map(items[5]),
            temperature_to_humidity: Self::parse_map(items[6]),
            humidity_to_location: Self::parse_map(items[7]),
        }
    }

    fn parse_map(list: &[&str]) -> Map {
        Map::parse(
            list.iter()
                .skip(1)
                .map(|i| {
                    i.split_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect()
                })
                .collect::<Vec<_>>(),
        )
    }

    fn to_soil(&self, seed: &u64) -> u64 {
        self.seed_to_soil.map(seed)
    }

    fn to_fertilizer(&self, seed: &u64) -> u64 {
        self.soil_to_fertilizer.map(&self.to_soil(seed))
    }

    fn to_water(&self, seed: &u64) -> u64 {
        self.fertilizer_to_water.map(&self.to_fertilizer(seed))
    }

    fn to_light(&self, seed: &u64) -> u64 {
        self.water_to_light.map(&self.to_water(seed))
    }

    fn to_temperature(&self, seed: &u64) -> u64 {
        self.light_to_temperature.map(&self.to_light(seed))
    }

    fn to_humidity(&self, seed: &u64) -> u64 {
        self.temperature_to_humidity.map(&self.to_temperature(seed))
    }

    fn to_location(&self, seed: &u64) -> u64 {
        self.humidity_to_location.map(&self.to_humidity(seed))
    }
}

struct Item {
    from: u64,
    to: u64,
    count: u64,
}

impl Item {
    pub fn parse(items: &Vec<u64>) -> Item {
        Item {
            from: items[1],
            to: items[0],
            count: items[2],
        }
    }

    pub fn contains(&self, item: &u64) -> bool {
        item >= &self.from && item < &(&self.from + &self.count)
    }

    pub fn map(&self, item: &u64) -> u64 {
        match self.contains(item) {
            true => item - self.from + self.to,
            false => *item,
        }
    }
}

struct Map {
    items: Vec<Item>,
}

impl Map {
    pub fn parse(items: Vec<Vec<u64>>) -> Map {
        Map {
            items: items.iter().map(|i| Item::parse(i)).collect(),
        }
    }

    pub fn map(&self, item: &u64) -> u64 {
        let mut output = *item;
        for map in &self.items {
            if map.contains(&output) {
                output = map.map(&output);
                break;
            }
        }

        output
    }
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file!")
}

#[cfg(test)]
mod test {
    use crate::{read_file, Alamnac};

    #[test]
    fn part1() {
        let file = read_file("src/test01.txt");
        let alamnac = Alamnac::parse(file.lines().collect::<Vec<&str>>());
        let location = alamnac
            .seeds
            .iter()
            .map(|s| alamnac.to_location(s))
            .min()
            .unwrap();
        assert_eq!(location, 35);
    }
}
