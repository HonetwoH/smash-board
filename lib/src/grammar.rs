use crate::config::Base;

#[derive(Debug)]
pub enum ParsingErrors {
    HigherOrderNumber,
}

//TODO: give some special meaning to each symbol add feature for graceful error returns
pub fn check(cap: Base) -> impl Fn(&str) -> Vec<Result<u8, ParsingErrors>> {
    move |line: &str| {
        let points = line.as_bytes();
        let mut buffers = vec![];

        let ignore = |x: u8| {
            let redundant = [b'.', b' ', b','];
            let mut found = false;
            for i in redundant {
                found |= if i == x { true } else { false };
            }
            found
        };
        for token in points {
            if ignore(*token) {
                continue;
            }
            // this should work and yeild only a single digit number for the given base
            if let Some(number) = char::from(*token).to_digit(cap as u32) {
                buffers.push(Ok(number as u8));
            } else {
                buffers.push(Err(ParsingErrors::HigherOrderNumber));
            }
        }
        buffers
    }
}
