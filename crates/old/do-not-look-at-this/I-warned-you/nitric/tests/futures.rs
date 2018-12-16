#![cfg(notnow)]

#[test]
fn test_spawn() {
    let mut pool: ThreadPool = ThreadPool::new().unwrap();

    let future = FnFuture::new(|| 1 + 1);
    assert_eq!(pool.run(future), 2);
}
