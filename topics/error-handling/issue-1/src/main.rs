#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
    non_upper_case_globals,
    non_camel_case_types,
    semicolon_in_expressions_from_macros,
    redundant_semicolons,
    unused_macros
)]

fn simplest_example() -> std::result::Result<String, std::io::Error> {
    let file = match std::fs::read_to_string("file.txt") {
        Ok(value) => value,
        Err(error) => {
            // early return here
            return Err(error);
        },
    };

    // some other logic here

    Ok(file)
}

fn main() {
    let result = simplest_example();
    dbg!(&result);
}
