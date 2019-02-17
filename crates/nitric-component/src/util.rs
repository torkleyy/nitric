//! Utility types
//!

use std::sync::Arc;

/// Like `InstanceId`, but only effective in debug mode.
#[derive(Debug, Default)]
pub struct DebugInstanceId {
    #[cfg(debug_assertions)]
    inner: InstanceId,
}

impl DebugInstanceId {
    /// Creates a new, unique instance id that is only used with debug
    /// assertions enabled.
    pub fn new() -> Self {
        Default::default()
    }

    /// If debug assertions are enabled, asserts instance id and reference are
    /// equal. Otherwise, does nothing.
    #[inline]
    pub fn debug_assert_eq(&self, reference: &DebugReference) {
        #![allow(unused)]

        #[cfg(debug_assertions)]
        self.inner.assert_eq(&reference.reference);
    }

    /// Creates a "reference" of this instance id. This is essentially like
    /// cloning, but `InstanceId`s don't implement `Clone` on purpose since
    /// values caring it should never be cloned.
    ///
    /// No-op if debug assertions are disabled.
    #[inline]
    pub fn reference(&self) -> DebugReference {
        DebugReference {
            #[cfg(debug_assertions)]
            reference: self.inner.reference(),
        }
    }

    /// Returns the inner instance id.
    ///
    /// This function is only available if debug assertion are enabled.
    #[cfg(debug_assertions)]
    pub fn inner(&self) -> &InstanceId {
        &self.inner
    }
}

/// A reference to a `DebugInstanceId`.
#[derive(Debug, Default)]
pub struct DebugReference {
    #[cfg(debug_assertions)]
    reference: Reference,
}

impl DebugReference {
    /// If debug assertions are enabled, asserts both references are equal.
    /// Otherwise, does nothing.
    #[inline]
    pub fn debug_assert_eq(&self, reference: &DebugReference) {
        #![allow(unused)]

        #[cfg(debug_assertions)]
        self.inner().assert_eq(reference.inner());
    }

    /// Returns the inner reference.
    ///
    /// This function is only available if debug assertion are enabled.
    #[cfg(debug_assertions)]
    pub fn inner(&self) -> &Reference {
        &self.reference
    }
}

/// A unique ID that can be used to assert two objects refer to another common
/// object.
///
/// Example:
///
/// We have an allocator type which allocates `Foo`s. Some operations could
/// cause bugs if two `Foo`s from different allocators are used; `InstanceId`
/// can assert that both are from the same allocator by comparing their
/// `InstanceId`.
#[derive(Debug, Default)]
pub struct InstanceId {
    inner: Arc<u8>,
}

impl InstanceId {
    /// Creates a new, unique instance id.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns the unique `usize` representation which is used for all the
    /// assertions.
    #[inline]
    pub fn as_usize(&self) -> usize {
        self.inner.as_ref() as *const _ as usize
    }

    /// Check if `self` and `reference` are equal, panic otherwise.
    #[inline]
    pub fn assert_eq(&self, reference: &Reference) {
        assert_eq!(
            self, reference,
            "`InstanceId` and the reference are not equal"
        );
    }

    /// If debug assertions are enabled, check if `self` and `reference` are
    /// equal, panic otherwise.
    #[inline]
    pub fn debug_assert_eq(&self, reference: &Reference) {
        debug_assert_eq!(
            self, reference,
            "`InstanceId` and the reference are not equal"
        );
    }

    /// Creates a "reference" of this instance id. This is essentially like
    /// cloning, but `InstanceId`s don't implement `Clone` on purpose since
    /// values caring it should never be cloned.
    #[inline]
    pub fn reference(&self) -> Reference {
        Reference {
            instance_id: InstanceId {
                inner: self.inner.clone(),
            },
        }
    }
}

impl PartialEq<Reference> for InstanceId {
    #[inline]
    fn eq(&self, reference: &Reference) -> bool {
        self.as_usize() == reference.as_usize()
    }
}

/// A reference to an `InstanceId`.
#[derive(Debug, Default)]
pub struct Reference {
    instance_id: InstanceId,
}

impl Reference {
    /// Check if `self` and `other` are equal, panic otherwise.
    #[inline]
    pub fn assert_eq(&self, reference: &Reference) {
        assert_eq!(
            self, reference,
            "References do not point to the same `InstanceId`"
        );
    }

    /// Check if `self` and `other` are equal, panic otherwise.
    #[inline]
    pub fn debug_assert_eq(&self, reference: &Reference) {
        debug_assert_eq!(
            self, reference,
            "References do not point to the same `InstanceId`"
        );
    }

    /// Returns the `usize` representation of the referenced instance id which
    /// is used for all the assertions.
    #[inline]
    pub fn as_usize(&self) -> usize {
        self.instance_id.as_usize()
    }
}

impl Eq for Reference {}

impl PartialEq for Reference {
    fn eq(&self, other: &Reference) -> bool {
        self.as_usize() == other.as_usize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let a = InstanceId::new();
        let b = InstanceId::new();

        let a1 = a.reference();
        let a2 = a.reference();
        let a3 = a.reference();

        a.assert_eq(&a1);
        a.assert_eq(&a2);
        a.assert_eq(&a3);

        a2.assert_eq(&a3);

        let b1 = b.reference();

        assert_ne!(a1, b1);
        assert_ne!(a3, b1);

        b.assert_eq(&b1);
    }

    #[test]
    fn debug() {
        let a = DebugInstanceId::new();
        let b = DebugInstanceId::new();

        let a1 = a.reference();
        let a2 = a.reference();
        let a3 = a.reference();

        a.debug_assert_eq(&a1);
        a.debug_assert_eq(&a2);
        a.debug_assert_eq(&a3);

        a2.debug_assert_eq(&a3);

        let b1 = b.reference();

        #[cfg(debug_assertions)]
        assert_ne!(a1.inner(), b1.inner());
        #[cfg(debug_assertions)]
        assert_ne!(a3.inner(), b1.inner());

        b.debug_assert_eq(&b1);
    }
}
