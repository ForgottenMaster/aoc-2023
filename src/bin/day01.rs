type IntegerType = u32;

const INPUT: &str = include_str!("../../data/day01.txt");
const PATTERNS_PART_1: &[(&str, IntegerType)] = &[
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
const PATTERNS_PART_2: &[(&str, IntegerType)] = &[
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
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
    println!("Part 1: {}", process_lines(INPUT, PATTERNS_PART_1));
    println!("Part 2: {}", process_lines(INPUT, PATTERNS_PART_2));
}

fn process_line(input: &str, patterns: &[(&str, IntegerType)]) -> IntegerType {
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

fn process_lines(input: &str, patterns: &[(&str, IntegerType)]) -> IntegerType {
    input
        .trim()
        .lines()
        .map(|line| process_line(line, patterns))
        .sum::<IntegerType>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_part_1() {
        const INPUT: &str = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ";
        const EXPECTED: IntegerType = 142;
        let output = process_lines(INPUT, PATTERNS_PART_1);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_process_lines_part_2() {
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
        let output = process_lines(INPUT, PATTERNS_PART_2);
        assert_eq!(output, EXPECTED);
    }
}