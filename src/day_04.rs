use std::collections::HashSet;
use std::mem;

pub fn part_1() -> usize {
    let mut number_of_removable_rolls = 0;
    let roll_locations = parse_input();
    for roll_location in roll_locations.iter() {
        if is_removable(roll_location, &roll_locations) {
            number_of_removable_rolls += 1;
        }
    }
    number_of_removable_rolls
}

pub fn part_2() -> usize {
    let mut number_of_removable_rolls = 0;
    let mut roll_locations = parse_input();
    let mut updated_roll_locations = HashSet::new();

    loop {
        for roll_location in roll_locations.iter() {
            if is_removable(roll_location, &roll_locations) {
                number_of_removable_rolls += 1;
            } else {
                updated_roll_locations.insert(*roll_location);
            }
        }
        if roll_locations.len() == updated_roll_locations.len() {
            break;
        }
        roll_locations.clear();
        mem::swap(&mut roll_locations, &mut updated_roll_locations);
    }

    number_of_removable_rolls
}

fn is_removable(roll_location: &(i32, i32), roll_locations: &HashSet<(i32, i32)>) -> bool {
    let number_of_adjacent_rolls =
        roll_locations.contains(&(roll_location.0 - 1, roll_location.1 - 1)) as usize
            + roll_locations.contains(&(roll_location.0, roll_location.1 - 1)) as usize
            + roll_locations.contains(&(roll_location.0 + 1, roll_location.1 - 1)) as usize
            + roll_locations.contains(&(roll_location.0 - 1, roll_location.1)) as usize
            + roll_locations.contains(&(roll_location.0 + 1, roll_location.1)) as usize
            + roll_locations.contains(&(roll_location.0 - 1, roll_location.1 + 1)) as usize
            + roll_locations.contains(&(roll_location.0, roll_location.1 + 1)) as usize
            + roll_locations.contains(&(roll_location.0 + 1, roll_location.1 + 1)) as usize;
    number_of_adjacent_rolls < 4
}

fn parse_input() -> HashSet<(i32, i32)> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/input_files/day_04.txt"
    ));

    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '@')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect()
}
