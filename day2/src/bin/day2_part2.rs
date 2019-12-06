use std::str::FromStr;

fn day2_part2(input: &str) -> i64 {
    let orig_program: Vec<i64> = input
        .split(',')
        .map(|s| i64::from_str(s).unwrap())
        .collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = orig_program.clone();
            program[1] = noun;
            program[2] = verb;
            let (program, _) = intcode::run(&program, &[]);
            let output = program[0];
            if output == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("no solution");
}

fn main() {
    let input = day2::INPUT.trim_end();
    println!("{}", day2_part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part2() {
        let input = day2::INPUT.trim_end();
        assert_eq!(day2_part2(input), 2003);
    }
}
