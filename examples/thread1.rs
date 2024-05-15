use anyhow::{anyhow, Result};
use std::{thread, sync::mpsc};
use std::time::Duration;

const NUM_THREADS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
    time: u64,
}

impl Msg {
    fn new(idx: usize, value: usize, time: u64) -> Self {
        Msg {
            idx,
            value,
            time,
        }
    }
}

fn main() -> Result<()> {
    println!("这个是测试");
    let (tx, rx) = mpsc::channel();
    // 创建producers  生产者
    for i in 0..NUM_THREADS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx)
        );
    }
    drop(tx);// drop 掉tx,不然主线程一直在等待
    // 创建消费者 consumer 消费者
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg)
        }
        println!("consumer exit");
        42
    });
   let secret =  consumer.join().map_err(|e| anyhow!("consumer join error:{:?}",e))?;
    println!("最后是{secret}");
    Ok(())
}

fn producer(i: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        // new 一个随机数
        let value = rand::random::<usize>();
        let sleep_time = rand::random::<u8>() as u64 * 10;
        // 发送这个值
        tx.send(Msg::new(i, value, sleep_time))?;

        //  线程 睡 1000秒毫秒 as _  rust 中必须显示转换 as _ 默认转换
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 5 == 0 {
            println!("线程 {} 退出",i);
            break;
        }
    }
    Ok(())
}