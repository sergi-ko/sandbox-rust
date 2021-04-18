use std::thread;
use std::thread::JoinHandle;

pub fn do_something() -> &'static str {
    let mut threads = vec![];
    threads.push(run_task1());
    threads.push(run_task2());

    for handle in threads {
        handle.join();
    }

    "doing something"
}

pub fn run_task1() -> JoinHandle<()> {
    thread::spawn(|| {
        println!("running task1");
    })
}

pub fn run_task2() -> JoinHandle<()> {
    thread::spawn(|| {
        println!("running task2");
    })
}
