use crate::internal;

pub trait Lock<'a>: internal::Lock<'a> {}

impl<'a, T> Lock<'a> for T where T: internal::Lock<'a> {}

pub struct Ref<T>(T);

impl<'a, T> internal::Lock<'a> for Ref<T>
where
    T: ReadLock<'a>,
{
    type Output = T::Output;

    unsafe fn lock_info(&self) -> internal::LockInfo<'_> {
        self.0.lock_info()
    }

    unsafe fn lock_unchecked(self) -> Self::Output {
        self.0.lock_unchecked()
    }
}

pub struct Mut<T>(T);

impl<'a, T> internal::Lock<'a> for Mut<T>
where
    T: WriteLock<'a>,
{
    type Output = <T as internal::WriteLock<'a>>::Output;

    unsafe fn lock_info(&self) -> internal::LockInfo<'_> {
        <T as internal::WriteLock<'a>>::lock_info(&self.0)
    }

    unsafe fn lock_unchecked(self) -> Self::Output {
        <T as internal::WriteLock<'a>>::lock_unchecked(self.0)
    }
}

pub trait ReadLock<'a>: internal::ReadLock<'a> {
    fn read(self) -> Ref<Self>
    where
        Self: Sized
    {
        Ref(self)
    }
}

impl<'a, T> ReadLock<'a> for T where T: internal::ReadLock<'a> {}

pub trait WriteLock<'a>: internal::WriteLock<'a> {
    fn write(self) -> Mut<Self>
    where
        Self: Sized,
    {
        Mut(self)
    }
}

impl<'a, T> WriteLock<'a> for T where T: internal::WriteLock<'a> {}
