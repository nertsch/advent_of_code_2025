pub fn part_1() -> i64 {
    let input = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input_files/day_06.txt"
    ));

    let lines = input.lines().collect::<Vec<&str>>();

    let numbers: Vec<Vec<i64>> = lines
        .iter()
        .take(lines.len() - 1)
        .map(|l| {
            l.split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let operators = lines[lines.len() - 1]
        .chars()
        .filter(|n| !n.is_whitespace())
        .collect::<Vec<char>>();

    let mut result = 0;
    for i in 0..operators.len() {
        if operators[i] == '+' {
            result += numbers[0][i] + numbers[1][i] + numbers[2][i] + numbers[3][i];
        } else {
            result += numbers[0][i] * numbers[1][i] * numbers[2][i] * numbers[3][i];
        }
    }
    result
}

pub fn part_2() -> i64 {
    let input_str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/input_files/day_06.txt"
    ));

    let input_matrix: Vec<Vec<char>> = input_str.lines().map(|l| l.chars().collect()).collect();


    let mut current_operands = Vec::new();
    let mut result = 0;
    let mut x = input_matrix[0].len() - 1;
    loop {
        current_operands.push(read_number(&input_matrix, x));
        if input_matrix[4][x] == '+' {
            result += current_operands.iter().sum::<i64>();
            current_operands.clear();
            if x == 0 { break; }
            x -= 1;
        } else if input_matrix[4][x] == '*' {
            result += current_operands.iter().fold(1, |acc, v| acc * v);
            current_operands.clear();
            if x == 0 { break; }
            x -= 1;
        }
        x -= 1;
    }
    result
}

fn read_number(matrix: &Vec<Vec<char>>, column: usize) -> i64 {
    let mut s = String::new();
    s.push(matrix[0][column]);
    s.push(matrix[1][column]);
    s.push(matrix[2][column]);
    s.push(matrix[3][column]);
    s.trim().parse().unwrap()
}
