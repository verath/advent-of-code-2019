use std::str::FromStr;

fn main() {
    let input = io::read_arg_path_to_string();
    let mut program: Vec<u64> = input
        .split(',')
        .map(|s| u64::from_str(s).unwrap())
        .collect();
    program[1] = 12;
    program[2] = 2;
    day2::run(&mut program);
    println!("{:?}", program[0]);
}
