use std::{sync::atomic::AtomicUsize, sync::atomic::Ordering::Relaxed, thread};

fn main() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // 4 个后台线程，去处理所有 100 个元素，每个 25 次。
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(i + t * 25);
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }

        // 主线程每秒展示一次状态更新。
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    println!("Done!");
}

fn process_item(_i: usize) {
    thread::sleep(std::time::Duration::from_millis(100));
}
