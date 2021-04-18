use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time;

pub fn do_something() -> &'static str {
    let mut threads = vec![];

    let res1 = Arc::new(RwLock::new(0));
    let res2 = Arc::new(RwLock::new(0));

    let res1c = Arc::clone(&res1);
    let res2c = Arc::clone(&res2);
    threads.push(run_monitoring(res1c, res2c));

    let res1c = Arc::clone(&res1);
    let res2c = Arc::clone(&res2);
    threads.push(run_task1(res1c, res2c));

    let res1c = Arc::clone(&res1);
    let res2c = Arc::clone(&res2);
    threads.push(run_task2(res1c, res2c));

    for handle in threads {
        handle.join().unwrap();
    }

    println!("{}", *res1.write().unwrap());
    println!("{}", *res2.write().unwrap());

    "doing something"
}

pub fn run_monitoring(res1: Arc<RwLock<u8>>, res2: Arc<RwLock<u8>>) -> JoinHandle<()> {
    thread::spawn(move || loop {
        println!("res1={}", res1.read().unwrap());
        println!("res2={}", res2.read().unwrap());

        thread::sleep(time::Duration::from_millis(250));
    })
}

pub fn run_task1(res1: Arc<RwLock<u8>>, res2: Arc<RwLock<u8>>) -> JoinHandle<()> {
    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(500));

        let mut task1_res2 = res2.write().unwrap();
        *task1_res2 += 1;

        thread::sleep(time::Duration::from_millis(2_000));

        let mut task1_res1 = res1.write().unwrap();
        *task1_res1 += 1;
        println!("running task1");
    })
}

pub fn run_task2(res1: Arc<RwLock<u8>>, res2: Arc<RwLock<u8>>) -> JoinHandle<()> {
    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(500));

        let mut task1_res1 = res1.write().unwrap();
        *task1_res1 += 10;
        let mut task1_res2 = res2.write().unwrap();
        *task1_res2 += 10;

        thread::sleep(time::Duration::from_millis(3_000));

        println!("running task2");
    })
}
