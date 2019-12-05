use std::str::FromStr;

fn main() {
    let input = io::read_arg_path_to_string();
    let total_required_fuel = input
        .split('\n')
        .map(|s| i64::from_str(s).unwrap())
        .map(day1::required_fuel)
        .sum::<i64>();
    println!("{}", total_required_fuel);
}
