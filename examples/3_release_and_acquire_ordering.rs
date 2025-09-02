use std::{
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Release},
    },
    thread,
    time::Duration,
};

// static DATA: AtomicU64 = AtomicU64::new(0);
static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        // DATA.store(123, Relaxed);
        unsafe { DATA = 123 };
        READY.store(true, Release); // 在这个 store 之前的所有操作都会在这个 store 操作之后可见
    });
    while !READY.load(Acquire) {
        // READY 的值变为 true，表示前面的 load 操作对于其他线程可见
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    // println!("{}", DATA.load(Relaxed));
    println!("{}", unsafe { DATA });
}
