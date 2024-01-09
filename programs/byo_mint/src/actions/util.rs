pub fn is_valid_hex_color(input: &str) -> bool {
    // Check if the input starts with '#' and has a length of 7
    if input.len() == 7 && input.starts_with('#') {
        // Check if all characters following '#' are hexadecimal
        input[1..].chars().all(|c| c.is_digit(16))
    } else {
        false
    }
}