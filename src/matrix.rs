use core::cmp::Ordering;
use core::ops::{Index, IndexMut};
use std::fmt::{self, Debug, Display};
use std::{mem, slice, vec};

use itertools::Itertools as _;

#[derive(Debug, Default, Clone)]
pub struct Matrix<T>(Vec<Vec<T>>);

impl<T: Debug + Display> Matrix<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn swap_elements(
        &mut self,
        (row_a, column_a): (usize, usize),
        (row_b, column_b): (usize, usize),
    ) {
        debug!(
            "swapping {:#?} at ({row_a}, {column_a}) with {:#?} at ({row_b}, {column_b})",
            self[(row_a, column_a)],
            self[(row_b, column_b)]
        );

        if row_a == row_b {
            // Fast path, just swap elements in the row.
            self[row_a].swap(column_a, column_b);
        } else {
            // Swap row indices so `row_a <= row_b`.
            let row_a = <usize>::min(row_a, row_b);
            let row_b = <usize>::max(row_a, row_b);

            // Split at greater row index.
            let (left, right) = self.0.split_at_mut(row_b);

            // Left part contains all rows needed to just index it by `row_a`.
            // Right part contains required row at index `0`.
            let row1 = &mut left[row_a];
            let row2 = &mut right[0];

            mem::swap(&mut row1[column_a], &mut row2[column_b]);
        }

        debug!("Swapped matrix:\n{self}");
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

    pub fn is_square(&self) -> bool {
        self.iter().all(|row| row.len() == self.rows())
    }

    pub fn iter(&self) -> slice::Iter<Vec<T>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<Vec<T>> {
        self.0.iter_mut()
    }

    pub fn primary_diagonal(&self) -> MatrixDiagonalIter<T> {
        assert!(self.is_square());

        MatrixDiagonalIter {
            matrix: self,
            i: 0,
            primary: true,
        }
    }

    pub fn side_diagonal(&self) -> MatrixDiagonalIter<T> {
        assert!(self.is_square());

        MatrixDiagonalIter {
            matrix: self,
            i: 0,
            primary: false,
        }
    }

    pub fn primary_diagonal_mut(&mut self) -> MatrixDiagonalIterMut<T> {
        assert!(self.is_square());

        MatrixDiagonalIterMut {
            matrix: self,
            i: 0,
            primary: true,
        }
    }

    pub fn side_diagonal_mut(&mut self) -> MatrixDiagonalIterMut<T> {
        assert!(self.is_square());

        MatrixDiagonalIterMut {
            matrix: self,
            i: 0,
            primary: false,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        &self.0[row][column]
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        &self.0[row]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        &mut self.0[row][column]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.0[row]
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

impl<T> IntoIterator for Matrix<T> {
    type Item = Vec<T>;
    type IntoIter = vec::IntoIter<Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct MatrixDiagonalIter<'a, T: 'a> {
    matrix: &'a Matrix<T>,
    i: usize,
    primary: bool,
}

pub struct MatrixDiagonalIterMut<'a, T: 'a> {
    pub matrix: &'a mut Matrix<T>,
    i: usize,
    primary: bool,
}

impl<'a, T: Debug + Display> Iterator for MatrixDiagonalIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let max = self.matrix.rows() - 1;

        if self.i > max {
            return None;
        }

        let i = self.i;
        self.i += 1;

        Some(if self.primary {
            &self.matrix.0[i][i]
        } else {
            &self.matrix.0[i][max - i]
        })
    }
}

impl<'a, T: Debug + Display> Iterator for MatrixDiagonalIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let max = self.matrix.rows() - 1;

        if self.i > max {
            return None;
        }

        let i = self.i;
        self.i += 1;

        let ptr = if self.primary {
            &raw mut self.matrix.0[i][i]
        } else {
            &raw mut self.matrix.0[i][max - i]
        };

        // SAFETY: We guarantee that each diagonal element is unique and no two
        // mutable references overlap. This unsafe block converts the raw
        // pointer back into a mutable reference.
        unsafe { Some(&mut *ptr) }
    }
}

impl<T: Ord + Debug + Display> Sort for MatrixDiagonalIterMut<'_, T> {
    type Item = T;

    fn len(&self) -> usize {
        self.matrix.rows()
    }

    fn swap(&mut self, a: usize, b: usize) {
        if self.primary {
            self.matrix.swap_elements((a, a), (b, b));
        } else {
            let max = self.len() - 1;
            self.matrix.swap_elements((a, max - a), (b, max - b));
        }
    }

    fn cmp<F>(&self, a: usize, b: usize, cmp: &mut F) -> Ordering
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        if self.primary {
            cmp(&self.matrix[(a, a)], &self.matrix[(b, b)])
        } else {
            let max = self.len() - 1;
            cmp(&self.matrix[(a, max - a)], &self.matrix[(b, max - b)])
        }
    }
}

pub trait Sort {
    type Item: Ord;

    fn len(&self) -> usize;
    fn swap(&mut self, a: usize, b: usize);
    fn cmp<F>(&self, a: usize, b: usize, cmp: &mut F) -> Ordering
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering;

    fn bubble_sort<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let len = self.len();

        for i in 0..len {
            let mut swapped = false;

            for j in (i + 1)..len {
                if self.cmp(i, j, &mut cmp) == Ordering::Greater {
                    self.swap(i, j);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }
    }
}
