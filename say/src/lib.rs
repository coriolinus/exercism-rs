static FIRST_TWENTY: [&'static str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

static TENS: [&'static str; 8] = [
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];

const HUNDRED: &'static str = "hundred";

static EXPONENT_3: [&'static str; 6] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];


pub fn encode(input: u64) -> String {
    if input == 0 {
        FIRST_TWENTY[0].to_string()
    } else {
        let mut output_segments = Vec::new();
        for (thousands_power, triplet) in to_digits(input).chunks(3).enumerate().rev() {
            if let Some(trip_said) = say_triplet(triplet) {
                output_segments.push(trip_said);
                if thousands_power != 0 && triplet.iter().any(|&d| d != 0) {
                    output_segments.push(EXPONENT_3[thousands_power - 1].to_string())
                }
            }
        }
        output_segments.join(" ")
    }
}

type Digits = Vec<u8>; // position 0 is the right-most digit

fn to_digits(input: u64) -> Digits {
    input
        .to_string()
        .chars()
        .rev()
        // unwrap is safe here because we get these values from .to_string,
        // which produces base-10 digits
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect()
}

fn say_triplet(triplet: &[u8]) -> Option<String> {
    let mut output_segments = Vec::new();
    if triplet.len() >= 3 && triplet[2] != 0 {
        // hundreds
        output_segments.push(format!("{} {}", FIRST_TWENTY[triplet[2] as usize], HUNDRED));
    }
    if triplet.len() >= 2 {
        let two_digits = ((10 * triplet[1]) + triplet[0]) as usize;
        if let Some(val) = match two_digits {
            0 => None,
            1...19 => Some(FIRST_TWENTY[two_digits].to_string()),
            _ => {
                if triplet[0] == 0 {
                    Some(TENS[triplet[1] as usize - 2].to_string())
                } else {
                    Some(format!(
                        "{}-{}",
                        TENS[triplet[1] as usize - 2],
                        FIRST_TWENTY[triplet[0] as usize]
                    ))
                }
            }
        }
        {
            output_segments.push(val);
        };
    } else if triplet.len() == 1 {
        output_segments.push(FIRST_TWENTY[triplet[0] as usize].to_string());
    }
    if output_segments.len() > 0 {
        Some(output_segments.join(" "))
    } else {
        None
    }
}
