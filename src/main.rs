mod config {
    // this modules will read and interpret config which is
    // as of now just the number of available buffers

    pub enum Base {
        Hexa,
        Octal,
        Decimal,
        HexaDecimal,
    }

    struct Config {
        available_buffers: Base,
    }
}

mod clipboard_sync {
    // will need to poll for changes in hopes on not hogging the cpu
}
mod grammer {
    // the core grammer
    use crate::config::Base;

    pub fn check(cap: Base) -> impl Fn(&str) -> Vec<u8> {
        let radix = match cap {
            Base::Hexa => 6,
            Base::Octal => 8,
            Base::Decimal => 10,
            Base::HexaDecimal => 16,
        };

        move |line: &str| {
            let points = line.as_bytes();
            let mut buffers = vec![];

            let ignore = |x: u8| {
                let x = char::from_u32(x as u32).unwrap();
                let redundant = ['.', ' ', ','];
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
                if let Some(number) = char::from(*token).to_digit(radix) {
                    // dbg!(number);
                    buffers.push(number as u8);
                } else {
                    panic!("Number higher than the Base");
                }
            }
            buffers
        }
    }
}

mod cli;
mod composer;
mod db;

fn main() {
    use crate::cli::{argument, Action};
    use crate::composer::compose_ui;
    use crate::config::Base;
    use crate::db::Db;
    use crate::grammer::check;

    let parser = check(Base::Octal);
    let pastes_db: Db = Db::new_connection().unwrap();
    match argument().unwrap() {
        // Action::Show => pastes_db.show(8),
        Action::Compose => {
            let items = pastes_db
                .fetch(Vec::from_iter(0..8))
                .into_iter()
                .map(|x| x.unwrap_or_default())
                .collect();
            _ = compose_ui(items, parser);
        }
        Action::Paste(from_buffers) => {
            let buffers = parser(&from_buffers)
                .into_iter()
                .map(|x| x as usize)
                .collect();
            pastes_db
                .fetch(buffers)
                .into_iter()
                .map(|x| x.unwrap_or_default())
                .for_each(|x| println!("{}", x));
        }
        Action::Copy(new) => {
            dbg!(&new);
            pastes_db.push(new).unwrap();
        }
        _ => panic!(),
    }
}
