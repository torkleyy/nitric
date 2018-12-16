#![cfg(notnow)]

use std::{
    pin::{Pin, Unpin},
    sync::{atomic::Ordering, Arc},
};

use futures::{
    executor::ThreadPool,
    prelude::*,
    task::{AtomicWaker, LocalWaker},
    Poll,
};

use crate::{atom::Atom, pool::Schedule};

pub fn compute<F, R>(f: F) -> ComputeFuture<F, R> {

}

pub struct ComputeFuture<F, R, S> {
    f: Option<F>,
    started: bool,
    result: Arc<Atom<R>>,
}

impl<F, R, S> ComputeFuture<F, R, S> {
    pub fn new(f: F, s: S) -> Self {
        ComputeFuture {
            f: Some(f),
            started: false,
            result: Arc::new(Atom::empty()),
        }
    }
}

impl<F, R, S> Unpin for ComputeFuture<F, R, S> {}

impl<'a, F, R, S> Future for ComputeFuture<F, R, S>
where
    F: 'a,
    S: Schedule<(F + 'a)>,
    R: 'a,
{
    type Output = R;

    fn poll(mut self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<<Self as Future>::Output> {
        match self.started {
            true => match self.result.take(Ordering::AcqRel) {
                Some(x) => Poll::Ready(x),
                None => Poll::Pending,
            },
            false => {
                let waker = AtomicWaker::new();
                waker.register(lw);
                let result = self.result.clone();
                let f = self.f.take().unwrap();
                self.started = true;

                rayon::spawn(move || {
                    // TODO: panic handling
                    result.set(f(), Ordering::Release);
                    waker.wake();
                });

                Poll::Pending
            }
        }
    }
}
