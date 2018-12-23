//! Joined locking
//!

use crate::{Lock, LockToken};

pub fn lock2<'token, A, B>(_: &'token mut LockToken, a: A, b: B) -> (A::Output, B::Output)
where
    A: Lock<'token> + 'token,
    B: Lock<'token> + 'token,
{
    unsafe {
        let lock_a = a.lock_info();
        let lock_b = b.lock_info();

        let mut locks = [&lock_a, &lock_b];

        if locks[0].id > locks[1].id {
            let tmp = locks[0];

            locks[0] = locks[1];
            locks[1] = tmp;
        }

        locks[0].guard.lock();
        locks[1].guard.lock();

        (a.lock_unchecked(), b.lock_unchecked())
    }
}

// TODO: add more `joinN` functions
// TODO: add macro

#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn test_lock2() {
        let mut group = LockGroup::new();
        let mut token = group.token();

        let mutex_a = group.mutex(42);
        let mutex_b = group.mutex(35);

        let (a, mut b) = lock2(&mut token, mutex_a.read(), mutex_b.write());

        assert_eq!(*a, 42);
        assert_eq!(*b, 35);

        *b = 15;
    }
}
