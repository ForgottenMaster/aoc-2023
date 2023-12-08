const INPUT: &str = include_str!("../../data/day06.txt");

type IntegerType = u64;

fn main() {
    println!("Part 1 => {}", solve_part_1(INPUT));
    println!("Part 2 => {}", solve_part_2(INPUT));
}

fn parse_times_and_distances_from_string(input: &str) -> (Vec<IntegerType>, Vec<IntegerType>) {
    let mut iter = input.trim().lines().map(|line| {
        line.split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect()
    });
    (iter.next().unwrap(), iter.next().unwrap())
}

fn parse_time_and_distance_from_string_with_kerning_correction(
    input: &str,
) -> (IntegerType, IntegerType) {
    let mut iter = input.trim().lines().map(|line| {
        line.split_once(':')
            .unwrap()
            .1
            .trim()
            .chars()
            .filter(|c| !c.is_whitespace())
            .fold(0, |total, c| {
                total * 10 + (c as IntegerType - '0' as IntegerType)
            })
    });
    (iter.next().unwrap(), iter.next().unwrap())
}

fn calculate_distance_if_button_held_for(time: IntegerType, limit: IntegerType) -> IntegerType {
    (limit - time) * time
}

fn calculate_number_of_ways_to_win(limit: IntegerType, record: IntegerType) -> IntegerType {
    (0..=limit)
        .filter(|time| calculate_distance_if_button_held_for(*time, limit) > record)
        .count() as IntegerType
}

fn calculate_margin_of_error(limits: &[IntegerType], records: &[IntegerType]) -> IntegerType {
    limits
        .iter()
        .zip(records.iter())
        .map(|(limit, record)| calculate_number_of_ways_to_win(*limit, *record))
        .product()
}

fn solve_part_1(input: &str) -> IntegerType {
    let (limits, records) = parse_times_and_distances_from_string(input);
    calculate_margin_of_error(&limits, &records)
}

fn solve_part_2(input: &str) -> IntegerType {
    let (limit, record) = parse_time_and_distance_from_string_with_kerning_correction(input);
    calculate_margin_of_error(&[limit], &[record])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_times_and_distances_from_string() {
        const INPUT: &str = "
        Time:      7  15   30
        Distance:  9  40  200
        ";
        const EXPECTED_TIMES: &[IntegerType] = &[7, 15, 30];
        const EXPECTED_DISTANCES: &[IntegerType] = &[9, 40, 200];
        let (times, distances) = parse_times_and_distances_from_string(INPUT);
        assert_eq!(&times, EXPECTED_TIMES);
        assert_eq!(&distances, EXPECTED_DISTANCES);
    }

    #[test]
    fn test_calculate_distance_if_button_held_for_0() {
        const INPUT_TIME: IntegerType = 0;
        const INPUT_LIMIT: IntegerType = 7;
        const EXPECTED: IntegerType = 0;
        let output = calculate_distance_if_button_held_for(INPUT_TIME, INPUT_LIMIT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_calculate_distance_if_button_held_for_1() {
        const INPUT_TIME: IntegerType = 1;
        const INPUT_LIMIT: IntegerType = 7;
        const EXPECTED: IntegerType = 6;
        let output = calculate_distance_if_button_held_for(INPUT_TIME, INPUT_LIMIT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_calculate_distance_if_button_held_for_2() {
        const INPUT_TIME: IntegerType = 2;
        const INPUT_LIMIT: IntegerType = 7;
        const EXPECTED: IntegerType = 10;
        let output = calculate_distance_if_button_held_for(INPUT_TIME, INPUT_LIMIT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_calculate_distance_if_button_held_for_3() {
        const INPUT_TIME: IntegerType = 3;
        const INPUT_LIMIT: IntegerType = 7;
        const EXPECTED: IntegerType = 12;
        let output = calculate_distance_if_button_held_for(INPUT_TIME, INPUT_LIMIT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_calculate_distance_if_button_held_for_4() {
        const INPUT_TIME: IntegerType = 4;
        const INPUT_LIMIT: IntegerType = 7;
        const EXPECTED: IntegerType = 12;
        let output = calculate_distance_if_button_held_for(INPUT_TIME, INPUT_LIMIT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_calculate_distance_if_button_held_for_5() {
        const INPUT_TIME: IntegerType = 5;
        const INPUT_LIMIT: IntegerType = 7;
        const EXPECTED: IntegerType = 10;
        let output = calculate_distance_if_button_held_for(INPUT_TIME, INPUT_LIMIT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_calculate_distance_if_button_held_for_6() {
        const INPUT_TIME: IntegerType = 6;
        const INPUT_LIMIT: IntegerType = 7;
        const EXPECTED: IntegerType = 6;
        let output = calculate_distance_if_button_held_for(INPUT_TIME, INPUT_LIMIT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_calculate_distance_if_button_held_for_7() {
        const INPUT_TIME: IntegerType = 7;
        const INPUT_LIMIT: IntegerType = 7;
        const EXPECTED: IntegerType = 0;
        let output = calculate_distance_if_button_held_for(INPUT_TIME, INPUT_LIMIT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_calculate_number_of_ways_for_race_case_1() {
        const INPUT_LIMIT: IntegerType = 7;
        const INPUT_RECORD: IntegerType = 9;
        const EXPECTED: IntegerType = 4;
        let output = calculate_number_of_ways_to_win(INPUT_LIMIT, INPUT_RECORD);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_calculate_number_of_ways_for_race_case_2() {
        const INPUT_LIMIT: IntegerType = 15;
        const INPUT_RECORD: IntegerType = 40;
        const EXPECTED: IntegerType = 8;
        let output = calculate_number_of_ways_to_win(INPUT_LIMIT, INPUT_RECORD);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_calculate_number_of_ways_for_race_case_3() {
        const INPUT_LIMIT: IntegerType = 30;
        const INPUT_RECORD: IntegerType = 200;
        const EXPECTED: IntegerType = 9;
        let output = calculate_number_of_ways_to_win(INPUT_LIMIT, INPUT_RECORD);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_calculate_margin_of_error() {
        const INPUT_LIMITS: &[IntegerType] = &[7, 15, 30];
        const INPUT_RECORDS: &[IntegerType] = &[9, 40, 200];
        const EXPECTED: IntegerType = 288;
        let output = calculate_margin_of_error(INPUT_LIMITS, INPUT_RECORDS);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_1() {
        const INPUT: &str = "
        Time:      7  15   30
        Distance:  9  40  200
        ";
        const EXPECTED: IntegerType = 288;
        let output = solve_part_1(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_time_and_distance_from_string_with_kerning_correction() {
        const INPUT: &str = "
        Time:      7  15   30
        Distance:  9  40  200
        ";
        const EXPECTED_TIME: IntegerType = 71530;
        const EXPECTED_DISTANCE: IntegerType = 940200;
        let (time, distance) = parse_time_and_distance_from_string_with_kerning_correction(INPUT);
        assert_eq!(time, EXPECTED_TIME);
        assert_eq!(distance, EXPECTED_DISTANCE);
    }

    #[test]
    fn test_solve_part_2() {
        const INPUT: &str = "
        Time:      7  15   30
        Distance:  9  40  200
        ";
        const EXPECTED: IntegerType = 71503;
        let output = solve_part_2(INPUT);
        assert_eq!(output, EXPECTED);
    }
}
