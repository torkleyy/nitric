use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

pub struct Atom<T> {
    inner: AtomicPtr<T>,
    marker: PhantomData<UnsafeCell<()>>,
}

impl<T> Atom<T> {
    pub fn empty() -> Atom<T> {
        Atom {
            inner: AtomicPtr::new(ptr::null_mut()),
            marker: PhantomData,
        }
    }

    pub fn take(&self, order: Ordering) -> Option<T> {
        let old = self.inner.swap(ptr::null_mut(), order);
        unsafe { Self::from_raw(old) }
    }

    pub fn set(&self, v: T, order: Ordering) -> Option<T> {
        let new = Box::new(v);
        let new = Box::into_raw(new);
        let old = self.inner.swap(new, order);
        unsafe { Self::from_raw(old) }
    }

    pub unsafe fn from_raw(ptr: *mut T) -> Option<T> {
        if ptr == ptr::null_mut() {
            return None;
        }

        Some(*Box::from_raw(ptr))
    }
}

unsafe impl<T> Send for Atom<T> where T: Send {}
unsafe impl<T> Sync for Atom<T> where T: Send {}

impl<T> Default for Atom<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Drop for Atom<T> {
    fn drop(&mut self) {
        self.take(Ordering::SeqCst);
    }
}
