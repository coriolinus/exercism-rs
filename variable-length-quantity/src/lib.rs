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

pub fn from_vlq(bytes: &[u8]) -> Result<u32, &'static str> {
    let mut value: u32 = 0;
    for (i, byte) in bytes.iter().enumerate() {
        value |= (byte & 0x7f) as u32;
        if i == bytes.len() - 1 {
            if byte & 0x80 != 0 {
                return Err("Trailing byte lacks initial 0");
            }
        } else {
            if byte & 0x80 == 0 {
                return Err("A leading byte lacks initial 1");
            }
            if value.leading_zeros() < 7 {
                return Err("Overflow constructing u32 from bytes");
            } else {
                value <<= 7;
            }
        }
    }
    Ok(value)
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut start = 0;
    let mut end = 0;
    let mut output = Vec::new();
    while start < bytes.len() {
        while end < bytes.len() && bytes[end] & 0x80 != 0 {
            end += 1;
        }
        if end == bytes.len() {
            return Err("Trailing byte lacks initial 0");
        }
        output.push(try!(from_vlq(&bytes[start..end + 1])));
        start = end + 1;
        end = start;
    }
    Ok(output)
}
