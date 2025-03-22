#[macro_use]
mod ui;
mod matrix;
mod request;

use std::str::FromStr;

use Command::*;

use crate::matrix::Sort;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    RunTask1,
    RunTask2,
    RunTask3,
    RunTask4,
    Exit,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim().to_lowercase();

        if input == "exit" {
            return Ok(Exit);
        }

        match input.parse() {
            Ok(1) => Ok(RunTask1),
            Ok(2) => Ok(RunTask2),
            Ok(3) => Ok(RunTask3),
            Ok(4) => Ok(RunTask4),
            Ok(task) => Err(format!("Unknown task: {task}")),
            Err(_) => Err(String::from("Invalid input")),
        }
    }
}

const TASKS: &str = "Tasks:
    1. Count negative elements in the matrix.
    2. Swap the corresponding elements of the first (technically 0) row and the main diagonal.
    3. Sort the side diagonal of the matrix from the minimum right-top to the maximum left-bottom.
    4. Sort the columns of the matrix by non-decreasing minimum element.
Or type 'exit' to exit the program.";

fn main() {
    'task_selector_loop: loop {
        message!("{TASKS}");

        match request::value::<String>("Select task").parse() {
            Ok(Exit) => break 'task_selector_loop,
            Ok(RunTask1) => task1(),
            Ok(RunTask2) => task2(),
            Ok(RunTask3) => task3(),
            Ok(RunTask4) => task4(),
            Err(error) => error!("{error}"),
        }
    }
}

/// Count negative elements in the matrix.
fn task1() {
    let matrix = request::matrix::<i32>();
    let negative_elements = matrix
        .iter()
        .map(|row| row.iter().filter(|elem| elem.is_negative()).count())
        .sum::<usize>();

    println!("{negative_elements}")
}

/// Swap the corresponding elements of the first row and the main diagonal.
///
/// Assume that the matrix is guaranteed to be square.
fn task2() {
    let mut matrix = request::square_matrix::<i32>();

    for i in 1..matrix.rows() {
        matrix.swap_elements((0, i), (i, i));
    }

    println!("{matrix}");
}

/// Sort the side diagonal of the matrix from the minimum right-top to the
/// maximum left-bottom.
///
/// Assume that the matrix is guaranteed to be square.
fn task3() {
    let mut matrix = request::square_matrix::<i32>();

    matrix
        .side_diagonal_mut()
        .bubble_sort(|a, b| <i32>::cmp(a, b).reverse());

    println!("{matrix}");
}

/// Sort the columns of the matrix by non-decreasing minimum element.
fn task4() {
    todo!()
}
