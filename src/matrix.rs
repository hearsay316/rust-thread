use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul};
use anyhow::{Result, anyhow};
use crate::Vector;


pub struct Matrix<T: Debug> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T: Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T: Debug> Display for Matrix<T>
    where T: Display {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{} ", self.data[i * self.col + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Debug for Matrix<T>
    where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Matrix {{ data: {:?}, row: {}, col: {} }}", self.data, self.row, self.col)
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
    where T: Debug + Default + Copy + Add<Output=T> + AddAssign + Mul<Output=T>, {
    if a.len() != b.len() {
        return Err(anyhow!("error"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

// where 关键字 对泛型做条件限制
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
    where T: Debug + Default + Copy + Add<Output=T> + AddAssign + Mul<Output=T>, {
    if a.col == b.row {
        return Err(anyhow!("error"));
    }
    let mut data = vec![T::default(); a.row * b.col];

    for i in 0..a.row {
        for j in 0..b.col {
                let row = Vector::new(&a.data[i * a.col..(i+1) * a.col]);
                let col_data = b.data[j..].iter().step_by(b.col).copied().collect::<Vec<_>>();
                let col = Vector::new(col_data);
                data[i * b.col + j] += dot_product(row, col)?;
        }
    }
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![7, 8, 9, 10, 11, 12], 1, 2);
        let c = multiply(&a, &b)?;
        println!("{:?}", c);
        Ok(())
    }
    #[test]
    fn test_dot_product() -> Result<()> {
        let a = Vector::new(vec![1, 2, 3]);
        let b = Vector::new(vec![4, 5, 6]);
        let c = dot_product(a,b);
        println!("{:?}", c.unwrap());
        Ok(())
    }
}