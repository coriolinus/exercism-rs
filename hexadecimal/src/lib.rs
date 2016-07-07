pub fn hex_to_int(hex: &str) -> Option<usize> {
    let mut result: usize = 0;
    let mut position: u32 = 0;
    for ch in hex.chars().rev() {
        let value = match ch {
            '0'...'9' => ch as u8 - 48, // produces 0-9
            'A'...'F' => ch as u8 - 55, // produces 10-16
            'a'...'f' => ch as u8 - 87, // produces 10-16
            _ => return None
        };
        result += value as usize * (16_usize.pow(position));
        position += 1;
    }
    Some(result)
}
