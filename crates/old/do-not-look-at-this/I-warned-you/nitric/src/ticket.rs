#![allow(unused)]

use std::cell::UnsafeCell;
use std::marker::PhantomData;

pub trait LockRaw<T: ?Sized> {
    unsafe fn read<'a>(&self, ticket: &'a ReadTicket<Self, T>) -> &'a T;
    unsafe fn write<'a>(&self, ticket: &'a mut WriteTicket<Self, T>) -> &'a mut T;
}

impl<T> LockRaw<T> for UnsafeCell<T> {
    unsafe fn read<'a>(&self, _ticket: &'a ReadTicket<Self, T>) -> &'a T {
        &*self.get()
    }

    unsafe fn write<'a>(&self, _ticket: &'a mut WriteTicket<Self, T>) -> &'a mut T {
        &mut *self.get()
    }
}

pub struct Locked<T> {
    inner: Box<T>,
}

impl<T> Locked<T> {
    pub fn read<'a, P: ?Sized>(&self, ticket: &'a ReadTicket<T, P>) -> Option<&'a P>
    where
        T: LockRaw<P>,
    {
        assert_eq!(self.inner.as_ref() as *const _ as usize, ticket.ptr);

        Some(unsafe { self.inner.as_ref().read(ticket) })
    }

    pub fn write<'a, P: ?Sized>(&self, ticket: &'a mut WriteTicket<T, P>) -> Option<&'a mut P>
    where
        T: LockRaw<P>,
    {
        assert_eq!(self.inner.as_ref() as *const _ as usize, ticket.ptr);

        Some(unsafe { self.inner.as_ref().write(ticket) })
    }
}

pub struct ReadTicket<T: ?Sized, P: ?Sized> {
    ptr: usize,
    marker: PhantomData<(Box<T>, Box<P>)>, // TODO
}

pub struct WriteTicket<T: ?Sized, P: ?Sized> {
    ptr: usize,
    marker: PhantomData<(Box<T>, Box<P>)>, // TODO
}
