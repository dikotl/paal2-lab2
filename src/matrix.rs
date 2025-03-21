use std::fmt::{self, Debug, Display};
use std::{mem, slice, vec};

use itertools::Itertools as _;

#[derive(Debug, Default, Clone)]
pub struct Matrix<T>(Vec<Vec<T>>);

impl<T: Display + Debug> Matrix<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn swap_elements(
        &mut self,
        (row_a, column_a): (usize, usize),
        (row_b, column_b): (usize, usize),
    ) {
        if row_a == row_b {
            self.0[row_a].swap(column_a, column_b);
        } else {
            let (row1, row2) = self.0.split_at_mut(row_b);
            let row1 = &mut row1[0];
            let row2 = &mut row2[0];

            mem::swap(&mut row1[column_a], &mut row2[column_b]);
        }
    }

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        self.0.swap(a, b);
    }

    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        self.0.push(row)
    }

    pub fn iter(&self) -> slice::Iter<Vec<T>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<Vec<T>> {
        self.0.iter_mut()
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = Vec<T>;
    type IntoIter = vec::IntoIter<Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Clone> From<&[&[T]]> for Matrix<T> {
    fn from(value: &[&[T]]) -> Self {
        Self(
            value
                .iter()
                .map(|row| row.iter().cloned().collect_vec())
                .collect_vec(),
        )
    }
}

impl<T: Clone> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Self(value)
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|row| format!("[{}]", row.iter().join(", ")))
                .join("\n")
        )
    }
}
