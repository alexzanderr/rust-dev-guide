Rust differs from other programming languages on the topic of error handling. Where other languages put exceptions in one big pile, Rust makes a distinction between 'recoverable' and 'unrecoverable' errors.

Recoverable errors are the kind of errors that we are familiar with:
    - making an HTTP request when the URL is invalid or when there is no internet.
    - trying to read from a file that doesn't exist.

These kinds of errors can be recovered from because we can show the user a friendly error message and carry on with doing other stuff like rendering an interface.

Unrecoverable errors are a sign of bad programming like accessing a buffer beyond its capacity or they are the result of an action that was tried and cannot be repared like accessing a protected file (there is nothing we can do if we don't have read and write access to file 'x.txt').

Yet again, Rust differs because it doesn't have an exception system like the one that we are familiar with: 'try-catch' and 'throw Exception'. Instead, we are given a 'Result' enum that encapsulates the successful result of an operation in 'Ok', or the error upon a failure in 'Err'.

--- Python

try:
    file = open("file.txt", "r")
except FileNotFoundError:
    file = open("file.txt", "w")

The exception system in Python is simple and straightforward. In the above example, 'open' in read mode throws a 'FileNotFoundError' is the file doesn't exist.

--- Rust
let file = match fs::read_to_string("file.txt") {
    Ok(value) => value,
    Err(error) => match File::create("file.txt") {
        Ok(_) => {}
        Err(_) => panic!(),
    },
};

I like this approach better because a 'Result' you can always return from a function, unlike traditional exceptions where you were usually returing the success but if you ever encountered an exception you had to think really hard what to do with it later down the line - propagate it, resolve it or wrap in your own custom exception. Something more, from analysing the code, you can see exactly what part it's going to return a Result (and possibly an error) which makes reasoning about logic much easier and natural.

So far so good. Now what is the problem that I have with this?

In Python you can throw your own Exceptions, say 'if "file doesn't exist" throw MyOwnCustomException', but how would you do that in Rust?

So far, I've only been shown methods that return Result but never created my own logic that returns Result.

Can you give me an example of a Rust program that checks for some logic and returns result?
