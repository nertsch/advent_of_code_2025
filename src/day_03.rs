pub fn part_1() -> u64 {
    calculate_total_output_joltage(2)
}
pub fn part_2() -> u64 {
    calculate_total_output_joltage(12)
}
pub fn calculate_total_output_joltage(number_of_batteries: usize) -> u64 {
    let mut total_output_joltage = 0;

    for bank in parse_input() {
        let mut start_index = 0;
        for position in (0..number_of_batteries).rev()
        {
            let (index, joltage_high) = bank
                .iter()
                .enumerate()
                .skip(start_index)
                .take(bank.len() - start_index - position)
                .rev()
                .max_by_key(|(_, value)| *value)
                .map(|(index, value)| (index, *value))
                .unwrap();
            total_output_joltage += (joltage_high as u64) * 10u64.pow((position) as u32);
            start_index = index+1;
        }
    }

    total_output_joltage
}

fn parse_input() -> Vec<Vec<u32>> {
    let input = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input_files/day_03.txt"
    ));

    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}