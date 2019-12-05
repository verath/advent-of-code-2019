use std::str::FromStr;

fn main() {
    let input = io::read_arg_path_to_string();
    let program: Vec<i64> = input
        .split(',')
        .map(|s| i64::from_str(s).unwrap())
        .collect();
    let (_, mut output) = intcode::run(&program, &[1]);
    println!("{:?}", output.pop().unwrap());
}
