fn meet_criteria(password: u64) -> bool {
    let password_str = password.to_string();
    let mut has_same_adjacent = false;
    let mut prev_char: Option<char> = None;
    for curr_char in password_str.chars() {
        if let Some(prev_char) = prev_char {
            if prev_char == curr_char {
                has_same_adjacent = true;
            } else if curr_char < prev_char {
                // If not same, digit must increase
                return false;
            }
        }
        prev_char = Some(curr_char);
    }
    has_same_adjacent
}

fn main() {
    let range = 24_0298..=784_956;
    let num_matches = range.filter(|&v| meet_criteria(v)).count();
    println!("{}", num_matches);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meet_criteria() {
        assert_eq!(meet_criteria(111_111), true);
        assert_eq!(meet_criteria(223_450), false);
        assert_eq!(meet_criteria(123_789), false);
    }
}
