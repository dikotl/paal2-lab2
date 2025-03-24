#[macro_use]
mod log;
mod matrix;
mod request;
mod sort;

use std::str::FromStr;

use itertools::Itertools as _;

use crate::sort::Sort;

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
            return Ok(Command::Exit);
        }

        match input.parse() {
            Ok(1) => Ok(Command::RunTask1),
            Ok(2) => Ok(Command::RunTask2),
            Ok(3) => Ok(Command::RunTask3),
            Ok(4) => Ok(Command::RunTask4),
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
            Ok(Command::Exit) => break 'task_selector_loop,
            Ok(Command::RunTask1) => task1(),
            Ok(Command::RunTask2) => task2(),
            Ok(Command::RunTask3) => task3(),
            Ok(Command::RunTask4) => task4(),
            Err(error) => error!("{error}"),
        }
    }
}

/// Count negative elements in the matrix.
fn task1() {
    let matrix = request::matrix::<i32>();
    let negative_elements = matrix
        .iter_rows()
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
        .iter_diagonal_mut(false)
        .bubble_sort(|a, b| <i32>::cmp(a, b).reverse());

    println!("{matrix}");
}

/// Sort the columns of the matrix by non-decreasing minimum element.
fn task4() {
    let mut matrix = request::matrix::<i32>();

    // 4 reallocations omg.
    matrix.transpose();
    matrix = matrix
        .into_iter()
        .sorted_by(|column_a, column_b| {
            <i32>::cmp(
                column_a.iter().max().unwrap_or(&0),
                column_b.iter().max().unwrap_or(&0),
            )
        })
        .collect_vec()
        .into();
    matrix.transpose();

    println!("{matrix}");
}
