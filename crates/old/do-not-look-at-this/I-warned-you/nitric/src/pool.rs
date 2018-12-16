use std::{
    collections::VecDeque,
    panic::{catch_unwind, resume_unwind, AssertUnwindSafe, UnwindSafe},
    pin::{Pin, Unpin},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::{spawn, Result as PanicResult},
};

use futures::{
    prelude::*,
    task::{LocalWaker, Waker},
    Poll,
};
use parking_lot::{Condvar, Mutex};

use crate::atom::Atom;

trait BoxFn: Send {
    fn call(self: Box<Self>);
}

impl<T> BoxFn for T
where
    T: FnOnce() -> () + Send,
{
    fn call(self: Box<Self>) {
        let this = *self;

        this();
    }
}

struct Com<T> {
    result: Atom<PanicResult<T>>,
    waker: Atom<Waker>,
}

impl<T> Default for Com<T> {
    fn default() -> Self {
        Com {
            result: Default::default(),
            waker: Default::default(),
        }
    }
}

struct Exec {
    ptr: *mut dyn BoxFn,
}

impl Exec {
    pub fn from_static<T: FnOnce() -> R + Send + 'static, R: Send + 'static>(
        val: T,
    ) -> (Self, Arc<Com<R>>) {
        unsafe { Self::from_scoped(val) }
    }

    pub unsafe fn from_scoped<'a, T: FnOnce() -> R + Send + 'a, R: Send + 'a>(
        closure: T,
    ) -> (Self, Arc<Com<R>>) {
        use std::mem::transmute;

        let com = Arc::new(Com::default());
        let com2 = com.clone();

        let wrapped = move || {
            let closure = AssertUnwindSafe(move || closure());
            let res = catch_unwind(move || closure());

            com2.result.set(res, Ordering::AcqRel); // TODO: Release

            if let Some(waker) = com2.waker.take(Ordering::AcqRel) {
                waker.wake();
            }
        };

        let ptr: Box<(BoxFn + 'a)> = Box::new(wrapped) as Box<BoxFn + 'a>;
        let ptr: Box<BoxFn + 'static> = transmute(ptr);
        let ptr = Box::into_raw(ptr) as *mut dyn BoxFn;

        (Exec { ptr }, com)
    }

    pub fn call(self) {
        use std::mem;

        let val: Box<BoxFn> = unsafe { Box::from_raw(self.ptr) };
        val.call();

        mem::forget(self);
    }
}

unsafe impl Send for Exec {}
impl UnwindSafe for Exec {}

impl Drop for Exec {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.ptr as _);
        }
    }
}

pub struct ExecFuture<T> {
    inner: ExecFutureInner<T>,
}

impl<T> ExecFuture<T> {
    pub fn panic_result(self) -> ExecFutureInner<T> {
        self.inner
    }
}

impl<T> Unpin for ExecFuture<T> {}

impl<T> Future for ExecFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<<Self as Future>::Output> {
        let inner = unsafe { Pin::map_unchecked_mut(self, |t| &mut t.inner) };

        match inner.poll(lw) {
            Poll::Ready(res) => match res {
                Ok(x) => Poll::Ready(x),
                Err(payload) => resume_unwind(payload),
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct ExecFutureInner<T> {
    com: Arc<Com<T>>,
}

impl<T> Unpin for ExecFutureInner<T> {}

impl<T> Future for ExecFutureInner<T> {
    type Output = PanicResult<T>;

    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<<Self as Future>::Output> {
        // Set the waker to this waker; this might clone unnecessarily, but it is hard to
        // optimize:
        // 1) We cannot set it after the "inner poll" (race condition)
        // 2) We have to guarantee the most recent waker is woken up
        let waker = lw.as_waker().clone();
        self.com.waker.set(waker, Ordering::AcqRel);

        if let Some(result) = self.com.result.take(Ordering::AcqRel) {
            return Poll::Ready(result);
        }

        Poll::Pending
    }
}

pub struct Scope<'a> {
    x: &'a i32, // TODO

    // TODO: panic handling
}

pub struct ThreadPool {
    num_threads: AtomicUsize,
    max_num: usize,
    work_queue: Arc<WorkQueue>,
}

impl ThreadPool {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn spawn<F, R>(&self, f: F) -> ExecFuture<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (exec, com) = Exec::from_static(f);
        let notified = self.work_queue.push(exec);

        if !notified {
            self.add_thread();
        }

        ExecFuture {
            inner: ExecFutureInner { com },
        }
    }

    pub fn scope<'a, F, R>(&self, f: F) -> R where F: FnOnce(&Scope<'a>) -> R + 'a, R: 'a {

    }

    /// Adds a new thread to the pool if the maximum of threads is not reached yet.
    pub fn add_thread(&self) {
        let snapshot = self.num_threads();

        if snapshot < self.max_num {
            if self
                .num_threads
                .compare_and_swap(snapshot, snapshot + 1, Ordering::AcqRel)
                != snapshot
            {
                // Retry
                self.add_thread();
            } else {
                self.create_thread();
            }
        }
    }

    fn create_thread(&self) {
        let thread = Thread {
            work_queue: self.work_queue.clone(),
        };

        spawn(move || thread.run());
    }

    pub fn num_threads(&self) -> usize {
        self.num_threads.load(Ordering::Acquire)
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        ThreadPool {
            num_threads: AtomicUsize::new(0),
            max_num: 12,
            work_queue: Default::default(),
        }
    }
}

impl UnwindSafe for ThreadPool {}

struct Thread {
    work_queue: Arc<WorkQueue>,
}

impl Thread {
    pub fn run(&self) {
        while let Some(e) = self.work_queue.pop() {
            e.call();
        }
    }
}

#[derive(Default)]
struct Work {
    tasks: VecDeque<Exec>,
}

impl Work {
    pub fn push(&mut self, f: Exec) {
        self.tasks.push_back(f);
    }

    pub fn pop(&mut self) -> Option<Exec> {
        self.tasks.pop_front()
    }
}

#[derive(Default)]
struct WorkQueue {
    work: Mutex<Work>,
    condvar: Condvar,
}

impl WorkQueue {
    /// Returns `true` if a thread was notified.
    pub fn push(&self, f: Exec) -> bool {
        self.work.lock().push(f);

        self.condvar.notify_one()
    }

    pub fn pop(&self) -> Option<Exec> {
        let mut work = self.work.lock();
        if let Some(x) = work.pop() {
            return Some(x);
        }

        self.condvar.wait(&mut work);

        work.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;
    use std::thread::sleep;
    use std::time::Duration;

    use futures::executor::block_on;

    #[test]
    fn test_single() {
        let pool = ThreadPool::new();

        let (s, r) = channel();

        pool.spawn(move || s.send(5).unwrap());

        assert_eq!(r.recv().unwrap(), 5);
    }

    #[test]
    fn test_more_threads() {
        let pool = ThreadPool::new();

        let (s, r) = channel();

        let a = pool.spawn(move || assert_eq!(r.recv().unwrap(), 5));
        let b = pool.spawn(move || s.send(5).unwrap());

        assert_eq!(pool.num_threads(), 2);

        block_on(a.join(b));

        // Sleep to make sure threads are listening again
        sleep(Duration::from_millis(100));

        let a = pool.spawn(|| {});
        block_on(a);

        assert_eq!(pool.num_threads(), 2);

        // Sleep to make sure threads are listening again
        sleep(Duration::from_millis(100));

        let (s, r) = channel();
        let (s2, r2) = channel();

        pool.spawn(move || assert_eq!(r.recv().unwrap(), 5));
        pool.spawn(move || assert_eq!(r2.recv().unwrap(), 8));
        pool.spawn(move || {
            s.send(5).unwrap();
            s2.send(8).unwrap();
        });

        assert_eq!(pool.num_threads(), 3);
    }

    fn fib(n: usize) -> usize {
        if n == 0 {
            0
        } else {
            n + fib(n - 1)
        }
    }

    #[test]
    fn test_wait_on() {
        let pool = ThreadPool::new();

        let fut = pool.spawn(|| fib(12)).map(|x| x * 2);

        assert_eq!(block_on(fut), 2 * 78);
    }

    #[test]
    #[should_panic(expected = "Panic: 42")]
    fn test_spawn_panic() {
        let pool = ThreadPool::new();

        block_on(pool.spawn(|| panic!("Panic: {}", 42)));
    }

    #[test]
    fn test_spawn_panic_catched() {
        use futures::executor::block_on;

        let pool = ThreadPool::new();

        assert!(block_on(pool.spawn(|| panic!("Panic: {}", 42)).panic_result()).is_err());
    }
}
