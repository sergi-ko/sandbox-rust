use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time;

pub fn do_something() -> &'static str {
    let mut threads = vec![];

    let res1 = Arc::new(Mutex::new(0));
    let res2 = Arc::new(Mutex::new(0));

    let res1c = Arc::clone(&res1);
    let res2c = Arc::clone(&res2);
    threads.push(run_task1(res1c, res2c));

    let res1c = Arc::clone(&res1);
    let res2c = Arc::clone(&res2);
    threads.push(run_task2(res1c, res2c));

    for handle in threads {
        handle.join().unwrap();
    }

    println!("{}", *res1.lock().unwrap());
    println!("{}", *res2.lock().unwrap());

    "doing something"
}

pub fn run_task1(res1: Arc<Mutex<u8>>, res2: Arc<Mutex<u8>>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut task1_res1 = res1.lock().unwrap();
        *task1_res1 += 1;
        let mut task1_res2 = res2.lock().unwrap();
        *task1_res2 += 1;
        thread::sleep(time::Duration::from_millis(2_000));
        println!("running task1");
    })
}

pub fn run_task2(res1: Arc<Mutex<u8>>, res2: Arc<Mutex<u8>>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut task1_res1 = res1.lock().unwrap();
        *task1_res1 += 10;
        let mut task1_res2 = res2.lock().unwrap();
        *task1_res2 += 10;
        thread::sleep(time::Duration::from_millis(3_000));
        println!("running task2");
    })
}
