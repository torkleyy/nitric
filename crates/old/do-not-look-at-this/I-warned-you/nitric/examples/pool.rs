#![feature(asm)]

use nitric::pool::ThreadPool;

fn a(m: usize, n: usize) -> usize {
    if m == 0 {
        n + 1
    } else if m > 0 && n == 0 {
        a(m - 1, 1)
    } else {
        a(m - 1, a(m, n - 1))
    }
}

fn work() {
    let dummy = a(3, 2);
    unsafe {
        asm!("" : : "r"(&dummy));
    }
}

fn main() {
    let pool = ThreadPool::new();

    for _ in 0..10_000_000 {
        pool.spawn(|| work());
    }
}
