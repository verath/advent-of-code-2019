use std::env;
use std::fs;

pub fn read_arg_path_to_string() -> String {
    let file_path = env::args().nth(1).expect("file_path param missing");
    let mut input = fs::read_to_string(file_path).unwrap();
    // Drop trailng newline
    let trailing_newline = input.pop();
    debug_assert!(trailing_newline == Some('\n'));
    input
}
