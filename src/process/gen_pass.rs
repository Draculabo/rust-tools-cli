use rand::seq::SliceRandom;
use rand::thread_rng;
const UPPER: &[u8] = b"ABCDEFGHIJKMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*()_";
pub fn process_genpass(
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
    length: u8,
) -> anyhow::Result<()> {
    let mut password = Vec::new();
    let mut chars = Vec::new();
    let mut rng = thread_rng();
    if uppercase {
        chars.extend(UPPER);
        password.push(*UPPER.choose(&mut rng).unwrap());
    }
    if lowercase {
        chars.extend(LOWER);
        password.push(*LOWER.choose(&mut rng).unwrap());
    }
    if number {
        chars.extend(NUMBER);
        password.push(*NUMBER.choose(&mut rng).unwrap());
    }
    if symbol {
        chars.extend(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).unwrap());
    }
    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).unwrap();
        password.push(*c);
    }
    password.shuffle(&mut rng);
    let value = String::from_utf8(password).unwrap();
    println!("{}", value);
    let score = zxcvbn::zxcvbn(&value, &[]);
    println!("{:?}", score.score());
    Ok(())
}
