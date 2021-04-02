#![no_std]
#![deny(clippy::all, clippy::pedantic, rust_2018_idioms)]
#![feature(ptr_metadata)]

use core::convert::TryFrom;
use core::marker::PhantomData;
use core::ptr::Pointee;

/// # `OnlyNull`
///
/// A new type of pointer that, intuitively, is only null. You can easily pass this to any function
/// you want, and it will know that it is always null. This is at least ``1 / 0`` times faster, and
/// that is a number so special it's incalcuable.
///
/// If you need to convert into a traditional form of pointer, that is trivial since [`OnlyNull`]
/// implements [`Into`] to these types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OnlyNull<T: ?Sized>
where
    T: Pointee,
{
    meta: T::Metadata,
    _phantom: PhantomData<*const T>,
}

impl<T: ?Sized> OnlyNull<T>
where
    T: Pointee<Metadata = ()>,
{
    /// Creates a null pointer
    #[inline]
    #[must_use]
    pub fn null() -> Self {
        Self {
            meta: (),
            _phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> OnlyNull<T>
where
    T: Pointee,
{
    /// Casts to a pointer of another type.
    #[must_use]
    pub fn cast<U: ?Sized>(self) -> OnlyNull<U>
    where
        U: Pointee<Metadata = T::Metadata>,
    {
        OnlyNull {
            meta: self.meta,
            _phantom: PhantomData,
        }
    }
}

impl<T> From<OnlyNull<T>> for *const T {
    #[inline]
    #[must_use]
    fn from(_: OnlyNull<T>) -> Self {
        core::ptr::null()
    }
}

impl<T> From<OnlyNull<T>> for *mut T {
    #[inline]
    #[must_use]
    fn from(_: OnlyNull<T>) -> Self {
        core::ptr::null_mut()
    }
}

impl<T> TryFrom<*const T> for OnlyNull<T> {
    type Error = ConvertToOnlyNullError;

    fn try_from(ptr: *const T) -> Result<Self, Self::Error> {
        if ptr.is_null() {
            Ok(Self {
                meta: ptr.to_raw_parts().1,
                _phantom: PhantomData,
            })
        } else {
            Err(ConvertToOnlyNullError)
        }
    }
}

impl<T> TryFrom<*mut T> for OnlyNull<T> {
    type Error = ConvertToOnlyNullError;

    fn try_from(ptr: *mut T) -> Result<Self, Self::Error> {
        if ptr.is_null() {
            Ok(Self {
                meta: ptr.to_raw_parts().1,
                _phantom: PhantomData,
            })
        } else {
            Err(ConvertToOnlyNullError)
        }
    }
}

/// An error type for converting to an [`OnlyNull`] pointer from a normal raw pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConvertToOnlyNullError;
