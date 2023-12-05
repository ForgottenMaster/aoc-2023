const INPUT: &str = include_str!("../../data/day04.txt");

fn main() {
    println!("Part 1 => {}", solve_part_1(INPUT));
    println!("Part 2 => {}", solve_part_2(INPUT));
}

fn count_matching_numbers_in_game(string: &str) -> usize {
    let string = string.split_once(": ").unwrap().1; // remove Game X:
    let (winning_numbers, your_numbers) = string.split_once(" | ").unwrap();
    winning_numbers
        .split_whitespace()
        .filter(|winning_number| {
            your_numbers
                .split_whitespace()
                .any(|your_number| *winning_number == your_number)
        })
        .count()
}

fn score_game_for_part_1(string: &str) -> usize {
    let count = count_matching_numbers_in_game(string.trim());
    if count == 0 {
        0
    } else {
        2_usize.pow((count - 1) as u32)
    }
}

fn solve_part_1(string: &str) -> usize {
    string.trim().lines().map(score_game_for_part_1).sum()
}

fn solve_part_2(string: &str) -> usize {
    let counts: Vec<_> = string
        .trim()
        .lines()
        .map(count_matching_numbers_in_game)
        .collect();
    let mut dp = vec![0; counts.len()];
    for (index, count) in counts.into_iter().enumerate().rev() {
        let mut total = 1;
        for i in 1..=count {
            total += dp[index + i];
        }
        dp[index] = total;
    }
    dp.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_matching_numbers_in_game() {
        const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        const EXPECTED: usize = 4;
        let output = count_matching_numbers_in_game(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_score_game_for_part_1() {
        const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        const EXPECTED: usize = 8;
        let output = score_game_for_part_1(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_1() {
        const INPUT: &str = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";
        const EXPECTED: usize = 13;
        let output = solve_part_1(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_2() {
        const INPUT: &str = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";
        const EXPECTED: usize = 30;
        let output = solve_part_2(INPUT);
        assert_eq!(output, EXPECTED);
    }
}
