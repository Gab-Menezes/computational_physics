// fn main() {
//     const M: f64 = 9.11e-31;
//     const E: f64 = 10f64;
//     const V: f64 = 9f64;
//     const H_: f64 = 6.582119e-16;
//     let k1 = (2f64 * M * E).sqrt() / H_;
//     let k2 = (2f64 * M * (E - V)).sqrt() / H_;
//     let t = (4f64 * k1 * k2) / (k1 + k2).powf(2f64);
//     let r = ((k1 - k2) / (k1 + k2)).powf(2f64);
//     println!("transmition: {t}");
//     println!("reflection: {r}");
// }

// use std::io;

// fn main() {
//     let mut buffer = String::new();

//     println!("r:");
//     io::stdin().read_line(&mut buffer).unwrap();
//     let r: f64 = buffer.trim().parse().unwrap();
//     buffer.clear();

//     println!("phi:");
//     io::stdin().read_line(&mut buffer).unwrap();
//     let phi: f64 = buffer.trim().parse().unwrap();
//     buffer.clear();

//     let x = r*phi.cos();
//     let y = r*phi.sin();
//     println!("(x: {x}, y: {y})");
// }

fn main() {
    let mut c = 1f64;
    let mut n = 0f64;
    print!("{c}, ");
    loop {
        c = c * (4f64 * n + 2f64) / (n + 2f64);
        n += 1f64;
        if c > 1_000_000_000f64 {
            break;
        }
        print!("{c}, ");
    }
    println!("");
}


// use std::io;

// fn main() {
//     let mut buffer = String::new();

//     println!("num:");
//     io::stdin().read_line(&mut buffer).unwrap();
//     let num: u32 = buffer.trim().parse().unwrap();
//     buffer.clear();

//     if num % 2 == 0 {
//         println!("even");
//     } else {
//         println!("odd");
//     }
// }


so I was thinking last night about a specific type of rwlock and I don't know if it exists or I'm overengineering. Its basically a rwlock, where the write lock is done in 2 phases. You first declare a intent to acquire the write lock in the future but not right now (so any other intent to acquire will be done after the total release of it), while you don't effectively acquire it the lock to write (during the intent phase) it will be in read state (so other threads can use it as read), but after the effective acquire only you can write (like a normal rwlock). I know that are locks that can be upgraded from read to write but this doesn't ensure that the view you had while reading will be the same as when you acquire the write, since something else could upgrade to write before you. With this lock you would be ensured that the view you had while reading is the same when you write. This would be useful when the write is depedent of the current state, but you don't want to cause contention in the lock by acquiring it for write during the whole operation only when the write will happen. Did I just reinvent the wheel or there is value to it ?