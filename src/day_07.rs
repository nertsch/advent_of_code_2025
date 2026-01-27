use std::collections::HashMap;
use std::ptr::fn_addr_eq;

pub fn part_1() -> u64 {
    get_possible_path_count(true)
}
pub fn part_2() -> u64 {
    get_possible_path_count(false) + 1
}

fn get_possible_path_count(skip_overlapping_paths: bool) -> u64 {
    let mut map = read_input();

    let start = *map.iter().find(|c| *c.1 == Field::Start).unwrap().0;
    let max_y = map.iter().map(|e| e.0.0).max().unwrap() + 1;
    trace_beam_and_get_possible_path_count(
        &mut map,
        start.0,
        start.1,
        max_y,
        skip_overlapping_paths,
    )
}

fn trace_beam_and_get_possible_path_count(
    map: &mut HashMap<(usize, usize), Field>,
    tip_y: usize,
    tip_x: usize,
    max_y: usize,
    skip_overlapping_paths: bool,
) -> u64 {
    if tip_y >= max_y {
        return 0;
    }

    let result = match map.get(&(tip_y + 1, tip_x)) {
        Some(Field::Visited {
            possible_path_count: ppc,
        }) => {
            if skip_overlapping_paths {
                0
            } else {
                *ppc
            }
        }
        None => trace_beam_and_get_possible_path_count(
            map,
            tip_y + 1,
            tip_x,
            max_y,
            skip_overlapping_paths,
        ),
        Some(Field::Splitter) => {
            1 + trace_beam_and_get_possible_path_count(
                map,
                tip_y,
                tip_x - 1,
                max_y,
                skip_overlapping_paths,
            ) + trace_beam_and_get_possible_path_count(
                map,
                tip_y,
                tip_x + 1,
                max_y,
                skip_overlapping_paths,
            )
        }
        _ => panic!(),
    };
    map.insert(
        (tip_y, tip_x),
        Field::Visited {
            possible_path_count: result,
        },
    );
    result
}

#[derive(PartialEq, Eq)]
enum Field {
    Start,
    Splitter,
    Visited { possible_path_count: u64 },
}

fn read_input() -> HashMap<(usize, usize), Field> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/input_files/day_07.txt"
    ));

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| {
                    (
                        (y, x),
                        match c {
                            'S' => Field::Start,
                            '^' => Field::Splitter,
                            _ => panic!(),
                        },
                    )
                })
        })
        .collect()
}
