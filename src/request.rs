use std::fmt::Display;
use std::io;
use std::str::FromStr;

use crate::matrix::Matrix;
use crate::request; // for self-referencing

pub fn value<T: FromStr<Err: Display>>(message: impl Display) -> T {
    eprint!("\x1b[36;1m{message}:\x1b[0m ");

    let mut buf = String::new();

    if let Err(error) = io::stdin().read_line(&mut buf) {
        error!("failed to read the input, {error}");
        return request::value(message);
    };

    match buf.trim().parse() {
        Ok(parsed) => parsed,
        Err(error) => {
            error!("can't parse the input '{buf}', {error}");
            request::value(message)
        }
    }
}

pub fn array<T: FromStr<Err: Display>>(was_error: &mut bool) -> Vec<T> {
    let mut buf = String::new();

    'read: loop {
        if let Err(error) = io::stdin().read_line(&mut buf) {
            error!("failed to read the input. {error}");
            *was_error = true;
            buf.clear();
            continue 'read;
        };

        let mut parsed_items = Vec::new();

        for elem in buf.split_whitespace() {
            match elem.parse() {
                Ok(parsed) => parsed_items.push(parsed),
                Err(error) => {
                    error!("failed to parse the input. {error}");
                    *was_error = true;
                    buf.clear();
                    continue 'read;
                }
            }
        }

        break 'read parsed_items;
    }
}

pub fn fixed_array<T: FromStr<Err: Display>>(size: usize, was_error: &mut bool) -> Vec<T> {
    loop {
        let array = request::array(was_error);

        if array.len() == size {
            break array;
        }

        error!(
            "unexpected input array size: {}, expected {size}",
            array.len()
        );
        *was_error = true;
    }
}

pub fn matrix<T: FromStr<Err: Display> + Display>() -> Matrix<T> {
    let columns = value("Input matrix rows count");
    let mut matrix = Matrix::with_rows(columns);
    let mut was_error = false;

    for _ in 0..columns {
        let array = request::array(&mut was_error);
        matrix.push_row(array);
    }

    if was_error {
        message!("Typed matrix:\n{matrix}",);
    }

    matrix
}

pub fn square_matrix<T: FromStr<Err: Display> + Display>() -> Matrix<T> {
    let size = value("Input matrix size");
    let mut matrix = Matrix::with_rows(size);
    let mut was_error = false;

    for _ in 0..size {
        let array = request::fixed_array(size, &mut was_error);
        matrix.push_row(array);
    }

    if was_error {
        message!("Typed matrix:\n{matrix}",);
    }

    matrix
}
