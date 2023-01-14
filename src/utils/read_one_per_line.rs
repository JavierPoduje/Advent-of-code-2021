use std::fs::read_to_string;
use std::str::FromStr;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>, ()>
where
    T: FromStr,
{
    Ok(read_to_string(path)
        .unwrap()
        .split('\n')
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}
