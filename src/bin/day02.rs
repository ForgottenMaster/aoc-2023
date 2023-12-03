use core::cmp;
use core::convert::Infallible;
use core::str::FromStr;

const INPUT: &str = include_str!("../../data/day02.txt");

fn main() {
    println!("Part 1 => {}", solve_part_1(INPUT));
    println!("Part 2 => {}", solve_part_2(INPUT));
}

fn solve_part_1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let game: Game = line.trim().parse().unwrap();
            if !game.valid() {
                None
            } else {
                Some(game.id.0)
            }
        })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let game: Game = line.trim().parse().unwrap();
            game.power()
        })
        .sum()
}

#[derive(Debug, PartialEq)]
struct GameId(u32);

impl FromStr for GameId {
    type Err = Infallible;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let id = string.strip_prefix("Game ").unwrap().parse().unwrap();
        Ok(Self(id))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Rgb {
    r: u32,
    g: u32,
    b: u32,
}

impl Rgb {
    fn valid(&self) -> bool {
        const RED_COUNT: u32 = 12;
        const GREEN_COUNT: u32 = 13;
        const BLUE_COUNT: u32 = 14;
        self.r <= RED_COUNT && self.g <= GREEN_COUNT && self.b <= BLUE_COUNT
    }

    fn maximum(&self, other: &Self) -> Self {
        Self {
            r: cmp::max(self.r, other.r),
            g: cmp::max(self.g, other.g),
            b: cmp::max(self.b, other.b),
        }
    }
}

impl FromStr for Rgb {
    type Err = Infallible;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut value = Self::default();
        for part in string.split(", ") {
            let (number, color) = part.split_once(' ').unwrap();
            let number = number.parse().unwrap();
            match color {
                "red" => value.r = number,
                "green" => value.g = number,
                "blue" => value.b = number,
                _ => unimplemented!("Invalid color"),
            }
        }
        Ok(value)
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: GameId,
    rgbs: Vec<Rgb>,
}

impl Game {
    fn valid(&self) -> bool {
        for rgb in &self.rgbs {
            if !rgb.valid() {
                return false;
            }
        }
        true
    }

    fn power(&self) -> u32 {
        let Rgb { r, g, b } = self
            .rgbs
            .iter()
            .fold(Rgb::default(), |state, elem| state.maximum(elem));
        r * g * b
    }
}

impl FromStr for Game {
    type Err = Infallible;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (id, string) = string.split_once(": ").unwrap();
        let id = id.parse().unwrap();
        let mut rgbs = Vec::with_capacity(3);
        for rgb in string.split("; ") {
            rgbs.push(rgb.parse().unwrap());
        }
        Ok(Self { id, rgbs })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game_id() {
        const INPUT: &str = "Game 1";
        const EXPECTED: GameId = GameId(1);
        let output: GameId = INPUT.parse().unwrap();
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_rgb_blue_red() {
        const INPUT: &str = "3 blue, 4 red";
        const EXPECTED: Rgb = Rgb { r: 4, g: 0, b: 3 };
        let output: Rgb = INPUT.parse().unwrap();
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_rgb_red_green_blue() {
        const INPUT: &str = "1 red, 2 green, 6 blue";
        const EXPECTED: Rgb = Rgb { r: 1, g: 2, b: 6 };
        let output: Rgb = INPUT.parse().unwrap();
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_rgb_green() {
        const INPUT: &str = "2 green";
        const EXPECTED: Rgb = Rgb { r: 0, g: 2, b: 0 };
        let output: Rgb = INPUT.parse().unwrap();
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_game() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game {
            id: GameId(1),
            rgbs: vec![
                Rgb { r: 4, g: 0, b: 3 },
                Rgb { r: 1, g: 2, b: 6 },
                Rgb { r: 0, g: 2, b: 0 },
            ],
        };
        let output: Game = INPUT.parse().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_rgb_valid() {
        const INPUT: Rgb = Rgb { r: 10, g: 9, b: 8 };
        const EXPECTED: bool = true;
        let output = INPUT.valid();
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_rgb_not_valid() {
        const INPUT: Rgb = Rgb { r: 13, g: 9, b: 8 };
        const EXPECTED: bool = false;
        let output = INPUT.valid();
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_game_valid() {
        let input = Game {
            id: GameId(0),
            rgbs: vec![
                Rgb { r: 0, g: 2, b: 1 },
                Rgb { r: 1, g: 3, b: 4 },
                Rgb { r: 0, g: 1, b: 1 },
            ],
        };
        const EXPECTED: bool = true;
        let output = input.valid();
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_game_not_valid() {
        let input = Game {
            id: GameId(0),
            rgbs: vec![
                Rgb { r: 20, g: 8, b: 6 },
                Rgb { r: 4, g: 13, b: 5 },
                Rgb { r: 1, g: 5, b: 0 },
            ],
        };
        const EXPECTED: bool = false;
        let output = input.valid();
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_1() {
        const INPUT: &str = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
        const EXPECTED: u32 = 8;
        let output = solve_part_1(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_rgb_maximum() {
        const INPUT_1: Rgb = Rgb { r: 12, g: 8, b: 14 };
        const INPUT_2: Rgb = Rgb { r: 10, g: 17, b: 7 };
        const EXPECTED: Rgb = Rgb {
            r: 12,
            g: 17,
            b: 14,
        };
        let output = INPUT_1.maximum(&INPUT_2);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_game_power() {
        let input = Game {
            id: GameId(3),
            rgbs: vec![
                Rgb { r: 20, g: 8, b: 6 },
                Rgb { r: 4, g: 13, b: 5 },
                Rgb { r: 1, g: 5, b: 0 },
            ],
        };
        const EXPECTED: u32 = 1560;
        let output = input.power();
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_2() {
        const INPUT: &str = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
        const EXPECTED: u32 = 2286;
        let output = solve_part_2(INPUT);
        assert_eq!(output, EXPECTED);
    }
}
