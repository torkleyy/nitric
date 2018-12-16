use lock_api::RawMutex as Unused0;
use parking_lot::RawMutex;

pub trait Lock<'a> {
    type Output;

    unsafe fn lock_info(&self) -> LockInfo<'_>;
    unsafe fn lock_unchecked(self) -> Self::Output;
}

pub struct LockInfo<'a> {
    pub id: usize,
    pub guard: RawLockGuard<'a>,
}

pub enum Never {}

pub enum RawLockGuard<'a> {
    RawMutex(&'a RawMutex),

    #[doc(hidden)]
    __NonExhaustive(Never),
}

impl<'a> RawLockGuard<'a> {
    pub fn lock(&self) {
        match *self {
            RawLockGuard::RawMutex(ref raw) => raw.lock(),
            RawLockGuard::__NonExhaustive(ref n) => match *n {},
        }
    }

    pub fn try_lock(&self) -> bool {
        match *self {
            RawLockGuard::RawMutex(ref raw) => raw.try_lock(),
            RawLockGuard::__NonExhaustive(ref n) => match *n {},
        }
    }
}

pub trait ReadLock<'a> {
    type Output;

    unsafe fn lock_info(&self) -> LockInfo<'_>;
    unsafe fn lock_unchecked(self) -> Self::Output;
}

pub trait WriteLock<'a> {
    type Output;

    unsafe fn lock_info(&self) -> LockInfo<'_>;
    unsafe fn lock_unchecked(self) -> <Self as WriteLock<'a>>::Output;
}
