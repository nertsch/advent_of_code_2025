use std::vec::Vec;

pub fn part_1_and_2() -> (u32, i32) {
    let mut current_position = 50;
    let mut number_of_hit_zeros = 0;
    let mut number_of_passed_zeros = 0;

    for number_of_clicks in parse_input() {
        if number_of_clicks < 0 {
            // calculate the position of a hypothetical dial,
            // has the same distance to the right that the regular dial has from the left
            let current_position_of_right_turn_only_dial = (100 - current_position) % 100;
            number_of_passed_zeros +=
                (current_position_of_right_turn_only_dial + -number_of_clicks) / 100;
        } else {
            number_of_passed_zeros += (current_position + number_of_clicks) / 100;
        }
        current_position = (current_position + number_of_clicks).rem_euclid(100);
        number_of_hit_zeros += (current_position == 0) as u32;
    }
    (number_of_hit_zeros, number_of_passed_zeros)
}

fn parse_input() -> Vec<i32> {
    let input = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input_files/day_01.txt"
    ));

    input
        .lines()
        .map(|l| l.split_at(1))
        .map(|(direction, number_of_clicks)| {
            let number_of_clicks = number_of_clicks.parse::<i32>().unwrap();
            if direction == "L" {
                -number_of_clicks
            } else {
                number_of_clicks
            }
        })
        .collect()
}