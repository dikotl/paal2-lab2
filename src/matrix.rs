use core::cmp::Ordering;
use core::iter::Flatten;
use core::ops::{Index, IndexMut};
use std::fmt::{self, Display};
use std::{mem, slice, vec};

use itertools::Itertools;

use crate::sort::Sort;

#[derive(Debug, Default, Clone)]
pub struct Matrix<T>(Vec<Vec<T>>);

#[allow(unused)]
impl<T> Matrix<T> {
    pub fn with_rows(capacity: usize) -> Matrix<T> {
        Matrix(Vec::with_capacity(capacity))
    }

    pub fn swap_elements(&mut self, a @ (ax, ay): (usize, usize), b @ (bx, by): (usize, usize)) {
        if ax == bx {
            // Fast path, just swap elements in the row.
            self[ax].swap(ay, by);
        } else {
            // Swap row indices so `a <= b`.
            let ((ax, ay), (bx, by)) = if ax > bx { (b, a) } else { (a, b) };

            // Split at greater row index.
            let (left, right) = self.0.split_at_mut(bx);

            // Left part contains all rows needed to just index it by `row_a`.
            // Right part contains required row at index `0`.
            let row_a = &mut left[ax];
            let row_b = &mut right[0];

            mem::swap(&mut row_a[ay], &mut row_b[by]);
        }
    }

    pub fn transpose(&mut self) {
        let n = self.columns();
        let m = self.rows();

        if n == m {
            // Fast path, no need to allocate new matrix, just swap the element.
            for i in 0..n {
                for j in (i + 1)..m {
                    if i != j {
                        debug!("{i} {j}");
                        self.swap_elements((i, j), (j, i));
                    }
                }
            }
        } else {
            // Slow path, allocate a new matrix.
            self.0 = (0..n)
                .map(|_| (0..m).map(|j| self.0[j].remove(0)).collect())
                .collect();
        }
    }

    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn columns(&self) -> usize {
        self.0.iter().map(|row| row.len()).max().unwrap_or(0)
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        self.0.push(row)
    }

    pub fn is_square(&self) -> bool {
        self.iter_rows().all(|row| row.len() == self.rows())
    }

    pub fn iter_rows(&self) -> slice::Iter<Vec<T>> {
        self.0.iter()
    }

    pub fn iter_rows_mut(&mut self) -> slice::IterMut<Vec<T>> {
        self.0.iter_mut()
    }

    pub fn iter_elements(&self) -> Flatten<slice::Iter<Vec<T>>> {
        self.0.iter().flatten()
    }

    pub fn iter_elements_mut(&mut self) -> Flatten<slice::IterMut<Vec<T>>> {
        self.0.iter_mut().flatten()
    }

    pub fn iter_diagonal(&self, primary: bool) -> MatrixDiagonalIter<T> {
        MatrixDiagonalIter {
            matrix: self,
            i: 0,
            primary,
        }
    }

    pub fn iter_diagonal_mut(&mut self, primary: bool) -> MatrixDiagonalIterMut<T> {
        MatrixDiagonalIterMut {
            matrix: self,
            i: 0,
            primary,
        }
    }

    pub fn iter_columns(&mut self) -> MatrixColumnsIter<T> {
        MatrixColumnsIter {
            matrix: self,
            column: 0,
        }
    }

    pub fn iter_columns_mut(&mut self) -> MatrixColumnsIterMut<T> {
        MatrixColumnsIterMut {
            matrix: self,
            column: 0,
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        &self.0[row]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.0[row]
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
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

//------------------------------------------------------------------------------
// MatrixDiagonalIter iterator
//------------------------------------------------------------------------------

pub struct MatrixDiagonalIter<'a, T: 'a> {
    matrix: &'a Matrix<T>,
    i: usize,
    primary: bool,
}

impl<'a, T> Iterator for MatrixDiagonalIter<'a, T> {
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

impl<T> ExactSizeIterator for MatrixDiagonalIter<'_, T> {
    fn len(&self) -> usize {
        self.matrix.rows()
    }
}

//------------------------------------------------------------------------------
// MatrixDiagonalIterMut iterator
//------------------------------------------------------------------------------

pub struct MatrixDiagonalIterMut<'a, T: 'a> {
    matrix: &'a mut Matrix<T>,
    i: usize,
    primary: bool,
}

impl<'a, T> Iterator for MatrixDiagonalIterMut<'a, T> {
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
        // mutable references overlap.
        unsafe { Some(&mut *ptr) }
    }
}

impl<T> ExactSizeIterator for MatrixDiagonalIterMut<'_, T> {
    fn len(&self) -> usize {
        self.matrix.rows()
    }
}

impl<T> Sort for MatrixDiagonalIterMut<'_, T> {
    type SortedItem = T;

    fn swap(&mut self, a: usize, b: usize) {
        if self.primary {
            self.matrix.swap_elements((a, a), (b, b));
        } else {
            let max = self.len() - 1;
            self.matrix.swap_elements((a, max - a), (b, max - b));
        }
    }

    fn cmp_by<F>(&self, a: usize, b: usize, cmp: &mut F) -> Ordering
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        if self.primary {
            cmp(&self.matrix[a][a], &self.matrix[b][b])
        } else {
            let max = self.len() - 1;
            cmp(&self.matrix[a][max - a], &self.matrix[b][max - b])
        }
    }
}

//------------------------------------------------------------------------------
// MatrixColumnsIter iterator
//------------------------------------------------------------------------------

pub struct MatrixColumnsIter<'a, T: 'a> {
    matrix: &'a Matrix<T>,
    column: usize,
}

impl<'a, T> Iterator for MatrixColumnsIter<'a, T> {
    type Item = Vec<Option<&'a T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let columns = self.matrix.columns();

        if self.column >= columns {
            return None;
        }

        let mut buf = Vec::with_capacity(columns);

        for row in 0..self.matrix.rows() {
            buf.push(self.matrix[row].get(self.column));
        }

        self.column += 1;
        Some(buf)
    }
}

impl<T> ExactSizeIterator for MatrixColumnsIter<'_, T> {
    fn len(&self) -> usize {
        self.matrix.columns()
    }
}

//------------------------------------------------------------------------------
// MatrixColumnsIterMut iterator
//------------------------------------------------------------------------------

pub struct MatrixColumnsIterMut<'a, T: 'a> {
    matrix: &'a mut Matrix<T>,
    column: usize,
}

impl<'a, T> Iterator for MatrixColumnsIterMut<'a, T> {
    type Item = Vec<Option<&'a mut T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let columns = self.matrix.columns();

        if self.column >= columns {
            return None;
        }

        let mut buf = Vec::with_capacity(columns);

        for i in 0..self.matrix.rows() {
            let row = &raw mut self.matrix[i];

            // SAFETY: We guarantee that each element is unique and no two
            // mutable references overlap.
            let item = unsafe { (*row).get_mut(self.column) };

            buf.push(item);
        }

        self.column += 1;
        Some(buf)
    }
}

impl<T> ExactSizeIterator for MatrixColumnsIterMut<'_, T> {
    fn len(&self) -> usize {
        self.matrix.columns()
    }
}
