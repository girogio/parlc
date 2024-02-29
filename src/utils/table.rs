use std::{
    fmt::{Display, Formatter},
    ops::Index,
};

use crate::scanner::dfsa::{Category, State};

#[derive(Debug)]
pub struct Table<T> {
    data: Vec<T>,
    cols: usize,
}

impl<T> Table<T> {
    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            data: vec![Default::default(); rows * cols],
            cols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row * self.cols + col)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.cols + col] = value;
    }

    pub fn new_all(rows: usize, cols: usize, value: T) -> Self
    where
        T: Clone + Default,
    {
        let mut table = Self::new(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                table.set(i, j, value.clone());
            }
        }

        table
    }
}

impl<T> Display for Table<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.data.len() {
            if i % self.cols == 0 {
                writeln!(f)?;
            }
            write!(f, "{:5} ", self.data[i])?;
        }
        writeln!(f)
    }
}

impl<T> Index<(usize, usize)> for Table<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1).unwrap()
    }
}

impl<T> Index<(&State, Category)> for Table<T> {
    type Output = T;

    fn index(&self, index: (&State, Category)) -> &Self::Output {
        self.get(index.0.id, index.1 as usize).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_2dArray() {
        let mut arr = Table::new_all(3, 3, 0);
        arr.set(1, 1, 1);
        assert_eq!(arr.get(6, 6), None);
    }

    #[rstest]
    #[should_panic]
    fn test_2dArray_panic() {
        let arr = Table::new_all(3, 3, 0);
        let i = arr[(6, 6)];
    }

    #[rstest]
    fn test_2dArray_index() {
        let arr = Table::new_all(3, 3, 0);
        println!("{}", arr);
    }
}
