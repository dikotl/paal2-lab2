//! Завдання:
//!
//! 1. Підрахувати кількість від’ємних елементів матриці.
//!
//! 2. Обміняти місцями відповідні елементи першого (технічно 0-го) рядка і
//!    головної діагоналі; вважати, що матриця гарантовано квадратна.
//!
//! 3. Упорядкувати побічну діагональ матриці від мінімального праворуч-угорі до
//!    максимального ліворуч-унизу; вважати, що матриця гарантовано квадратна.
//!
//! 4. Упорядкувати стовпчики матриці за неспаданням мінімального елемента.

use itertools::Itertools;

use std::str::FromStr;
use std::{fmt, io};

use Task::*;

fn main() {
    select_task();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Task {
    Task1,
    Task2,
    Task3,
    Task4,
}

macro_rules! message {
    () => {
        std::eprintln!()
    };
    ($($arg:tt)*) => {{
        std::eprintln!("\x1b[1m{}\x1b[0m", std::format_args!($($arg)*))
    }};
}

macro_rules! error {
    () => {
        std::eprintln!()
    };
    ($($arg:tt)*) => {{
        std::eprintln!("\x1b[31;1mError!\x1b[0m \x1b[1m{}\x1b[0m", std::format_args!($($arg)*))
    }};
}

fn prompt<T>(message: impl fmt::Display) -> T
where
    T: FromStr,
    T::Err: fmt::Display,
{
    eprint!("\x1b[1m{message}\x1b[0m: ");

    let mut buf = String::new();

    if let Err(error) = io::stdin().read_line(&mut buf) {
        error!("failed to read the input. {error}");
        buf.clear();
    };

    match buf.trim().parse() {
        Ok(parsed) => parsed,
        Err(error) => {
            error!("can't parse the input. {error}. input is '{buf}'");
            prompt(message)
        }
    }
}

fn select_task() {
    message!(
        "Tasks:
    1. ...
    2. ...
    3. ...
    4. ...",
    );

    let selected_task = loop {
        match prompt::<String>("Select task").trim().parse() {
            Ok(1) => break Task1,
            Ok(2) => break Task2,
            Ok(3) => break Task3,
            Ok(4) => break Task4,
            Ok(task) => error!("Unknown task: {task}"),
            Err(_) => error!("Invalid input"),
        }
    };

    match selected_task {
        Task1 => task1(),
        Task2 => task2(),
        Task3 => task3(),
        Task4 => task4(),
    }
}

fn read_array<T>() -> Vec<T>
where
    T: FromStr,
    T::Err: fmt::Display,
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

fn read_array_fixed<T>(size: usize) -> Vec<T>
where
    T: FromStr + fmt::Display,
    T::Err: fmt::Display,
{
    loop {
        let array = read_array();

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

fn read_matrix<T>() -> Vec<Vec<T>>
where
    T: FromStr + fmt::Display,
    T::Err: fmt::Display,
{
    let columns = prompt::<usize>("Input matrix columns count");
    let mut matrix = Vec::with_capacity(columns);

    for _ in 0..columns {
        matrix.push(read_array());
    }

    message!(
        "Typed matrix:\n{}",
        matrix
            .iter()
            .map(|row| format!("[{}]", row.iter().join(", ")))
            .join("\n")
    );
    matrix
}

fn read_square_matrix<T>() -> Vec<Vec<T>>
where
    T: FromStr + fmt::Display,
    T::Err: fmt::Display,
{
    let size = prompt::<usize>("Input matrix size");
    let mut matrix = Vec::with_capacity(size);

    for _ in 0..size {
        matrix.push(read_array_fixed(size));
    }

    message!(
        "Typed matrix:\n{}",
        matrix
            .iter()
            .map(|row| format!("[{}]", row.iter().join(", ")))
            .join("\n")
    );
    matrix
}

/// Підрахувати кількість від’ємних елементів матриці.
fn task1() {
    let matrix = read_matrix::<i32>();
    let negative_elements = matrix
        .iter()
        .map(|row| row.iter().filter(|elem| elem.is_negative()).count())
        .sum::<usize>();

    println!("{negative_elements}")
}

fn task2() {
    let matrix = read_square_matrix::<i32>();
}

fn task3() {
    println!("task3")
}

fn task4() {
    println!("task4")
}
