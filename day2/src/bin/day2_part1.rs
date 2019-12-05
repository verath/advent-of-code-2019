use std::str::FromStr;

fn main() {
    let input = io::read_arg_path_to_string();
    let mut program: Vec<i64> = input
        .split(',')
        .map(|s| i64::from_str(s).unwrap())
        .collect();
    program[1] = 12;
    program[2] = 2;
    let (program, _) = intcode::run(&program, &[]);
    println!("{:?}", program[0]);
}
