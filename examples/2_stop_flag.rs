use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // 产生一个线程，去做一些工作
    let background_thread = std::thread::spawn(|| {
        while !STOP.load(Relaxed) {
            something();
        }
    });

    // 使用主线程监听用户输入
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("command: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {}", cmd),
        }
    }

    // 告知后台线程需要停止
    STOP.store(true, Relaxed);

    // 等待后台线程结束
    background_thread.join().unwrap();
}

fn something() {
    std::thread::sleep(std::time::Duration::from_secs(1));
    // println!("do something");
}
