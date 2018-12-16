use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use parking_lot_core::{park, unpark_one, DEFAULT_PARK_TOKEN, DEFAULT_UNPARK_TOKEN};

#[derive(Clone)]
struct Notifier {
    current: Arc<AtomicUsize>,
    notify_on: usize,
}

impl Notifier {
    pub fn notify(self) {
        use std::mem::drop;

        // Unnecessary, but let's be explicit.
        // The notification works by simply dropping self.
        drop(self);
    }

    pub fn discard(mut self) {
        use std::mem::{forget};
        use std::ptr::drop_in_place;

        // Decrement ref count
        unsafe {
            drop_in_place(&mut self.current);
        }
        // But don't call `Notifier::drop`
        forget(self);
    }

    fn key(&self) -> usize {
        self.current.as_ref() as *const _ as usize
    }
}

impl Drop for Notifier {
    fn drop(&mut self) {
        if self.current.fetch_add(1, Ordering::RelAcq) == self.notify_on {
            unsafe {
                unpark_one(self.key(), |_| DEFAULT_UNPARK_TOKEN)
            }
        }
    }
}

pub struct Waiter {
    inner: Notifier,
    num_clones: usize,
}

impl Waiter {
    pub fn wait_on(num: usize) -> Waiter {
        Waiter {
            inner: Notifier {
                current: Arc::new(AtomicUsize::new(0)),
                notify_on: 0
            },
            num_clones: 0,
        }
    }

    /// Returns one of the notifiers.
    pub fn notifier(&self) -> Notifier {
        self.inner.clone()
    }

    pub fn wait(self) {
        unsafe {
            park(self.inner.key(), || {}, || {}, || {}, DEFAULT_PARK_TOKEN, None);
        }
    }
}
