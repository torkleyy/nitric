use std::{cell::UnsafeCell, marker::PhantomData, ops::Deref, ops::DerefMut};

use lock_api::RawMutex as Unused0;
use parking_lot::RawMutex;

use crate::internal;

pub fn new_mutex<T>(data: T, id: usize) -> Mutex<T> {
    Mutex {
        data: UnsafeCell::new(data),
        id,
        raw: RawMutex::INIT,
    }
}

pub struct Mutex<T> {
    data: UnsafeCell<T>,
    id: usize,
    raw: RawMutex,
}

impl<T> Mutex<T> {
    pub fn lock_id(&self) -> usize {
        self.id
    }

    // TODO: decide whether to expose them, and how
    // TODO: (simply exposing would easily allow deadlocks)
    /*
    pub unsafe fn lock(&self) -> MutexGuard<'_, T> {
        self.raw.lock();

        unsafe { self.acquire_guard() }
    }

    pub unsafe fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        match self.raw.try_lock() {
            true => unsafe { Some(self.acquire_guard()) },
            false => None,
        }
    }
    */

    pub unsafe fn raw(&self) -> &RawMutex {
        &self.raw
    }

    pub unsafe fn acquire_guard(&self) -> MutexGuard<'_, T> {
        MutexGuard {
            marker: PhantomData,
            mutex: self,
        }
    }
}

impl<'a, T> internal::ReadLock<'a> for &'a Mutex<T>
where
    T: 'a,
{
    type Output = MutexGuard<'a, T>;

    unsafe fn lock_info(&self) -> internal::LockInfo<'_> {
        internal::LockInfo {
            id: self.lock_id(),
            guard: internal::RawLockGuard::RawMutex(self.raw()),
        }
    }

    unsafe fn lock_unchecked(self) -> <Self as internal::ReadLock<'a>>::Output {
        self.acquire_guard()
    }
}

impl<'a, T> internal::WriteLock<'a> for &'a Mutex<T>
where
    T: 'a,
{
    type Output = MutexGuard<'a, T>;

    unsafe fn lock_info(&self) -> internal::LockInfo<'_> {
        internal::LockInfo {
            id: self.lock_id(),
            guard: internal::RawLockGuard::RawMutex(self.raw()),
        }
    }

    unsafe fn lock_unchecked(self) -> <Self as internal::WriteLock<'a>>::Output {
        self.acquire_guard()
    }
}

pub struct MutexGuard<'a, T> {
    marker: PhantomData<(&'a mut T, *mut ())>,
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.raw.unlock();
    }
}
