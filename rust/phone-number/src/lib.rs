pub fn number(user_number: &str) -> Option<String> {
    let mut number = user_number
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<Vec<_>>();

    // remove country code
    if number.len() == 11 && number[0] == '1' {
        number.remove(0);
    }

    // NXXNXXXXXX: N is any digit from 2 through 9
    if number.len() != 10 {
        return None;
    }
    if let '0' | '1' = number[0] {
        return None;
    }
    if let '0' | '1' = number[3] {
        return None;
    }

    Some(number.into_iter().collect())
}
