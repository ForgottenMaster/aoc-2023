const INPUT: &str = include_str!("../../data/day05.txt");

#[derive(Debug, PartialEq)]
enum RelationType {
    Before,
    After,
    OverlapStart,
    OverlapEnd,
    Contained,
    Envelops,
}

#[derive(Debug, PartialEq)]
enum MappingResult {
    Nothing,
    Unmapped((u32, u32)),
    Mapped((u32, u32)),
}

fn main() {
    let (numbers, maps) = parse_seed_numbers_and_maps_from_string(INPUT);
    let map_refs = maps
        .iter()
        .map(|map| map as _)
        .collect::<Vec<&[(u32, u32, u32)]>>();
    println!("Part 1 => {}", solve_part_1(&numbers, &map_refs));
    println!("Part 2 => {}", solve_part_2(&numbers, &map_refs));
}

fn solve_part_1(numbers: &[u32], maps: &[&[(u32, u32, u32)]]) -> u32 {
    let mut ranges = numbers.iter().map(|num| (*num, 1)).collect();
    solve_for_ranges(&mut ranges, maps)
}

fn solve_part_2(numbers: &[u32], maps: &[&[(u32, u32, u32)]]) -> u32 {
    let mut ranges = numbers
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();
    solve_for_ranges(&mut ranges, maps)
}

fn solve_for_ranges(ranges: &mut Vec<(u32, u32)>, maps: &[&[(u32, u32, u32)]]) -> u32 {
    fully_map_ranges(ranges, maps);
    ranges.iter().map(|(start, _)| *start).min().unwrap()
}

fn fully_map_ranges(ranges: &mut Vec<(u32, u32)>, maps: &[&[(u32, u32, u32)]]) {
    let mut mapped: Vec<(u32, u32)> = Vec::new();
    let mut unmapped: Vec<(u32, u32)> = Vec::new();
    for map in maps {
        for section in *map {
            while let Some(range) = ranges.pop() {
                let (start, middle, end) = map_range(range, *section);
                match start {
                    MappingResult::Mapped(range) => mapped.push(range),
                    MappingResult::Unmapped(range) => unmapped.push(range),
                    _ => {}
                };
                match middle {
                    MappingResult::Mapped(range) => mapped.push(range),
                    MappingResult::Unmapped(range) => unmapped.push(range),
                    _ => {}
                };
                match end {
                    MappingResult::Mapped(range) => mapped.push(range),
                    MappingResult::Unmapped(range) => unmapped.push(range),
                    _ => {}
                };
            }
            std::mem::swap(ranges, &mut unmapped);
        }
        ranges.append(&mut mapped);
    }
}

fn parse_numbers_from_string(string: &str) -> Vec<u32> {
    string
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect()
}

fn parse_map_from_string(string: &str) -> Vec<(u32, u32, u32)> {
    string.lines().skip(1).map(parse_line).collect()
}

fn parse_line(string: &str) -> (u32, u32, u32) {
    let mut iter = string.split_whitespace().map(|num| num.parse().unwrap());
    (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

type Section = (u32, u32, u32);
type Map = Vec<Section>;

fn parse_seed_numbers_and_maps_from_string(string: &str) -> (Vec<u32>, Vec<Map>) {
    let mut chunks = string.trim().split("\n\n").map(|line| line.trim());
    let seed_numbers = parse_numbers_from_string(chunks.next().unwrap());
    let maps = chunks.map(parse_map_from_string).collect();
    (seed_numbers, maps)
}

fn relation_type(
    (range_start, range_length): (u32, u32),
    (_, source_start, map_length): (u32, u32, u32),
) -> RelationType {
    let range_end = range_start + range_length - 1;
    let source_end = source_start + map_length - 1;
    if range_end < source_start {
        RelationType::Before
    } else if range_start > source_end {
        RelationType::After
    } else if range_start < source_start && range_end <= source_end {
        RelationType::OverlapStart
    } else if range_start >= source_start && range_end > source_end {
        RelationType::OverlapEnd
    } else if range_start <= source_start && range_end >= source_end {
        RelationType::Envelops
    } else {
        RelationType::Contained
    }
}

fn map_range(
    (range_start, range_length): (u32, u32),
    (destination_start, source_start, map_length): (u32, u32, u32),
) -> (MappingResult, MappingResult, MappingResult) {
    match relation_type(
        (range_start, range_length),
        (destination_start, source_start, map_length),
    ) {
        RelationType::Before => map_range_with_before_relation(
            (range_start, range_length),
            (destination_start, source_start, map_length),
        ),
        RelationType::After => map_range_with_after_relation(
            (range_start, range_length),
            (destination_start, source_start, map_length),
        ),
        RelationType::OverlapStart => map_range_with_overlap_start_relation(
            (range_start, range_length),
            (destination_start, source_start, map_length),
        ),
        RelationType::OverlapEnd => map_range_with_overlap_end_relation(
            (range_start, range_length),
            (destination_start, source_start, map_length),
        ),
        RelationType::Contained => map_range_with_contained_relation(
            (range_start, range_length),
            (destination_start, source_start, map_length),
        ),
        RelationType::Envelops => map_range_with_envelops_relation(
            (range_start, range_length),
            (destination_start, source_start, map_length),
        ),
    }
}

fn map_range_with_before_relation(
    (range_start, range_length): (u32, u32),
    (_, _, _): (u32, u32, u32),
) -> (MappingResult, MappingResult, MappingResult) {
    (
        MappingResult::Unmapped((range_start, range_length)),
        MappingResult::Nothing,
        MappingResult::Nothing,
    )
}

fn map_range_with_after_relation(
    (range_start, range_length): (u32, u32),
    (_, _, _): (u32, u32, u32),
) -> (MappingResult, MappingResult, MappingResult) {
    (
        MappingResult::Nothing,
        MappingResult::Nothing,
        MappingResult::Unmapped((range_start, range_length)),
    )
}

fn map_range_with_overlap_start_relation(
    (range_start, range_length): (u32, u32),
    (destination_start, source_start, _): (u32, u32, u32),
) -> (MappingResult, MappingResult, MappingResult) {
    let number_unmapped = source_start - range_start;
    let number_mapped = range_length - number_unmapped;
    (
        MappingResult::Unmapped((range_start, number_unmapped)),
        MappingResult::Mapped((destination_start, number_mapped)),
        MappingResult::Nothing,
    )
}

fn map_range_with_overlap_end_relation(
    (range_start, range_length): (u32, u32),
    (destination_start, source_start, map_length): (u32, u32, u32),
) -> (MappingResult, MappingResult, MappingResult) {
    let offset = range_start - source_start;
    let number_mapped = map_length - offset;
    let number_unmapped = range_length - number_mapped;
    (
        MappingResult::Nothing,
        MappingResult::Mapped((destination_start + offset, number_mapped)),
        MappingResult::Unmapped((range_start + number_mapped, number_unmapped)),
    )
}

fn map_range_with_contained_relation(
    (range_start, range_length): (u32, u32),
    (destination_start, source_start, _): (u32, u32, u32),
) -> (MappingResult, MappingResult, MappingResult) {
    let offset = range_start - source_start;
    (
        MappingResult::Nothing,
        MappingResult::Mapped((destination_start + offset, range_length)),
        MappingResult::Nothing,
    )
}

fn map_range_with_envelops_relation(
    (range_start, range_length): (u32, u32),
    (destination_start, source_start, map_length): (u32, u32, u32),
) -> (MappingResult, MappingResult, MappingResult) {
    let number_preceding = source_start - range_start;
    let number_trailing = range_length - map_length - number_preceding;
    (
        MappingResult::Unmapped((range_start, number_preceding)),
        MappingResult::Mapped((destination_start, map_length)),
        MappingResult::Unmapped((range_start + number_preceding + map_length, number_trailing)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers_from_string() {
        const INPUT: &str = "seeds: 79 14 55 13";
        const EXPECTED: &[u32] = &[79, 14, 55, 13];
        let output = parse_numbers_from_string(INPUT);
        assert_eq!(&output, EXPECTED);
    }

    #[test]
    fn test_parse_line() {
        const INPUT: &str = "0 15 37";
        const EXPECTED: (u32, u32, u32) = (0, 15, 37);
        let output = parse_line(INPUT);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_map_from_string() {
        const INPUT: &str = "
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        ";
        const EXPECTED: &[(u32, u32, u32)] = &[(0, 15, 37), (37, 52, 2), (39, 0, 15)];
        let output = parse_map_from_string(INPUT.trim());
        assert_eq!(&output, EXPECTED);
    }

    #[test]
    fn test_parse_seed_numbers_and_maps_from_string() {
        const INPUT: &str = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
        ";
        let expected: (Vec<u32>, Vec<Vec<(u32, u32, u32)>>) = (
            vec![79, 14, 55, 13],
            vec![
                vec![(50, 98, 2), (52, 50, 48)],
                vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
                vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
                vec![(88, 18, 7), (18, 25, 70)],
                vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)],
                vec![(0, 69, 1), (1, 0, 69)],
                vec![(60, 56, 37), (56, 93, 4)],
            ],
        );
        let output = parse_seed_numbers_and_maps_from_string(INPUT);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_solve_part_1() {
        const INPUT_MAPS: &[&[(u32, u32, u32)]] = &[
            &[(50, 98, 2), (52, 50, 48)],
            &[(0, 15, 37), (37, 52, 2), (39, 0, 15)],
            &[(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
            &[(88, 18, 7), (18, 25, 70)],
            &[(45, 77, 23), (81, 45, 19), (68, 64, 13)],
            &[(0, 69, 1), (1, 0, 69)],
            &[(60, 56, 37), (56, 93, 4)],
        ];
        const INPUT_NUMBERS: &[u32] = &[79, 14, 55, 13];
        const EXPECTED: u32 = 35;
        let output = solve_part_1(INPUT_NUMBERS, INPUT_MAPS);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_part_2() {
        const INPUT_MAPS: &[&[(u32, u32, u32)]] = &[
            &[(50, 98, 2), (52, 50, 48)],
            &[(0, 15, 37), (37, 52, 2), (39, 0, 15)],
            &[(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
            &[(88, 18, 7), (18, 25, 70)],
            &[(45, 77, 23), (81, 45, 19), (68, 64, 13)],
            &[(0, 69, 1), (1, 0, 69)],
            &[(60, 56, 37), (56, 93, 4)],
        ];
        const INPUT_NUMBERS: &[u32] = &[79, 14, 55, 13];
        const EXPECTED: u32 = 46;
        let output = solve_part_2(INPUT_NUMBERS, INPUT_MAPS);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_solve_for_ranges() {
        const INPUT_MAPS: &[&[(u32, u32, u32)]] = &[
            &[(50, 98, 2), (52, 50, 48)],
            &[(0, 15, 37), (37, 52, 2), (39, 0, 15)],
            &[(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
            &[(88, 18, 7), (18, 25, 70)],
            &[(45, 77, 23), (81, 45, 19), (68, 64, 13)],
            &[(0, 69, 1), (1, 0, 69)],
            &[(60, 56, 37), (56, 93, 4)],
        ];
        let mut input_ranges = vec![(79, 14), (55, 13)];
        const EXPECTED: u32 = 46;
        let output = solve_for_ranges(&mut input_ranges, INPUT_MAPS);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_before() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (15, 4);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Unmapped((15, 4)),
            MappingResult::Nothing,
            MappingResult::Nothing,
        );
        let output = map_range(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_after() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (27, 3);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Nothing,
            MappingResult::Nothing,
            MappingResult::Unmapped((27, 3)),
        );
        let output = map_range(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_overlap_start() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (15, 7);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Unmapped((15, 5)),
            MappingResult::Mapped((10, 2)),
            MappingResult::Nothing,
        );
        let output = map_range(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_overlap_end() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (22, 5);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Nothing,
            MappingResult::Mapped((12, 3)),
            MappingResult::Unmapped((25, 2)),
        );
        let output = map_range(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_contained() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (20, 4);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Nothing,
            MappingResult::Mapped((10, 4)),
            MappingResult::Nothing,
        );
        let output = map_range(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_envelops() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (15, 15);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Unmapped((15, 5)),
            MappingResult::Mapped((10, 5)),
            MappingResult::Unmapped((25, 5)),
        );
        let output = map_range(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_with_before_relation() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (15, 4);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Unmapped((15, 4)),
            MappingResult::Nothing,
            MappingResult::Nothing,
        );
        let output = map_range_with_before_relation(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_with_after_relation() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (27, 3);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Nothing,
            MappingResult::Nothing,
            MappingResult::Unmapped((27, 3)),
        );
        let output = map_range_with_after_relation(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_with_overlap_start_relation() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (15, 7);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Unmapped((15, 5)),
            MappingResult::Mapped((10, 2)),
            MappingResult::Nothing,
        );
        let output = map_range_with_overlap_start_relation(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_with_overlap_end_relation() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (22, 5);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Nothing,
            MappingResult::Mapped((12, 3)),
            MappingResult::Unmapped((25, 2)),
        );
        let output = map_range_with_overlap_end_relation(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_with_contained_relation() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (20, 4);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Nothing,
            MappingResult::Mapped((10, 4)),
            MappingResult::Nothing,
        );
        let output = map_range_with_contained_relation(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_map_range_with_envelops_relation() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (15, 15);
        const EXPECTED: (MappingResult, MappingResult, MappingResult) = (
            MappingResult::Unmapped((15, 5)),
            MappingResult::Mapped((10, 5)),
            MappingResult::Unmapped((25, 5)),
        );
        let output = map_range_with_envelops_relation(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_relation_type_before() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (15, 5);
        const EXPECTED: RelationType = RelationType::Before;
        let output = relation_type(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_relation_type_after() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (26, 3);
        const EXPECTED: RelationType = RelationType::After;
        let output = relation_type(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_relation_type_overlap_start() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (15, 7);
        const EXPECTED: RelationType = RelationType::OverlapStart;
        let output = relation_type(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_relation_type_overlap_end() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (22, 7);
        const EXPECTED: RelationType = RelationType::OverlapEnd;
        let output = relation_type(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_relation_type_contained() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (21, 3);
        const EXPECTED: RelationType = RelationType::Contained;
        let output = relation_type(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_relation_type_envelops() {
        const INPUT_MAP: (u32, u32, u32) = (10, 20, 5);
        const INPUT_RANGE: (u32, u32) = (15, 15);
        const EXPECTED: RelationType = RelationType::Envelops;
        let output = relation_type(INPUT_RANGE, INPUT_MAP);
        assert_eq!(output, EXPECTED);
    }
}
