use std::str::FromStr;

fn day2_part1(input: &str) -> i64 {
    let mut program: Vec<i64> = input
        .split(',')
        .map(|s| i64::from_str(s).unwrap())
        .collect();
    program[1] = 12;
    program[2] = 2;
    let (program, _) = intcode::run(&program, &[]);
    program[0]
}

fn main() {
    let input = day2::INPUT.trim_end();
    println!("{:?}", day2_part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        let input = day2::INPUT.trim_end();
        assert_eq!(day2_part1(input), 12_490_719);
    }
}
