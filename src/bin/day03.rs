use core::iter::Peekable;

const INPUT: &str = include_str!("../../data/day03.txt");

#[derive(Debug, PartialEq)]
struct NumberSection {
    value: u32,
    start_x: usize,
    end_x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct SymbolSection {
    symbol: char,
    x: usize,
    y: usize,
}

fn main() {
    let (number_sections, symbol_sections) = read_sections_from_string(INPUT);
    println!(
        "Part 1 => {}",
        solve_part_1(&number_sections, &symbol_sections)
    );
    println!(
        "Part 2 => {}",
        solve_part_2(&number_sections, &symbol_sections)
    );
}

fn solve_part_1(number_sections: &[NumberSection], symbol_sections: &[SymbolSection]) -> u32 {
    number_sections
        .iter()
        .filter_map(|number_section| {
            if symbol_sections.iter().any(|symbol_section| {
                is_number_section_adjacent_to_symbol_section(number_section, symbol_section)
            }) {
                Some(number_section.value)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_2(number_sections: &[NumberSection], symbol_sections: &[SymbolSection]) -> u32 {
    symbol_sections
        .iter()
        .filter_map(|symbol_section| {
            let mut adjacent_numbers = number_sections.iter().filter(|number_section| {
                is_number_section_adjacent_to_symbol_section(number_section, symbol_section)
            });
            adjacent_numbers.next().and_then(|number_section_1| {
                adjacent_numbers.next().and_then(|number_section_2| {
                    if adjacent_numbers.next().is_some() {
                        None
                    } else {
                        Some(number_section_1.value * number_section_2.value)
                    }
                })
            })
        })
        .sum()
}

fn read_number_section_from_iterator(
    iter: &mut Peekable<impl Iterator<Item = (usize, char)>>,
    line_length: usize,
) -> NumberSection {
    let mut value = 0;
    let (start_x, y) = convert_index_to_coordinate(iter.peek().unwrap().0, line_length);
    let mut end_x = 0;
    while let Some((index, digit)) = iter.peek() {
        let (x, local_y) = convert_index_to_coordinate(*index, line_length);
        if local_y == y {
            if let Some(digit) = digit.to_digit(10) {
                end_x = x;
                value = value * 10 + digit;
                iter.next().unwrap(); // advance iterator to remove digit.
            } else {
                break;
            }
        } else {
            break;
        }
    }
    NumberSection {
        value,
        start_x,
        end_x,
        y,
    }
}

fn read_symbol_section_from_iterator(
    iter: &mut impl Iterator<Item = (usize, char)>,
    line_length: usize,
) -> SymbolSection {
    let (index, symbol) = iter.next().unwrap(); // caller should check the next char is a symbol we care about.
    let (x, y) = convert_index_to_coordinate(index, line_length);
    SymbolSection { symbol, x, y }
}

fn read_sections_from_iterator(
    iter: impl Iterator<Item = (usize, char)>,
    line_length: usize,
) -> (Vec<NumberSection>, Vec<SymbolSection>) {
    let mut number_sections = vec![];
    let mut symbol_sections = vec![];
    let mut iter = iter.peekable();
    while let Some((_, peeked)) = iter.peek() {
        if *peeked == '.' {
            iter.next(); // just throw away periods
        } else if peeked.is_ascii_digit() {
            number_sections.push(read_number_section_from_iterator(&mut iter, line_length));
        } else {
            symbol_sections.push(read_symbol_section_from_iterator(&mut iter, line_length));
        }
    }
    (number_sections, symbol_sections)
}

fn read_sections_from_string(string: &str) -> (Vec<NumberSection>, Vec<SymbolSection>) {
    let line_length = string
        .trim()
        .lines()
        .next()
        .unwrap()
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .count();
    read_sections_from_iterator(
        string
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .enumerate(),
        line_length,
    )
}

const fn convert_index_to_coordinate(index: usize, line_length: usize) -> (usize, usize) {
    let y = index / line_length;
    let x = index % line_length;
    (x, y)
}

const fn is_number_section_adjacent_to_symbol_section(
    number_section: &NumberSection,
    symbol_section: &SymbolSection,
) -> bool {
    symbol_section.x >= number_section.start_x.saturating_sub(1)
        && symbol_section.x <= number_section.end_x + 1
        && symbol_section.y >= number_section.y.saturating_sub(1)
        && symbol_section.y <= number_section.y + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::iter;

    #[test]
    fn test_read_number_section_from_iterator() {
        let mut input = iter::once((3, '2'))
            .chain(iter::once((4, '8')))
            .chain(iter::once((5, '2')))
            .chain(iter::once((6, '3')))
            .peekable();
        const INPUT_LINE_LENGTH: usize = 7;
        const EXPECTED: NumberSection = NumberSection {
            value: 2823,
            start_x: 3,
            end_x: 6,
            y: 0,
        };
        let output = read_number_section_from_iterator(&mut input, INPUT_LINE_LENGTH);
        assert_eq!(output, EXPECTED);
        assert_eq!(input.next(), None);
    }

    #[test]
    fn test_read_number_section_from_iterator_with_remaining() {
        let mut input = iter::once((3, '2'))
            .chain(iter::once((4, '8')))
            .chain(iter::once((5, '2')))
            .chain(iter::once((6, '3')))
            .chain(iter::once((7, '*')))
            .peekable();
        const INPUT_LINE_LENGTH: usize = 8;
        const EXPECTED: NumberSection = NumberSection {
            value: 2823,
            start_x: 3,
            end_x: 6,
            y: 0,
        };
        let output = read_number_section_from_iterator(&mut input, INPUT_LINE_LENGTH);
        assert_eq!(output, EXPECTED);
        assert_eq!(input.next(), Some((7, '*')));
    }

    #[test]
    fn test_read_number_section_from_iterator_with_no_digits() {
        let mut input = iter::once((7, '*')).peekable();
        const INPUT_LINE_LENGTH: usize = 5;
        const EXPECTED: NumberSection = NumberSection {
            value: 0,
            start_x: 2,
            y: 1,
            end_x: 0,
        };
        let output = read_number_section_from_iterator(&mut input, INPUT_LINE_LENGTH);
        assert_eq!(output, EXPECTED);
        assert_eq!(input.next(), Some((7, '*')));
    }

    #[test]
    fn test_read_symbol_section_from_iterator() {
        let mut input = iter::once((8, '#'));
        const INPUT_LINE_LENGTH: usize = 5;
        const EXPECTED: SymbolSection = SymbolSection {
            symbol: '#',
            x: 3,
            y: 1,
        };
        let output = read_symbol_section_from_iterator(&mut input, INPUT_LINE_LENGTH);
        assert_eq!(output, EXPECTED);
        assert_eq!(input.next(), None);
    }

    #[test]
    fn test_read_sections_from_iterator() {
        let input = "2345..*.#".chars().enumerate();
        let expected = (
            vec![NumberSection {
                value: 2345,
                start_x: 0,
                end_x: 3,
                y: 0,
            }],
            vec![
                SymbolSection {
                    symbol: '*',
                    x: 6,
                    y: 0,
                },
                SymbolSection {
                    symbol: '#',
                    x: 8,
                    y: 0,
                },
            ],
        );
        let output = read_sections_from_iterator(input, 9);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_read_sections_from_string_single_line() {
        const INPUT: &str = "2345..*.#";
        let expected = (
            vec![NumberSection {
                value: 2345,
                start_x: 0,
                end_x: 3,
                y: 0,
            }],
            vec![
                SymbolSection {
                    symbol: '*',
                    x: 6,
                    y: 0,
                },
                SymbolSection {
                    symbol: '#',
                    x: 8,
                    y: 0,
                },
            ],
        );
        let output = read_sections_from_string(INPUT);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_read_sections_from_string_multi_line() {
        const INPUT: &str = "
        2345..*.#
        ..?.1719%
        ";
        let expected = (
            vec![
                NumberSection {
                    value: 2345,
                    start_x: 0,
                    end_x: 3,
                    y: 0,
                },
                NumberSection {
                    value: 1719,
                    start_x: 4,
                    end_x: 7,
                    y: 1,
                },
            ],
            vec![
                SymbolSection {
                    symbol: '*',
                    x: 6,
                    y: 0,
                },
                SymbolSection {
                    symbol: '#',
                    x: 8,
                    y: 0,
                },
                SymbolSection {
                    symbol: '?',
                    x: 2,
                    y: 1,
                },
                SymbolSection {
                    symbol: '%',
                    x: 8,
                    y: 1,
                },
            ],
        );
        let output = read_sections_from_string(INPUT);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_convert_index_to_coordinate() {
        const INPUT_INDEX: usize = 17;
        const INPUT_LINE_LENGTH: usize = 4;
        const EXPECTED: (usize, usize) = (1, 4);
        let output = convert_index_to_coordinate(INPUT_INDEX, INPUT_LINE_LENGTH);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_number_section_adjacent_to_symbol_section() {
        const INPUT_NUMBER_SECTION: &NumberSection = &NumberSection {
            value: 1719,
            start_x: 4,
            end_x: 7,
            y: 1,
        };
        const INPUT_SYMBOL_SECTION: &SymbolSection = &SymbolSection {
            symbol: '%',
            x: 8,
            y: 1,
        };
        const EXPECTED: bool = true;
        let output = is_number_section_adjacent_to_symbol_section(
            INPUT_NUMBER_SECTION,
            INPUT_SYMBOL_SECTION,
        );
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_number_section_not_adjacent_to_symbol_section() {
        const INPUT_NUMBER_SECTION: &NumberSection = &NumberSection {
            value: 1719,
            start_x: 4,
            end_x: 7,
            y: 1,
        };
        const INPUT_SYMBOL_SECTION: &SymbolSection = &SymbolSection {
            symbol: '?',
            x: 2,
            y: 1,
        };
        const EXPECTED: bool = false;
        let output = is_number_section_adjacent_to_symbol_section(
            INPUT_NUMBER_SECTION,
            INPUT_SYMBOL_SECTION,
        );
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_1() {
        const INPUT: &str = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+..58
        592.......
        ......755.
        ...$.*....
        .664.598..
        ";
        const EXPECTED: u32 = 3769;
        let (number_sections, symbol_sections) = read_sections_from_string(INPUT);
        let output = solve_part_1(&number_sections, &symbol_sections);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_2() {
        const INPUT: &str = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ";
        const EXPECTED: u32 = 467835;
        let (number_sections, symbol_sections) = read_sections_from_string(INPUT);
        let output = solve_part_2(&number_sections, &symbol_sections);
        assert_eq!(output, EXPECTED);
    }
}
