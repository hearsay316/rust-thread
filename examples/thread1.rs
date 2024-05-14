use anyhow::Result;
use std::{thread, sync::mpsc};
use std::time::Duration;

const NUM_THREADS: usize = 4;

fn main() -> Result<()> {
    println!("这个是测试");
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_THREADS {
        let rx = rx.clone();
        thread::spawn(move || producer(i, rx)
        );
    }
    // let producer = thread::spawn(move || {
    //     for i in 0..10 {
    //         tx.send(i).unwrap();
    //     }
    // });
    Ok(())
}

fn producer(i: i8, tx: mpsc::Sender<usize>)->Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(value)?;
        thread::sleep(Duration::from_millis(1000));
    }
}