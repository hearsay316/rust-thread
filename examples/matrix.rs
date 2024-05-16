use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul};
use std::process::Output;
use anyhow::{Result, anyhow};

#[derive(Debug)]
struct Matrix<T: Debug> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

fn main() -> Result<()> {
    Ok(())
}

// where 关键字 对泛型做条件限制
fn multiply<T>(a: Matrix<T>, b: Matrix<T>) -> Result<Matrix<T>>
    where T: Debug +Copy+ Add<Output=T> + AddAssign + Mul<Output=T>, {
    if a.col != b.row {
        return Err(anyhow!("error"));
    }
    let mut data = Vec::with_capacity(a.row * b.col);
    let mut c = Matrix {
        data: vec![0; a.row * b.col],
        row: a.row,
        col: b.col,
    };
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                c.data[i * c.col + j] += a.data[i*a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    Ok(c)
}