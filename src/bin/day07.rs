use core::cmp::Ordering;

const INPUT: &str = include_str!("../../data/day07.txt");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    hand_type: HandType,
    values: [Value; 5],
    counts: [Option<(Value, u8)>; 5],
}

fn main() {
    println!("Part 1 => {}", solve_part_1(INPUT));
}

fn solve_part_1(input: &str) -> u32 {
    let mut hands_and_bids = parse_hands_and_bids(input);
    hands_and_bids.sort_by(|hand_1, hand_2| hand_2.0.cmp(&hand_1.0));
    hands_and_bids
        .into_iter()
        .enumerate()
        .map(|(index, (_, bid))| (index as u32 + 1) * bid)
        .sum()
}

fn parse_value(input: char) -> Value {
    match input {
        'A' => Value::Ace,
        'K' => Value::King,
        'Q' => Value::Queen,
        'J' => Value::Jack,
        'T' => Value::Ten,
        '9' => Value::Nine,
        '8' => Value::Eight,
        '7' => Value::Seven,
        '6' => Value::Six,
        '5' => Value::Five,
        '4' => Value::Four,
        '3' => Value::Three,
        '2' => Value::Two,
        _ => unimplemented!("Not implemented for {}", input),
    }
}

fn parse_hand_values(input: &str) -> [Value; 5] {
    let mut chars = input.chars().map(parse_value);
    [
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
    ]
}

fn count_values(hand: &[Value; 5]) -> [Option<(Value, u8)>; 5] {
    let mut counts = [None; 5];
    let mut index = 0;
    'outer: for value in hand {
        for count in &mut counts {
            if let Some((storage_value, count)) = count {
                if value == storage_value {
                    *count += 1;
                    continue 'outer;
                }
            } else {
                break;
            }
        }
        counts[index] = Some((*value, 1));
        index += 1;
    }
    counts.sort_by(|count_1, count_2| match (count_1, count_2) {
        (None, Some(_)) | (None, None) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some((_, count_1)), Some((_, count_2))) => count_2.cmp(count_1),
    });
    counts
}

fn determine_hand_type(counts: &[Option<(Value, u8)>; 5]) -> HandType {
    match counts {
        [Some((_, 5)), ..] => HandType::FiveOfAKind,
        [Some((_, 4)), ..] => HandType::FourOfAKind,
        [Some((_, 3)), Some((_, 2)), ..] => HandType::FullHouse,
        [Some((_, 3)), ..] => HandType::ThreeOfAKind,
        [Some((_, 2)), Some((_, 2)), ..] => HandType::TwoPair,
        [Some((_, 2)), ..] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn parse_hand(input: &str) -> Hand {
    let values = parse_hand_values(input);
    let counts = count_values(&values);
    let hand_type = determine_hand_type(&counts);
    Hand {
        hand_type,
        counts,
        values,
    }
}

fn parse_hands_and_bids(input: &str) -> Vec<(Hand, u32)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parse_hand(parts.next().unwrap());
            let bid: u32 = parts.next().unwrap().parse().unwrap();
            (hand, bid)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value_ace() {
        const INPUT: char = 'A';
        const EXPECTED: Value = Value::Ace;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_king() {
        const INPUT: char = 'K';
        const EXPECTED: Value = Value::King;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_queen() {
        const INPUT: char = 'Q';
        const EXPECTED: Value = Value::Queen;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_jack() {
        const INPUT: char = 'J';
        const EXPECTED: Value = Value::Jack;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_ten() {
        const INPUT: char = 'T';
        const EXPECTED: Value = Value::Ten;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_nine() {
        const INPUT: char = '9';
        const EXPECTED: Value = Value::Nine;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_eight() {
        const INPUT: char = '8';
        const EXPECTED: Value = Value::Eight;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_seven() {
        const INPUT: char = '7';
        const EXPECTED: Value = Value::Seven;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_six() {
        const INPUT: char = '6';
        const EXPECTED: Value = Value::Six;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_five() {
        const INPUT: char = '5';
        const EXPECTED: Value = Value::Five;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_four() {
        const INPUT: char = '4';
        const EXPECTED: Value = Value::Four;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_three() {
        const INPUT: char = '3';
        const EXPECTED: Value = Value::Three;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_value_two() {
        const INPUT: char = '2';
        const EXPECTED: Value = Value::Two;
        let output = parse_value(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_hand_values() {
        const INPUT: &str = "AT52A";
        const EXPECTED: [Value; 5] = [Value::Ace, Value::Ten, Value::Five, Value::Two, Value::Ace];
        let output = parse_hand_values(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_count_values_five_of_a_kind() {
        const INPUT: [Value; 5] = [Value::Ace, Value::Ace, Value::Ace, Value::Ace, Value::Ace];
        const EXPECTED: [Option<(Value, u8)>; 5] = [Some((Value::Ace, 5)), None, None, None, None];
        let output = count_values(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_count_values_four_of_a_kind() {
        const INPUT: [Value; 5] = [Value::Ace, Value::Ace, Value::Eight, Value::Ace, Value::Ace];
        const EXPECTED: [Option<(Value, u8)>; 5] = [
            Some((Value::Ace, 4)),
            Some((Value::Eight, 1)),
            None,
            None,
            None,
        ];
        let output = count_values(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_count_values_full_house() {
        const INPUT: [Value; 5] = [
            Value::Two,
            Value::Three,
            Value::Three,
            Value::Three,
            Value::Two,
        ];
        const EXPECTED: [Option<(Value, u8)>; 5] = [
            Some((Value::Three, 3)),
            Some((Value::Two, 2)),
            None,
            None,
            None,
        ];
        let output = count_values(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_count_values_three_of_a_kind() {
        const INPUT: [Value; 5] = [
            Value::Ten,
            Value::Ten,
            Value::Ten,
            Value::Nine,
            Value::Eight,
        ];
        const EXPECTED: [Option<(Value, u8)>; 5] = [
            Some((Value::Ten, 3)),
            Some((Value::Nine, 1)),
            Some((Value::Eight, 1)),
            None,
            None,
        ];
        let output = count_values(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_count_values_two_pair() {
        const INPUT: [Value; 5] = [
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Three,
            Value::Two,
        ];
        const EXPECTED: [Option<(Value, u8)>; 5] = [
            Some((Value::Two, 2)),
            Some((Value::Three, 2)),
            Some((Value::Four, 1)),
            None,
            None,
        ];
        let output = count_values(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_count_values_one_pair() {
        const INPUT: [Value; 5] = [
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Ace,
            Value::Four,
        ];
        const EXPECTED: [Option<(Value, u8)>; 5] = [
            Some((Value::Ace, 2)),
            Some((Value::Two, 1)),
            Some((Value::Three, 1)),
            Some((Value::Four, 1)),
            None,
        ];
        let output = count_values(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_count_values_high_card() {
        const INPUT: [Value; 5] = [
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
        ];
        const EXPECTED: [Option<(Value, u8)>; 5] = [
            Some((Value::Two, 1)),
            Some((Value::Three, 1)),
            Some((Value::Four, 1)),
            Some((Value::Five, 1)),
            Some((Value::Six, 1)),
        ];
        let output = count_values(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_determine_hand_type_five_of_a_kind() {
        const INPUT: [Option<(Value, u8)>; 5] = [Some((Value::Ace, 5)), None, None, None, None];
        const EXPECTED: HandType = HandType::FiveOfAKind;
        let output = determine_hand_type(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_determine_hand_type_four_of_a_kind() {
        const INPUT: [Option<(Value, u8)>; 5] = [
            Some((Value::Ace, 4)),
            Some((Value::Eight, 1)),
            None,
            None,
            None,
        ];
        const EXPECTED: HandType = HandType::FourOfAKind;
        let output = determine_hand_type(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_determine_hand_type_full_house() {
        const INPUT: [Option<(Value, u8)>; 5] = [
            Some((Value::Three, 3)),
            Some((Value::Two, 2)),
            None,
            None,
            None,
        ];
        const EXPECTED: HandType = HandType::FullHouse;
        let output = determine_hand_type(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_determine_hand_type_three_of_a_kind() {
        const INPUT: [Option<(Value, u8)>; 5] = [
            Some((Value::Ten, 3)),
            Some((Value::Nine, 1)),
            Some((Value::Eight, 1)),
            None,
            None,
        ];
        const EXPECTED: HandType = HandType::ThreeOfAKind;
        let output = determine_hand_type(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_determine_hand_type_two_pair() {
        const INPUT: [Option<(Value, u8)>; 5] = [
            Some((Value::Two, 2)),
            Some((Value::Three, 2)),
            Some((Value::Four, 1)),
            None,
            None,
        ];
        const EXPECTED: HandType = HandType::TwoPair;
        let output = determine_hand_type(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_determine_hand_type_one_pair() {
        const INPUT: [Option<(Value, u8)>; 5] = [
            Some((Value::Ace, 2)),
            Some((Value::Two, 1)),
            Some((Value::Three, 1)),
            Some((Value::Four, 1)),
            None,
        ];
        const EXPECTED: HandType = HandType::OnePair;
        let output = determine_hand_type(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_determine_hand_type_high_card() {
        const INPUT: [Option<(Value, u8)>; 5] = [
            Some((Value::Two, 1)),
            Some((Value::Three, 1)),
            Some((Value::Four, 1)),
            Some((Value::Five, 1)),
            Some((Value::Six, 1)),
        ];
        const EXPECTED: HandType = HandType::HighCard;
        let output = determine_hand_type(&INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_hand_five_of_a_kind() {
        const INPUT: &str = "99999";
        const EXPECTED: Hand = Hand {
            hand_type: HandType::FiveOfAKind,
            counts: [Some((Value::Nine, 5)), None, None, None, None],
            values: [
                Value::Nine,
                Value::Nine,
                Value::Nine,
                Value::Nine,
                Value::Nine,
            ],
        };
        let output = parse_hand(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_hand_four_of_a_kind() {
        const INPUT: &str = "99799";
        const EXPECTED: Hand = Hand {
            hand_type: HandType::FourOfAKind,
            counts: [
                Some((Value::Nine, 4)),
                Some((Value::Seven, 1)),
                None,
                None,
                None,
            ],
            values: [
                Value::Nine,
                Value::Nine,
                Value::Seven,
                Value::Nine,
                Value::Nine,
            ],
        };
        let output = parse_hand(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_hand_full_house() {
        const INPUT: &str = "99797";
        const EXPECTED: Hand = Hand {
            hand_type: HandType::FullHouse,
            counts: [
                Some((Value::Nine, 3)),
                Some((Value::Seven, 2)),
                None,
                None,
                None,
            ],
            values: [
                Value::Nine,
                Value::Nine,
                Value::Seven,
                Value::Nine,
                Value::Seven,
            ],
        };
        let output = parse_hand(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_hand_three_of_a_kind() {
        const INPUT: &str = "69799";
        const EXPECTED: Hand = Hand {
            hand_type: HandType::ThreeOfAKind,
            counts: [
                Some((Value::Nine, 3)),
                Some((Value::Six, 1)),
                Some((Value::Seven, 1)),
                None,
                None,
            ],
            values: [
                Value::Six,
                Value::Nine,
                Value::Seven,
                Value::Nine,
                Value::Nine,
            ],
        };
        let output = parse_hand(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_hand_two_pair() {
        const INPUT: &str = "69797";
        const EXPECTED: Hand = Hand {
            hand_type: HandType::TwoPair,
            counts: [
                Some((Value::Nine, 2)),
                Some((Value::Seven, 2)),
                Some((Value::Six, 1)),
                None,
                None,
            ],
            values: [
                Value::Six,
                Value::Nine,
                Value::Seven,
                Value::Nine,
                Value::Seven,
            ],
        };
        let output = parse_hand(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_hand_one_pair() {
        const INPUT: &str = "69798";
        const EXPECTED: Hand = Hand {
            hand_type: HandType::OnePair,
            counts: [
                Some((Value::Nine, 2)),
                Some((Value::Six, 1)),
                Some((Value::Seven, 1)),
                Some((Value::Eight, 1)),
                None,
            ],
            values: [
                Value::Six,
                Value::Nine,
                Value::Seven,
                Value::Nine,
                Value::Eight,
            ],
        };
        let output = parse_hand(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_hand_high_card() {
        const INPUT: &str = "69738";
        const EXPECTED: Hand = Hand {
            hand_type: HandType::HighCard,
            counts: [
                Some((Value::Six, 1)),
                Some((Value::Nine, 1)),
                Some((Value::Seven, 1)),
                Some((Value::Three, 1)),
                Some((Value::Eight, 1)),
            ],
            values: [
                Value::Six,
                Value::Nine,
                Value::Seven,
                Value::Three,
                Value::Eight,
            ],
        };
        let output = parse_hand(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_hands_and_bids() {
        const INPUT: &str = "
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        ";
        const EXPECTED: &[(Hand, u32)] = &[
            (
                Hand {
                    hand_type: HandType::OnePair,
                    counts: [
                        Some((Value::Three, 2)),
                        Some((Value::Two, 1)),
                        Some((Value::Ten, 1)),
                        Some((Value::King, 1)),
                        None,
                    ],
                    values: [
                        Value::Three,
                        Value::Two,
                        Value::Ten,
                        Value::Three,
                        Value::King,
                    ],
                },
                765,
            ),
            (
                Hand {
                    hand_type: HandType::ThreeOfAKind,
                    counts: [
                        Some((Value::Five, 3)),
                        Some((Value::Ten, 1)),
                        Some((Value::Jack, 1)),
                        None,
                        None,
                    ],
                    values: [
                        Value::Ten,
                        Value::Five,
                        Value::Five,
                        Value::Jack,
                        Value::Five,
                    ],
                },
                684,
            ),
            (
                Hand {
                    hand_type: HandType::TwoPair,
                    counts: [
                        Some((Value::King, 2)),
                        Some((Value::Seven, 2)),
                        Some((Value::Six, 1)),
                        None,
                        None,
                    ],
                    values: [
                        Value::King,
                        Value::King,
                        Value::Six,
                        Value::Seven,
                        Value::Seven,
                    ],
                },
                28,
            ),
            (
                Hand {
                    hand_type: HandType::TwoPair,
                    counts: [
                        Some((Value::Ten, 2)),
                        Some((Value::Jack, 2)),
                        Some((Value::King, 1)),
                        None,
                        None,
                    ],
                    values: [
                        Value::King,
                        Value::Ten,
                        Value::Jack,
                        Value::Jack,
                        Value::Ten,
                    ],
                },
                220,
            ),
            (
                Hand {
                    hand_type: HandType::ThreeOfAKind,
                    counts: [
                        Some((Value::Queen, 3)),
                        Some((Value::Jack, 1)),
                        Some((Value::Ace, 1)),
                        None,
                        None,
                    ],
                    values: [
                        Value::Queen,
                        Value::Queen,
                        Value::Queen,
                        Value::Jack,
                        Value::Ace,
                    ],
                },
                483,
            ),
        ];
        let output = parse_hands_and_bids(INPUT);
        assert_eq!(&output, EXPECTED);
    }

    #[test]
    fn test_solve_part_1() {
        const INPUT: &str = "
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        ";
        const EXPECTED: u32 = 6440;
        let output = solve_part_1(INPUT);
        assert_eq!(output, EXPECTED);
    }
}
