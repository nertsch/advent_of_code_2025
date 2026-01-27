use std::collections::HashMap;
pub fn part_1() -> u32 {
    let mut map = read_input();

    let start_x = map[0].iter().enumerate().find(|c| *c.1 == 'S').unwrap().0;
    trace_beam_and_get_splits(&mut map, 0, start_x)
}

fn trace_beam_and_get_splits(map: &mut Vec<Vec<char>>, tip_y: usize, tip_x: usize) -> u32 {
    if tip_y >= map.len() - 1 {
        return 0;
    }
    match map[tip_y + 1][tip_x] {
        '|' => return 0,
        '.' => {
            map[tip_y + 1][tip_x] = '|';
            return trace_beam_and_get_splits(map, tip_y + 1, tip_x);
        }
        '^' => {
            return 1
                + trace_beam_and_get_splits(map, tip_y, tip_x - 1)
                + trace_beam_and_get_splits(map, tip_y, tip_x + 1);
        }
        _ => panic!(),
    }
}
pub fn part_2() -> u64 {
    let mut map = read_input();

    let start_x = map[0].iter().enumerate().find(|c| *c.1 == 'S').unwrap().0;
    let mut result_cache = HashMap::new();
    trace_beam_and_get_possible_path_count(&mut map, 0, start_x, &mut result_cache) + 1
}

fn trace_beam_and_get_possible_path_count(
    map: &mut Vec<Vec<char>>,
    tip_y: usize,
    tip_x: usize,
    result_cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if tip_y >= map.len() - 1 {
        return 0;
    }

    if let Some(&value) = result_cache.get(&(tip_y, tip_x)) {
        return value;
    }

    let result = match map[tip_y + 1][tip_x] {
        '|' => 0,
        '.' => trace_beam_and_get_possible_path_count(map, tip_y + 1, tip_x, result_cache),
        '^' => {
            1 + trace_beam_and_get_possible_path_count(map, tip_y, tip_x - 1, result_cache)
                + trace_beam_and_get_possible_path_count(map, tip_y, tip_x + 1, result_cache)
        }
        _ => panic!(),
    };
    result_cache.insert((tip_y, tip_x), result);
    result
}

fn read_input() -> Vec<Vec<char>> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/input_files/day_07.txt"
    ));

    input.lines().map(|l| l.chars().collect()).collect()
}
