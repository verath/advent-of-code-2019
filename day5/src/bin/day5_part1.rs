use std::str::FromStr;

fn day5_part1(input: &str) -> i64 {
    let program: Vec<i64> = input
        .split(',')
        .map(|s| i64::from_str(s).unwrap())
        .collect();
    let (_, mut output) = intcode::run(&program, &[1]);
    output.pop().unwrap()
}

fn main() {
    let input = day5::INPUT.trim_end();
    println!("{}", day5_part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1() {
        let input = day5::INPUT.trim_end();
        assert_eq!(day5_part1(input), 9_938_601);
    }
}
