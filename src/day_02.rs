pub fn part_1() -> u64 {
    let mut result = 0;

    for (from, to) in parse_input() {
        for i in from..=to {
            let s = i.to_string();
            if !is_valid(&s, 2) {
                result += i;
            }
        }
    }

    result
}

pub fn part_2() -> u64 {
    let mut result = 0;

    for (from, to) in parse_input() {
        for i in from..=to {
            let s = i.to_string();
            'check_loop: for number_of_chunks in 2..=s.len() {
                if !is_valid(&s, number_of_chunks) {
                    result += i;
                    break 'check_loop;
                }
            }
        }
    }

    result
}

fn is_valid(number: &str, number_of_chunks: usize) -> bool {
    // len and treating number as bytes is safe, since numbers are just ASCII
    if number.len() % number_of_chunks != 0 {
        return true;
    }
    let sequence_length = number.len() / number_of_chunks;
    let mut chunks = number.as_bytes().chunks(sequence_length);
    let first_chunk = chunks.next().unwrap();
    for chunk in chunks {
        if chunk != first_chunk {
            return true;
        }
    }
    false
}

fn parse_input() -> Vec<(u64, u64)> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/input_files/day_02.txt"
    ));

    input
        .split(",")
        .map(|r| r.split_once("-").unwrap())
        .map(|(from, to)| (from.parse().unwrap(), to.parse().unwrap()))
        .collect()
}
