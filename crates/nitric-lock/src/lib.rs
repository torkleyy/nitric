#![deny(unused_must_use)]

//! # `nitric-lock`
//!

pub use self::group::{LockGroup, LockToken};
pub use self::lock::{Lock, Ref, Mut, ReadLock, WriteLock};
pub use self::mutex::{Mutex, MutexGuard};

pub(crate) use nitric_lock_internals as internal;

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use lock_api::RawMutex as Unused0;
use parking_lot::RawMutex;

mod group;
mod join;
mod lock;
mod mutex;

// TODO: remove this code once the `join` mod is done
/*
pub struct MutexJoin<'a, A, B> {
    head: &'a Mutex<A>,
    tail: B,
}

impl<'a, A, B> MutexJoin<'a, A, B>
where
    B: 'a,
{
    pub fn new(head: &'a Mutex<A>, tail: B) -> Self {
        MutexJoin { head, tail }
    }

    //noinspection RsNeedlessLifetimes
    pub fn lock<'l>(&'l self) -> <Self as MutexLikeInner>::Output
    where
        B: MutexLikeInner<'l>,
    {
        let collection = self.collect_raw_set();

        for (_, mutex) in collection {
            mutex.lock();
        }

        unsafe { self.lock_unchecked() }
    }

    //noinspection RsNeedlessLifetimes
    pub fn try_lock<'l>(&'l self) -> Option<<Self as MutexLikeInner<'_>>::Output>
    where
        B: MutexLikeInner<'l>,
    {
        let collection = self.collect_raw_set();

        if collection.into_iter().map(|(_, m)| m.try_lock()).all(|b| b) {
            unsafe { Some(self.lock_unchecked()) }
        } else {
            None
        }
    }

    //noinspection RsNeedlessLifetimes
    fn collect_raw_set<'l>(&'l self) -> Vec<(usize, &RawMutex)>
    where
        B: MutexLikeInner<'l>,
    {
        let mut collection = vec![];

        unsafe {
            self.extend_raw_set(&mut collection);
        }

        collection.sort_by_key(|e| e.0);

        collection
    }
}

impl<'a, A, B> MutexLikeInner<'a> for MutexJoin<'_, A, B>
where
    A: 'a,
    B: MutexLikeInner<'a> + 'a,
{
    type Output = (MutexGuard<'a, A>, <B as MutexLikeInner<'a>>::Output);

    unsafe fn extend_raw_set<E: Extend<(usize, &'a RawMutex)>>(&'a self, e: &mut E) {
        self.head.extend_raw_set(e);
        self.tail.extend_raw_set(e);
    }

    unsafe fn lock_unchecked(&'a self) -> <Self as MutexLikeInner<'a>>::Output {
        (self.head.lock_unchecked(), self.tail.lock_unchecked())
    }
}

pub trait MutexLike {}

impl<T> MutexLike for T where T: for<'a> MutexLikeInner<'a> {}

pub trait MutexLikeInner<'a> {
    type Output;

    unsafe fn extend_raw_set<E: Extend<(usize, &'a RawMutex)>>(&'a self, e: &mut E);
    unsafe fn lock_unchecked(&'a self) -> Self::Output;
}

impl<'a, T> MutexLikeInner<'a> for &'a Mutex<T>
where
    T: 'a,
{
    type Output = MutexGuard<'a, T>;

    unsafe fn extend_raw_set<E: Extend<(usize, &'a RawMutex)>>(&'a self, e: &mut E) {
        e.extend(Some((self.lock_id(), self.raw())))
    }

    unsafe fn lock_unchecked(&'a self) -> <Self as MutexLikeInner<'a>>::Output {
        self.acquire_guard()
    }
}

#[macro_export]
macro_rules! lock {
    ($($ac:ident $id:ident),+) => {
        lock!(lock => $($ac $id),*);
    };
    ($action:ident => $($ac:ident $id:ident),+) => {

        {
            let __join_binding;
            __join_binding = lock!(@new $($id),*);
            let lock!(@nest $($id),*) = __join_binding.$action();

            ($($id),*)
        }
    };
    (@nest $head:ident, $($id:ident),+) => {
        ($head, lock!(@nest $($id),*))
    };
    (@nest $head:ident) => {
        $head
    };
    (@new $head:ident, $($id:ident),+) => {
        {
            use std::borrow::Borrow;

            MutexJoin::new($head.borrow(), lock!(@new $($id),*))
        }
    };
    (@new $head:ident) => {
        {
            use std::borrow::Borrow;

            $head.borrow()
        }
    };
    (@brw ref $e:expr) => {
        &$e
    };
    (@brw mut $e:expr) => {
        &mut $e
    };
}


#[cfg(never)]
mod tests {
    use super::*;

    #[test]
    fn test_join_macro() {
        let mut group = LockGroup::new();

        let a = group.mutex(18);
        let b = group.mutex(false);
        let c = group.mutex("Hello world".to_owned());

        let (a, b, c) = lock!(ref a, ref b, ref c);

        assert_eq!(*a, 18);
        assert_eq!(*b, false);
    }

    #[test]
    fn test_deadlock() {
        let mut group = LockGroup::new();

        let a = group.mutex(18);
        let b = group.mutex(false);
        let c = group.mutex("Hello world".to_owned());

        let (a, b, c) = lock!(ref a, ref b, ref c);

        assert_eq!(*a, 18);
        assert_eq!(*b, false);
    }

    #[test]
    fn test_foo() {
        fn foo(b: &mut i32, a: &bool) {
            *b = 5;

            println!("{} {}", b, a);
        }

        fn foo_locking(b: &Mutex<i32>, a: &Mutex<bool>) {
            let (mut b, a) = lock!(mut b, ref a);
            foo(&mut b, &a);
        }

        let mut group = LockGroup::new();

        let a = group.mutex(false);
        let b = group.mutex(18);

        foo_locking(&b, &a);
    }
}
*/
