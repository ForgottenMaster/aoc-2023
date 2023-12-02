type IntegerType = u16;

const INPUT: &str = include_str!("../../data/day01.txt");
const NUMBER_PATTERNS: &[(&str, IntegerType)] = &[
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];
const WORD_PATTERNS: &[(&str, IntegerType)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    println!("Part 1: {}", solve_part_1(INPUT));
    println!("Part 2: {}", solve_part_2(INPUT));
}

fn solve_part_1(input: &str) -> IntegerType {
    process_lines(input, NUMBER_PATTERNS.iter())
}

fn solve_part_2(input: &str) -> IntegerType {
    process_lines(input, NUMBER_PATTERNS.iter().chain(WORD_PATTERNS))
}

fn process_line<'a>(
    input: &str,
    patterns: impl Iterator<Item = &'a (&'a str, IntegerType)>,
) -> IntegerType {
    let mut leftmost_index = usize::MAX;
    let mut leftmost_value = 0;
    let mut rightmost_index = usize::MIN;
    let mut rightmost_value = 0;

    for (pattern, pattern_value) in patterns {
        if let Some(index) = input.find(pattern) {
            if index < leftmost_index {
                leftmost_index = index;
                leftmost_value = *pattern_value;
            }
        }

        if let Some(index) = input.rfind(pattern) {
            let index = index + pattern.len();
            if index > rightmost_index {
                rightmost_index = index;
                rightmost_value = *pattern_value;
            }
        }
    }

    leftmost_value * 10 + rightmost_value
}

fn process_lines<'a>(
    input: &str,
    patterns: impl Iterator<Item = &'a (&'a str, IntegerType)> + Clone,
) -> IntegerType {
    input
        .trim()
        .lines()
        .map(|line| process_line(line, patterns.clone()))
        .sum::<IntegerType>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        const INPUT: &str = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ";
        const EXPECTED: IntegerType = 142;
        let output = solve_part_1(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_2() {
        const INPUT: &str = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        ";
        const EXPECTED: IntegerType = 281;
        let output = solve_part_2(INPUT);
        assert_eq!(output, EXPECTED);
    }
}
