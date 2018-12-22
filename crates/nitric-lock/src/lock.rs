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

pub struct Mut<T>(T);

impl<'a, T> Lock<'a> for Mut<T>
where
    T: WriteLock<'a>,
{
    type Output = <T as WriteLock<'a>>::Output;

    unsafe fn lock_info(&self) -> LockInfo<'_> {
        <T as WriteLock<'a>>::lock_info(&self.0)
    }

    unsafe fn lock_unchecked(self) -> Self::Output {
        <T as WriteLock<'a>>::lock_unchecked(self.0)
    }
}

pub struct Ref<T>(T);

impl<'a, T> Lock<'a> for Ref<T>
where
    T: ReadLock<'a>,
{
    type Output = T::Output;

    unsafe fn lock_info(&self) -> LockInfo<'_> {
        self.0.lock_info()
    }

    unsafe fn lock_unchecked(self) -> Self::Output {
        self.0.lock_unchecked()
    }
}

pub trait ReadLock<'a> {
    type Output;

    fn read(self) -> Ref<Self>
    where
        Self: Sized,
    {
        Ref(self)
    }

    unsafe fn lock_info(&self) -> LockInfo<'_>;
    unsafe fn lock_unchecked(self) -> Self::Output;
}

pub trait WriteLock<'a> {
    type Output;

    fn write(self) -> Mut<Self>
    where
        Self: Sized,
    {
        Mut(self)
    }

    unsafe fn lock_info(&self) -> LockInfo<'_>;
    unsafe fn lock_unchecked(self) -> <Self as WriteLock<'a>>::Output;
}
