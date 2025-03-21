use std::fmt::{Debug, Display};
use std::io;
use std::str::FromStr;

use crate::matrix::Matrix;

pub fn value<T>(message: impl Display) -> T
where
    T: FromStr,
    T::Err: Display,
{
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

pub fn array<T>() -> Vec<T>
where
    T: FromStr,
    T::Err: Display,
{
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

pub fn array_fixed<T>(size: usize) -> Vec<T>
where
    T: FromStr + Display,
    T::Err: Display,
{
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

pub fn matrix<T>() -> Matrix<T>
where
    T: FromStr + Display + Debug,
    T::Err: Display,
{
    let columns = value::<usize>("Input matrix rows count");
    let mut matrix = Matrix::with_capacity(columns);

    for _ in 0..columns {
        matrix.push_row(array());
    }

    message!("Typed matrix:\n{matrix}",);
    matrix
}

pub fn square_matrix<T>() -> Matrix<T>
where
    T: FromStr + Display + Debug,
    T::Err: Display,
{
    let size = value::<usize>("Input matrix size");
    let mut matrix = Matrix::with_capacity(size);

    for _ in 0..size {
        matrix.push_row(array_fixed(size));
    }

    message!("Typed matrix:\n{matrix}",);
    matrix
}
