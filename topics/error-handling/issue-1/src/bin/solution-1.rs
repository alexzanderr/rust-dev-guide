//! this file is the answer for this question
//! In Python you can throw your own Exceptions, say `if "file doesn't exist" throw MyOwnCustomException`, but how would you do that in Rust?

#[derive(thiserror::Error, Debug)]
enum ReadNumberError {
    #[error("read line error")]
    ReadLine(#[from] std::io::Error),

    #[error("parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
}

// use your custom error types with thiserror crate
fn read_number() -> std::result::Result<i32, ReadNumberError> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    let number = buffer.trim().parse::<i32>()?;

    Ok(number)
}

fn main() {
    let result = read_number();
    dbg!(&result);
}
