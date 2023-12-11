use std::collections::HashMap;

const INPUT: &str = include_str!("../../data/day08.txt");

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

fn main() {
    println!("Part 1 => {}", solve_part_1(INPUT));
    println!("Part 2 => {}", solve_part_2(INPUT));
}

fn solve_part_1(input: &str) -> u32 {
    let (mut sequence, mapping) = parse_input(input);
    calculate_steps("AAA", |current| current == "ZZZ", &mut sequence, &mapping)
}

fn solve_part_2(_input: &str) -> u32 {
    todo!()
}

fn calculate_steps(
    from: &str,
    terminates: impl Fn(&str) -> bool,
    sequence: &mut impl Iterator<Item = Direction>,
    mapping: &HashMap<&str, (&str, &str)>,
) -> u32 {
    let mut count = 0;
    let mut current = from;
    while !terminates(current) {
        current = match sequence.next().unwrap() {
            Direction::Left => mapping[current].0,
            Direction::Right => mapping[current].1,
        };
        count += 1;
    }
    count
}

fn parse_move_sequence(input: &str) -> impl Iterator<Item = Direction> + '_ {
    input
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unimplemented!("Invalid character {c} found in input sequence"),
        })
        .cycle()
}

fn parse_move(input: &str) -> (&str, &str, &str) {
    let (label, input) = input.split_once(" = (").unwrap();
    let input = input.strip_suffix(')').unwrap();
    let (left, right) = input.split_once(", ").unwrap();
    (label, left, right)
}

fn parse_input(
    input: &str,
) -> (
    impl Iterator<Item = Direction> + '_,
    HashMap<&str, (&str, &str)>,
) {
    let mut lines = input.lines().filter_map(|line| {
        let line = line.trim();
        if line.is_empty() {
            None
        } else {
            Some(line)
        }
    });
    let sequence = parse_move_sequence(lines.next().unwrap());
    let mut hm = HashMap::new();
    for line in lines {
        let (label, left, right) = parse_move(line);
        hm.insert(label, (left, right));
    }
    (sequence, hm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_move_sequence() {
        const INPUT: &str = "LLR";
        const EXPECTED: &[Direction] = &[
            Direction::Left,
            Direction::Left,
            Direction::Right,
            Direction::Left,
            Direction::Left,
            Direction::Right,
        ];
        let output: Vec<_> = parse_move_sequence(INPUT).take(6).collect();
        assert_eq!(&output, EXPECTED);
    }

    #[test]
    fn test_parse_move() {
        const INPUT: &str = "CCC = (ZZZ, GGG)";
        const EXPECTED: (&str, &str, &str) = ("CCC", "ZZZ", "GGG");
        let output = parse_move(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_input() {
        const INPUT: &str = "
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        ";
        const EXPECTED_SEQUENCE: &[Direction] =
            &[Direction::Left, Direction::Left, Direction::Right];
        const EXPECTED_MAPPING: &[(&str, (&str, &str))] = &[
            ("AAA", ("BBB", "BBB")),
            ("BBB", ("AAA", "ZZZ")),
            ("ZZZ", ("ZZZ", "ZZZ")),
        ];
        let (sequence, mapping) = parse_input(INPUT);
        let sequence: Vec<_> = sequence.take(3).collect();
        let mut mapping: Vec<_> = mapping.into_iter().collect();
        mapping.sort();
        assert_eq!(&sequence, EXPECTED_SEQUENCE);
        assert_eq!(&mapping, EXPECTED_MAPPING);
    }

    #[test]
    fn test_solve_part_1_case_1() {
        const INPUT: &str = "
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        ";
        const EXPECTED: u32 = 2;
        let output = solve_part_1(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_1_case_2() {
        const INPUT: &str = "
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        ";
        const EXPECTED: u32 = 6;
        let output = solve_part_1(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_2() {
        const INPUT: &str = "
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
        ";
        const EXPECTED: u32 = 6;
        let output = solve_part_2(INPUT);
        assert_eq!(output, EXPECTED);
    }
}
