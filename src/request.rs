use std::fmt::{Debug, Display};
use std::io;
use std::str::FromStr;

use crate::matrix::Matrix;

pub trait FromStrDisplayErr: FromStr<Err: Display> {}

impl<T> FromStrDisplayErr for T where T: FromStr<Err: Display> {}

pub fn value<T: FromStrDisplayErr>(message: impl Display) -> T {
    eprint!("\x1b[36;1m{message}:\x1b[0m ");

    let mut buf = String::new();

    if let Err(error) = io::stdin().read_line(&mut buf) {
        error!("failed to read the input, {error}");
        return value(message);
    };

    match buf.trim().parse() {
        Ok(parsed) => parsed,
        Err(error) => {
            error!("can't parse the input '{buf}', {error}");
            value(message)
        }
    }
}

pub fn array<T: FromStrDisplayErr>() -> Vec<T> {
    let mut buf = String::new();

    if let Err(error) = io::stdin().read_line(&mut buf) {
        error!("failed to read the input. {error}");
        buf.clear();
    };

    let mut parsed_items = Vec::new();

    for elem in buf.split_whitespace() {
        match elem.parse() {
            Ok(parsed) => parsed_items.push(parsed),
            Err(error) => error!("failed to read the input. {error}"),
        }
    }

    parsed_items
}

pub fn fixed_array<T: FromStrDisplayErr>(size: usize) -> Vec<T> {
    loop {
        let array = array();

        if array.len() != size {
            error!(
                "unexpected input array size: {}, expected {size}",
                array.len()
            )
        } else {
            break array;
        }
    }
}

pub fn matrix<T: FromStrDisplayErr + Display + Debug>() -> Matrix<T> {
    let columns = value("Input matrix rows count");
    let mut matrix = Matrix::with_rows(columns);

    for _ in 0..columns {
        matrix.push_row(array());
    }

    // TODO: print it on error only
    message!("Typed matrix:\n{matrix}",);
    matrix
}

pub fn square_matrix<T: FromStrDisplayErr + Display + Debug>() -> Matrix<T> {
    let size = value("Input matrix size");
    let mut matrix = Matrix::with_rows(size);

    for _ in 0..size {
        matrix.push_row(fixed_array(size));
    }

    message!("Typed matrix:\n{matrix}",);
    matrix
}
