//! Utility types
//!

/// Like `InstanceId`, but only effective in debug mode.
#[derive(Debug, Default)]
pub struct DebugInstanceId {
    #[cfg(debug_assertions)]
    inner: InstanceId,
}

impl DebugInstanceId {
    /// If debug assertions are enabled, asserts both instance ids are equal.
    /// Otherwise, does nothing.
    #[inline]
    pub fn debug_assert_eq(&self, other: &DebugInstanceId) {
        #![allow(unused)]

        #[cfg(debug_assertions)]
        self.inner.assert_eq(&other.inner)
    }

    /// Returns the inner instance id.
    ///
    /// This function is only available if debug assertion are enabled.
    #[cfg(debug_assertions)]
    pub fn inner(&self) -> &InstanceId {
        &self.inner
    }
}

/// A unique ID that can be used to assert two objects refer to another common object.
///
/// Example:
///
/// We have an allocator type which allocates `Foo`s. Some operations could cause bugs if two
/// `Foo`s from different allocators are used; `InstanceId` can assert that both are from the same
/// allocator by comparing their `InstanceId`.
#[derive(Debug, Default, Eq)]
pub struct InstanceId {
    inner: Box<u8>,
}

impl InstanceId {
    /// Returns the unique `usize` representation which is used for all the assertions.
    #[inline]
    pub fn as_usize(&self) -> usize {
        self.inner.as_ref() as *const _ as usize
    }

    /// Check if `self` and `other` are equal, panic otherwise.
    #[inline]
    pub fn assert_eq(&self, other: &InstanceId) {
        assert_eq!(self, other, "`InstanceId`s are not equal");
    }

    /// If debug assertions are enabled, check if `self` and `other` are equal, panic otherwise.
    #[inline]
    pub fn debug_assert_eq(&self, other: &InstanceId) {
        debug_assert_eq!(self, other, "`InstanceId`s are not equal");
    }
}

impl PartialEq for InstanceId {
    #[inline]
    fn eq(&self, other: &InstanceId) -> bool {
        self.as_usize() == other.as_usize()
    }
}
