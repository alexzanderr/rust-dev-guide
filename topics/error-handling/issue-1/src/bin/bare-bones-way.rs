use std::io::ErrorKind;
use std::num::ParseIntError;

impl From<ParseIntError> for std::io::Error {
    fn from(error: ParseIntError) {
        // second argument is the message
        Self::new(ErrorKind::Other, error.to_string())
    }
}

// use your custom error types with thiserror crate
fn read_number() -> std::io::Result<i32> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    let number = buffer.trim().parse::<i32>()?;

    Ok(123)
}

fn main() {
    let result = read_number();
    dbg!(&result);
}
