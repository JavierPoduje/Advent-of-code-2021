use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> i32 {
    let rows: Vec<Row> = read_one_per_line::<String>("./src/day_8/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| Row::new(row))
        .collect();

    let mut count = 0;
    for row in rows {
        let output = &row.output;
        for str_digit in output {
            match str_digit.len() {
                2 | 4 | 3 | 7 => count += 1,
                _ => {}
            }
        }
    }

    count
}

#[derive(Debug)]
struct Row {
    input: Vec<String>,
    output: Vec<String>,
}

impl Row {
    pub fn new(raw_line: String) -> Self {
        let pairs = raw_line
            .split("|")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|pair| {
                pair.split(" ")
                    .filter(|value| !value.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect::<Vec<Vec<&str>>>();

        let mut input: Vec<Vec<String>> = Vec::new();
        let mut output: Vec<Vec<String>> = Vec::new();
        for (idx, pair) in pairs.into_iter().enumerate() {
            let response = pair
                .into_iter()
                .map(|value| {
                    let mut sorted = value.chars().collect::<Vec<char>>();
                    sorted.sort_by(|a, b| a.cmp(b));
                    String::from_iter(sorted)
                })
                .collect::<Vec<_>>();

            if idx == 0 {
                input.push(response);
            } else {
                output.push(response);
            }
        }

        Self {
            input: input[0].clone(),
            output: output[0].clone(),
        }
    }
}
