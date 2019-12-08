fn main() {
    let input = "307237-769058";
    let range_strings: Vec<&str> = input.split("-").collect();
    let range_start: i32 = range_strings[0].parse().unwrap();
    let range_end: i32 = range_strings[1].parse().unwrap();

    let mut counter = 0;
    for password in range_start..range_end + 1 {
        if no_digits_are_in_decreasing_order(password.to_string()) &&
           two_adjacent_digits_are_the_same(password.to_string()) {
            counter += 1;
        }
    }

    println!("{} different passwords", counter);
}

fn no_digits_are_in_decreasing_order(password: String) -> bool {
    let chars: Vec<char> = password.chars().collect();
    let mut sorted_chars = chars.clone();
    sorted_chars.sort();
    return chars == sorted_chars;
}

fn two_adjacent_digits_are_the_same(password: String) -> bool {
    let mut previous_char = None;

    for character in password.chars() {
        if let Some(previous_char_value) = previous_char {
            if previous_char_value == character {
                return true;
            }
        }
        previous_char = Some(character);
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_adjacent_digits_are_the_same() {
        assert!(two_adjacent_digits_are_the_same(String::from("112345")));
        assert!(!two_adjacent_digits_are_the_same(String::from("123456")));
    }

    #[test]
    fn test_no_digits_are_in_decreasing_order() {
        assert!(no_digits_are_in_decreasing_order(String::from("112345")));
        assert!(no_digits_are_in_decreasing_order(String::from("123456")));
        assert!(!no_digits_are_in_decreasing_order(String::from("123465")));
    }
}
