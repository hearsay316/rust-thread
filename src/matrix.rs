use std::{fmt, thread};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul};
use std::sync::mpsc;
use anyhow::{Result, anyhow};
use crate::Vector;

const NUM_THREADS: usize = 4;

pub struct Matrix<T: Debug> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        MsgInput {
            idx,
            row,
            col,
        }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Msg {
            input,
            sender,
        }
    }
}

impl <T> Mul for Matrix<T>  where T: Debug + Default + Copy + Add<Output=T> + AddAssign + Mul<Output=T> + Send + 'static,  {
    type Output  = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("matrix multiply failed这个是出错啦")
    }
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
    where T: Debug + Default + Copy + Add<Output=T> + AddAssign + Mul<Output=T> + Send + 'static, {
    if a.col != b.row {
        println!("error");
        return Err(anyhow!("error"));
    }
    // 接受数据
    let senders = (0..NUM_THREADS).map(|_| {
        let (tx, rx) = mpsc::channel::<Msg<T>>();
        thread::spawn(move || {
            for msg in rx {
                let value = dot_product(msg.input.row, msg.input.col)?;
                if let Err(e) = msg.sender.send(MsgOutput { idx: msg.input.idx, value }) {
                    eprintln!("Send error :{:?}", e);
                };
            }
            Ok::<_, anyhow::Error>(())
        });
        tx
    }).collect::<Vec<_>>();
    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..].iter().step_by(b.col).copied().collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            //  发送数据
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("sender Err{:?}", e);
            };
            // let ret = rx.revc()?;
            // data[ret.idx] = ret.value;
            receivers.push(rx);
        }
    }
    // 回写数据
    for rx in receivers {
        let ret = rx.recv()?;
        data[ret.idx] = ret.value;
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
        let b = Matrix::new(vec![7, 8, 9, 10, 11, 12], 3, 2);
        let c = a * b;
        println!("{:?}", c);
        Ok(())
    }

    #[test]
    fn test_dot_product() -> Result<()> {
        let a = Vector::new(vec![1, 2, 3]);
        let b = Vector::new(vec![4, 5, 6]);
        let c = dot_product(a, b);
        println!("{:?}", c.unwrap());
        Ok(())
    }
    #[test]
    fn test_a_can_not_multiply_b()  {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let c = multiply(&a, &b);
       println!("{:?}", c.is_err()) ;
    }
    #[test]
    #[should_panic]
    fn test_a_can_not_multiply_b_panic(){
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4], 2, 3);
        let _c = a*b;
    }
}