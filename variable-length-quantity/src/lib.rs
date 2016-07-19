pub fn to_vlq(mut value: u32) -> Vec<u8> {
    let mut output = vec![(value & 0x7f) as u8];
    value >>= 7;

    while value != 0 {
        output.push(0x80 | (value & 0x7f) as u8);
        value >>= 7;
    }

    output.reverse();

    output
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| to_vlq(*v)).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    unimplemented!()
}
