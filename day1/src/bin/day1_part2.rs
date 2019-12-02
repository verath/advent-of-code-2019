use std::str::FromStr;

fn recursive_required_fuel(module_mass: i64) -> i64 {
    let mut acc = 0;
    let mut v = day1::required_fuel(module_mass);
    while v > 0 {
        acc += v;
        v = day1::required_fuel(v);
    }
    acc
}

fn main() {
    let input = shared::read_arg_path_to_string();
    let total_required_fuel = input
        .split('\n')
        .map(|s| i64::from_str(s).unwrap())
        .map(recursive_required_fuel)
        .sum::<i64>();
    println!("{}", total_required_fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_required_fuel() {
        assert_eq!(recursive_required_fuel(14), 2);
        assert_eq!(recursive_required_fuel(1969), 966);
        assert_eq!(recursive_required_fuel(100_756), 50346);
    }
}
