use std::sync::{atomic::{AtomicBool, Ordering::Relaxed, AtomicU32}, RwLock};

fn example_1() {
    let flag = AtomicBool::new(true);
    std::thread::scope(|s| {
        s.spawn(|| {
            while flag.load(Relaxed) {
                println!("Hello, world!");
            }
        });
        s.spawn(|| {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            flag.store(false, Relaxed)
        });
    });
}

fn example_2() {
    let flag = AtomicU32::new(0);
    std::thread::scope(|s| {
        s.spawn(|| {
            while flag.fetch_add(1, Relaxed) != 99_999 {}
        });
        s.spawn(|| {
            let mut v = flag.load(Relaxed);
            while v != 100_000 {
                println!("thread 1: {v}");
                v = flag.load(Relaxed);
            }
        });
    });
    println!("thread main: {}", flag.load(Relaxed));
}

fn example_3() {
    let flag = AtomicU32::new(0);
    std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10_000 {
                flag.fetch_add(1, Relaxed);
            }
        });
        s.spawn(|| {
            for _ in 0..10_000 {
                flag.fetch_add(1, Relaxed);
            }
        });
    });
    println!("thread main: {}", flag.load(Relaxed));
}

fn example_4() {
    static mut FLAG: u32 = 0u32;
    std::thread::scope(|s| {
        s.spawn(|| {
            unsafe {
                for _ in 0..10_000 {
                    FLAG += 1;
                }
            }
        });
        s.spawn(|| {
            unsafe {
                for _ in 0..10_000 {
                    FLAG += 1;
                }
            }
        });
    });
    println!("thread main: {}", unsafe { FLAG });
}

fn example_5() {
    let v = RwLock::new(Vec::new());
    std::thread::scope(|s| {
        s.spawn(|| {
            for i in 0..500 {
                let mut v = v.write().unwrap();
                v.push(i);
            }
        });
        s.spawn(|| {
            for _ in 0..100 {
                let v = v.read().unwrap();
                println!("thread 1: {:?}", v.last());
            }
        });
        s.spawn(|| {
            for _ in 0..100 {
                let v = v.read().unwrap();
                println!("thread 2: {:?}", v.last());
            }
        });
    });
    println!("thread main: {}", v.read().unwrap().len());
}

fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
    example_5();
}
