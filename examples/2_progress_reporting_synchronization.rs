use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        // 一个后台线程，去处理100个任务
        s.spawn(|| {
            for i in 0..100 {
                // 假设处理需要一些时间
                process_item(i);
                num_done.store(i + 1, Relaxed);
                // 唤醒主线程
                main_thread.unpark();
            }
        });

        // 主线程展示状态更新
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::park_timeout(std::time::Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn process_item(_i: usize) {
    thread::sleep(std::time::Duration::from_millis(100));
}
