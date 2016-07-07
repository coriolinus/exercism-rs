pub fn hex_to_int(hex: &str) -> Option<usize> {
    let mut result = 0;
    let mut position = 0;
    for ch in hex.chars().rev() {
        let value = match ch {
            '0'...'9' => ch as u8 - '0' as u8, // produces 0-9
            'A'...'F' => ch as u8 - 'A' as u8 + 10, // produces 10-16
            'a'...'f' => ch as u8 - 'a' as u8 + 10, // produces 10-16
            _ => return None
        } as usize;
        result += value  * (16_usize.pow(position));
        position += 1;
    }
    Some(result)
}
