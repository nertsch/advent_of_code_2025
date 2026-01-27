use std::ops::RangeInclusive;

pub fn part_1() -> usize {
    let mut number_of_fresh_ingredients = 0;
    let (valid_ranges, ids) = parse_input();

    for ref ingredient_id in ids {
        for valid_range in valid_ranges.iter() {
            if valid_range.contains(ingredient_id) {
                number_of_fresh_ingredients += 1;
                break;
            }
        }
    }

    number_of_fresh_ingredients
}

pub fn part_2() -> usize {
    let (mut valid_ranges, _) = parse_input();
    valid_ranges.sort_by_key(|r| *r.start());
    let mut merged_ranges = Vec::new();
    let mut valid_ranges = valid_ranges.into_iter();
    let mut current_range = valid_ranges.next().unwrap();

    for next_range in valid_ranges {
        if current_range.end() < next_range.start() {
            merged_ranges.push(current_range);
            current_range = next_range;
        } else if current_range.end() < next_range.end() {
            current_range = *current_range.start()..=*next_range.end();
        }
    }
    merged_ranges.push(current_range);

    merged_ranges
        .into_iter()
        .map(|r| r.end() - r.start() + 1)
        .sum()
}

fn parse_input() -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/input_files/day_05.txt"
    ));

    let mut lines = input.lines();

    let ranges = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let bounds = l.split_once("-").unwrap();
            bounds.0.parse::<usize>().unwrap()..=bounds.1.parse::<usize>().unwrap()
        })
        .collect::<Vec<RangeInclusive<usize>>>();

    let ids = lines.map(|l| l.parse::<usize>().unwrap()).collect();

    (ranges, ids)
}
