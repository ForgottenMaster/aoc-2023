type IntegerType = i32;

const INPUT: &str = include_str!("../../data/day09.txt");

fn main() {
    println!("Part 1 => {}", solve_part_1(INPUT));
    println!("Part 2 => {}", solve_part_2(INPUT));
}

fn solve_part_1(input: &str) -> IntegerType {
    parse_histories(input)
        .iter()
        .map(|history| get_extrapolated_history(history))
        .sum()
}

fn solve_part_2(input: &str) -> IntegerType {
    parse_histories(input)
        .iter()
        .map(|history| get_backwards_extrapolated_history(history))
        .sum()
}

fn parse_history(input: &str) -> Vec<IntegerType> {
    input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn parse_histories(input: &str) -> Vec<Vec<IntegerType>> {
    input.trim().lines().map(parse_history).collect()
}

fn get_histories_differences(input: &[IntegerType]) -> Vec<IntegerType> {
    input
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

fn get_extrapolated_history(input: &[IntegerType]) -> IntegerType {
    if input.iter().all(|val| *val == 0) {
        0
    } else {
        input[input.len() - 1] + get_extrapolated_history(&get_histories_differences(input))
    }
}

fn get_backwards_extrapolated_history(input: &[IntegerType]) -> IntegerType {
    if input.iter().all(|val| *val == 0) {
        0
    } else {
        input[0] - get_backwards_extrapolated_history(&get_histories_differences(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_history() {
        const INPUT: &str = "0 3 6 9 12 15";
        const EXPECTED: &[IntegerType] = &[0, 3, 6, 9, 12, 15];
        let output = parse_history(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_histories() {
        const INPUT: &str = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        ";
        const EXPECTED: &[&[IntegerType]] = &[
            &[0, 3, 6, 9, 12, 15],
            &[1, 3, 6, 10, 15, 21],
            &[10, 13, 16, 21, 30, 45],
        ];
        let output = parse_histories(INPUT);
        let output_refs: Vec<_> = output.iter().collect();
        assert_eq!(&output_refs, EXPECTED);
    }

    #[test]
    fn test_get_histories_differences() {
        const INPUT: &[IntegerType] = &[1, 3, 6, 10, 15, 21];
        const EXPECTED: &[IntegerType] = &[2, 3, 4, 5, 6];
        let output = get_histories_differences(INPUT);
        assert_eq!(&output, EXPECTED);
    }

    #[test]
    fn test_get_extrapolated_history() {
        const INPUT: &[IntegerType] = &[10, 13, 16, 21, 30, 45];
        const EXPECTED: IntegerType = 68;
        let output = get_extrapolated_history(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_1() {
        const INPUT: &str = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        ";
        const EXPECTED: IntegerType = 114;
        let output = solve_part_1(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_get_backwards_extrapolated_history() {
        const INPUT: &[IntegerType] = &[10, 13, 16, 21, 30, 45];
        const EXPECTED: IntegerType = 5;
        let output = get_backwards_extrapolated_history(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_2() {
        const INPUT: &str = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        ";
        const EXPECTED: IntegerType = 2;
        let output = solve_part_2(INPUT);
        assert_eq!(output, EXPECTED);
    }
}
