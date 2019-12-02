use std::str::FromStr;

fn main() {
    let input = shared::read_arg_path_to_string();
    let orig_program: Vec<u64> = input
        .split(',')
        .map(|s| u64::from_str(s).unwrap())
        .collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = orig_program.clone();
            program[1] = noun;
            program[2] = verb;
            day2::run(&mut program);
            let output = program[0];
            if output == 19690720 {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}
