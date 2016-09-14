// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

pub mod multi_zip;
pub mod numbers;

use multi_zip::mzip;
use numbers::{CHAR_H, CHAR_W, match_lchar};

#[derive(Debug)]
pub enum ConversionStatus {
    Success,
    NotEnoughRows,
    NotEnoughColumns,
}

pub fn convert(input: &str) -> Result<String, ConversionStatus> {
    // break up the input into a Vec<Vec<char>>
    let lines: Vec<_> = input.split('\n').map(|line| line.chars().collect::<Vec<_>>()).collect();

    let expected_chars = input.len() / (CHAR_H * CHAR_W);
    let expected_lines = lines.len() / CHAR_H;

    let mut output = String::with_capacity(expected_chars + expected_lines);

    for logical_row in lines.chunks(CHAR_H) {
        if logical_row.len() != CHAR_H {
            return Err(ConversionStatus::NotEnoughRows);
        }

        // We've divided the logical row into chunks of appropriate height.
        // Now, the task is to iterate simultaneously through all chunks of appropriate width,
        // and then pass those in to the character-identifying function.

        // Now transform the logical row into an iterator over logical characters
        // First transform row vectors to chunked iterators
        let logical_row: Vec<_> = logical_row.iter().map(|row| row.chunks(CHAR_W)).collect();

        for lchar in mzip(logical_row) {
            // lchar is of type Vec<Chunk>, and should be addressed row-first.
            if lchar[0].len() != CHAR_W {
                return Err(ConversionStatus::NotEnoughColumns);
            }

            // match the characters here
            output.push(match_lchar(lchar));
        }

        output.push(',');

    }

    // remove the last char from the output to get rid of the trailing \n
    output.pop();

    Ok(output)
}



pub fn has_valid_shape(input: &str) -> bool {
    let lines = input.split('\n');
    let rows_work = lines.clone().count() % CHAR_H == 0;
    let cols_work = lines.clone().all(|line| line.len() % CHAR_W == 0);
    let first_line_len = lines.clone().next().map(|line| line.len());
    let cols_match = lines.fold(true, |acc, line| acc && Some(line.len()) == first_line_len);
    rows_work && cols_work && cols_match
}
