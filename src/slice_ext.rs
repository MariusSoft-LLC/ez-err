//! Slice extensions that integrate well with the error handling system.

use crate::core::*;
use crate::flc;
use std::ops;

/// Extension trait for slices.
pub trait SliceExt<I, O>
where
    O: ?Sized,
{
    /// Returns a reference to an element or subslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the element at that
    ///   position or [`Err(_)`] if out of bounds.
    /// - If given a range, returns the subslice corresponding to that range,
    ///   or [`Err(_)`] if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ez_err::prelude::*;
    /// let v = [10, 40, 30];
    /// assert_eq!(Ok(&40), v.eget(1));
    /// assert_eq!(Ok(&[10, 40][..]), v.eget(0..2));
    /// assert_eq!(&ErrorType::IndexOutOfBounds(3, 3), v.eget(3).err().unwrap().ty());
    /// assert_eq!(&ErrorType::RangeOutOfBounds(0, 4, 3), v.eget(0..4).err().unwrap().ty());
    /// ```
    fn eget(&self, index: I) -> Result<&O>;
}

/// Mutable version of [`SliceExt`].
pub trait SliceExtMut<I, O>
where
    O: ?Sized,
{
    /// Returns a mutable reference to an element or subslice depending on the
    /// type of index (see [`eget`]) or [`Err(_)`] if the index is out of bounds.
    ///
    /// [`eget`]: SliceExt::eget
    ///
    /// # Examples
    ///
    /// ```
    /// # use ez_err::prelude::*;
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Ok(elem) = x.eget_mut(1) {
    ///     *elem = 42;
    /// }
    /// assert_eq!(x, &[0, 42, 2]);
    /// ```
    fn eget_mut(&mut self, index: I) -> Result<&mut O>;
}

impl<T> SliceExt<usize, T> for [T] {
    #[inline]
    fn eget(&self, index: usize) -> Result<&T> {
        if index < self.len() {
            Ok(unsafe { self.get_unchecked(index) })
        } else {
            Err(EzError::new(ErrorType::IndexOutOfBounds(index, self.len()))).loc(flc!())
        }
    }
}

impl<'a, T> SliceExt<usize, T> for &'a [T] {
    #[inline]
    fn eget(&self, index: usize) -> Result<&T> {
        if index < self.len() {
            Ok(unsafe { self.get_unchecked(index) })
        } else {
            Err(EzError::new(ErrorType::IndexOutOfBounds(index, self.len()))).loc(flc!())
        }
    }
}

impl<T> SliceExtMut<usize, T> for [T] {
    #[inline]
    fn eget_mut(&mut self, index: usize) -> Result<&mut T> {
        if index < self.len() {
            Ok(unsafe { self.get_unchecked_mut(index) })
        } else {
            Err(EzError::new(ErrorType::IndexOutOfBounds(index, self.len()))).loc(flc!())
        }
    }
}

impl<'a, T> SliceExtMut<usize, T> for &'a mut [T] {
    #[inline]
    fn eget_mut(&mut self, index: usize) -> Result<&mut T> {
        if index < self.len() {
            Ok(unsafe { self.get_unchecked_mut(index) })
        } else {
            Err(EzError::new(ErrorType::IndexOutOfBounds(index, self.len()))).loc(flc!())
        }
    }
}

impl<T> SliceExt<ops::Range<usize>, [T]> for [T] {
    #[inline]
    fn eget(&self, index: ops::Range<usize>) -> Result<&[T]> {
        if index.start > index.end {
            Err(EzError::new(ErrorType::InvalidRange)).loc(flc!())
        } else if index.start >= self.len() || index.end > self.len() {
            Err(EzError::new(ErrorType::RangeOutOfBounds(
                index.start,
                index.end,
                self.len(),
            )))
            .loc(flc!())
        } else {
            Ok(unsafe { self.get_unchecked(index) })
        }
    }
}

impl<'a, T> SliceExt<ops::Range<usize>, [T]> for &'a [T] {
    #[inline]
    fn eget(&self, index: ops::Range<usize>) -> Result<&[T]> {
        if index.start > index.end {
            Err(EzError::new(ErrorType::InvalidRange)).loc(flc!())
        } else if index.start >= self.len() || index.end > self.len() {
            Err(EzError::new(ErrorType::RangeOutOfBounds(
                index.start,
                index.end,
                self.len(),
            )))
            .loc(flc!())
        } else {
            Ok(unsafe { self.get_unchecked(index) })
        }
    }
}

impl<T> SliceExtMut<ops::Range<usize>, [T]> for [T] {
    #[inline]
    fn eget_mut(&mut self, index: ops::Range<usize>) -> Result<&mut [T]> {
        if index.start > index.end {
            Err(EzError::new(ErrorType::InvalidRange)).loc(flc!())
        } else if index.start >= self.len() || index.end > self.len() {
            Err(EzError::new(ErrorType::RangeOutOfBounds(
                index.start,
                index.end,
                self.len(),
            )))
            .loc(flc!())
        } else {
            Ok(unsafe { self.get_unchecked_mut(index) })
        }
    }
}

impl<'a, T> SliceExtMut<ops::Range<usize>, [T]> for &'a mut [T] {
    #[inline]
    fn eget_mut(&mut self, index: ops::Range<usize>) -> Result<&mut [T]> {
        if index.start > index.end {
            Err(EzError::new(ErrorType::InvalidRange)).loc(flc!())
        } else if index.start >= self.len() || index.end > self.len() {
            Err(EzError::new(ErrorType::RangeOutOfBounds(
                index.start,
                index.end,
                self.len(),
            )))
            .loc(flc!())
        } else {
            Ok(unsafe { self.get_unchecked_mut(index) })
        }
    }
}

impl<T> SliceExt<ops::RangeTo<usize>, [T]> for [T] {
    #[inline]
    fn eget(&self, index: ops::RangeTo<usize>) -> Result<&[T]> {
        self.eget(0..index.end).loc(flc!())
    }
}

impl<'a, T> SliceExt<ops::RangeTo<usize>, [T]> for &'a [T] {
    #[inline]
    fn eget(&self, index: ops::RangeTo<usize>) -> Result<&[T]> {
        self.eget(0..index.end).loc(flc!())
    }
}

impl<T> SliceExtMut<ops::RangeTo<usize>, [T]> for [T] {
    #[inline]
    fn eget_mut(&mut self, index: ops::RangeTo<usize>) -> Result<&mut [T]> {
        self.eget_mut(0..index.end).loc(flc!())
    }
}

impl<'a, T> SliceExtMut<ops::RangeTo<usize>, [T]> for &'a mut [T] {
    #[inline]
    fn eget_mut(&mut self, index: ops::RangeTo<usize>) -> Result<&mut [T]> {
        self.eget_mut(0..index.end).loc(flc!())
    }
}

impl<T> SliceExt<ops::RangeFrom<usize>, [T]> for [T] {
    #[inline]
    fn eget(&self, index: ops::RangeFrom<usize>) -> Result<&[T]> {
        self.eget(index.start..self.len()).loc(flc!())
    }
}

impl<'a, T> SliceExt<ops::RangeFrom<usize>, [T]> for &'a [T] {
    #[inline]
    fn eget(&self, index: ops::RangeFrom<usize>) -> Result<&[T]> {
        self.eget(index.start..self.len()).loc(flc!())
    }
}

impl<T> SliceExtMut<ops::RangeFrom<usize>, [T]> for [T] {
    #[inline]
    fn eget_mut(&mut self, index: ops::RangeFrom<usize>) -> Result<&mut [T]> {
        self.eget_mut(index.start..self.len()).loc(flc!())
    }
}

impl<'a, T> SliceExtMut<ops::RangeFrom<usize>, [T]> for &'a mut [T] {
    #[inline]
    fn eget_mut(&mut self, index: ops::RangeFrom<usize>) -> Result<&mut [T]> {
        self.eget_mut(index.start..self.len()).loc(flc!())
    }
}

impl<T> SliceExt<ops::RangeFull, [T]> for [T] {
    #[inline]
    fn eget(&self, _: ops::RangeFull) -> Result<&[T]> {
        Ok(self)
    }
}

impl<'a, T> SliceExt<ops::RangeFull, [T]> for &'a [T] {
    #[inline]
    fn eget(&self, _: ops::RangeFull) -> Result<&[T]> {
        Ok(self)
    }
}

impl<T> SliceExtMut<ops::RangeFull, [T]> for [T] {
    #[inline]
    fn eget_mut(&mut self, _: ops::RangeFull) -> Result<&mut [T]> {
        Ok(self)
    }
}

impl<'a, T> SliceExtMut<ops::RangeFull, [T]> for &'a mut [T] {
    #[inline]
    fn eget_mut(&mut self, _: ops::RangeFull) -> Result<&mut [T]> {
        Ok(self)
    }
}

impl<T> SliceExt<ops::RangeInclusive<usize>, [T]> for [T] {
    #[inline]
    fn eget(&self, index: ops::RangeInclusive<usize>) -> Result<&[T]> {
        if *index.end() == usize::MAX {
            Err(EzError::new(ErrorType::InvalidRange)).loc(flc!())
        } else {
            self.eget(*index.start()..(*index.end() + 1))
        }
    }
}

impl<'a, T> SliceExt<ops::RangeInclusive<usize>, [T]> for &'a [T] {
    #[inline]
    fn eget(&self, index: ops::RangeInclusive<usize>) -> Result<&[T]> {
        if *index.end() == usize::MAX {
            Err(EzError::new(ErrorType::InvalidRange)).loc(flc!())
        } else {
            self.eget(*index.start()..(*index.end() + 1))
        }
    }
}

impl<T> SliceExtMut<ops::RangeInclusive<usize>, [T]> for [T] {
    #[inline]
    fn eget_mut(&mut self, index: ops::RangeInclusive<usize>) -> Result<&mut [T]> {
        if *index.end() == usize::MAX {
            Err(EzError::new(ErrorType::InvalidRange)).loc(flc!())
        } else {
            self.eget_mut(*index.start()..(*index.end() + 1))
        }
    }
}

impl<'a, T> SliceExtMut<ops::RangeInclusive<usize>, [T]> for &'a mut [T] {
    #[inline]
    fn eget_mut(&mut self, index: ops::RangeInclusive<usize>) -> Result<&mut [T]> {
        if *index.end() == usize::MAX {
            Err(EzError::new(ErrorType::InvalidRange)).loc(flc!())
        } else {
            self.eget_mut(*index.start()..(*index.end() + 1))
        }
    }
}

impl<T> SliceExt<ops::RangeToInclusive<usize>, [T]> for [T] {
    #[inline]
    fn eget(&self, index: ops::RangeToInclusive<usize>) -> Result<&[T]> {
        self.eget(0..=index.end).loc(flc!())
    }
}

impl<'a, T> SliceExt<ops::RangeToInclusive<usize>, [T]> for &'a [T] {
    #[inline]
    fn eget(&self, index: ops::RangeToInclusive<usize>) -> Result<&[T]> {
        self.eget(0..=index.end).loc(flc!())
    }
}

impl<T> SliceExtMut<ops::RangeToInclusive<usize>, [T]> for [T] {
    #[inline]
    fn eget_mut(&mut self, index: ops::RangeToInclusive<usize>) -> Result<&mut [T]> {
        self.eget_mut(0..=index.end).loc(flc!())
    }
}

impl<'a, T> SliceExtMut<ops::RangeToInclusive<usize>, [T]> for &'a mut [T] {
    #[inline]
    fn eget_mut(&mut self, index: ops::RangeToInclusive<usize>) -> Result<&mut [T]> {
        self.eget_mut(0..=index.end).loc(flc!())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_index() {
        let arr = [6, 12, 5];

        assert_eq!(Ok(&6), arr.eget(0));
        assert_eq!(Ok(&12), arr.eget(1));
        assert_eq!(Ok(&5), arr.eget(2));
        assert!(arr.eget(3).is_err());
    }

    #[test]
    fn plain_index_ref() {
        let arr = &[6, 12, 5];

        assert_eq!(Ok(&6), arr.eget(0));
        assert_eq!(Ok(&12), arr.eget(1));
        assert_eq!(Ok(&5), arr.eget(2));
        assert!(arr.eget(3).is_err());
    }

    #[test]
    fn plain_index_mut() {
        let arr = &mut [6, 12, 5];

        assert_eq!(Ok(&mut 6), arr.eget_mut(0));
        assert_eq!(Ok(&mut 12), arr.eget_mut(1));
        assert_eq!(Ok(&mut 5), arr.eget_mut(2));
        assert!(arr.eget_mut(3).is_err());
    }

    #[test]
    fn range() {
        let arr = [6, 12, 5];

        assert_eq!(Ok(&arr[0..2]), arr.eget(0..2));
        assert_eq!(Ok(&arr[1..3]), arr.eget(1..3));
        assert!(arr.eget(1..4).is_err());
    }

    #[test]
    fn range_ref() {
        let arr = &[6, 12, 5];

        assert_eq!(Ok(&arr[0..2]), arr.eget(0..2));
        assert_eq!(Ok(&arr[1..3]), arr.eget(1..3));
        assert!(arr.eget(1..4).is_err());
    }

    #[test]
    fn range_mut() {
        let arr = &mut [6, 12, 5];
        let clone = &mut [6, 12, 5];

        assert_eq!(Ok(&mut clone[0..2]), arr.eget_mut(0..2));
        assert_eq!(Ok(&mut clone[1..3]), arr.eget_mut(1..3));
        assert!(arr.eget_mut(1..4).is_err());
    }

    #[test]
    fn range_to() {
        let arr = [6, 12, 5];

        assert_eq!(Ok(&arr[..2]), arr.eget(..2));
        assert_eq!(Ok(&arr[..3]), arr.eget(..3));
        assert!(arr.eget(..4).is_err());
    }

    #[test]
    fn range_to_ref() {
        let arr = &[6, 12, 5];

        assert_eq!(Ok(&arr[..2]), arr.eget(..2));
        assert_eq!(Ok(&arr[..3]), arr.eget(..3));
        assert!(arr.eget(..4).is_err());
    }

    #[test]
    fn range_to_mut() {
        let arr = &mut [6, 12, 5];
        let clone = &mut [6, 12, 5];

        assert_eq!(Ok(&mut clone[..2]), arr.eget_mut(..2));
        assert_eq!(Ok(&mut clone[..3]), arr.eget_mut(..3));
        assert!(arr.eget_mut(..4).is_err());
    }

    #[test]
    fn range_from() {
        let arr = [6, 12, 5];

        assert_eq!(Ok(&arr[1..]), arr.eget(1..));
        assert_eq!(Ok(&arr[2..]), arr.eget(2..));
        assert!(arr.eget(3..).is_err());
    }

    #[test]
    fn range_from_ref() {
        let arr = &[6, 12, 5];

        assert_eq!(Ok(&arr[1..]), arr.eget(1..));
        assert_eq!(Ok(&arr[2..]), arr.eget(2..));
        assert!(arr.eget(3..).is_err());
    }

    #[test]
    fn range_from_mut() {
        let arr = &mut [6, 12, 5];
        let clone = &mut [6, 12, 5];

        assert_eq!(Ok(&mut clone[1..]), arr.eget_mut(1..));
        assert_eq!(Ok(&mut clone[2..]), arr.eget_mut(2..));
        assert!(arr.eget_mut(3..).is_err());
    }

    #[test]
    fn range_full() {
        let arr = [6, 12, 5];

        assert_eq!(Ok(&arr[..]), arr.eget(..));
    }

    #[test]
    fn range_full_ref() {
        let arr = &[6, 12, 5];

        assert_eq!(Ok(&arr[..]), arr.eget(..));
    }

    #[test]
    fn range_full_mut() {
        let arr = &mut [6, 12, 5];
        let clone = &mut [6, 12, 5];

        assert_eq!(Ok(&mut clone[..]), arr.eget_mut(..));
    }

    #[test]
    fn range_inclusive() {
        let arr = [6, 12, 5];

        assert_eq!(Ok(&arr[0..=1]), arr.eget(0..=1));
        assert_eq!(Ok(&arr[1..=2]), arr.eget(1..=2));
        assert!(arr.eget(1..=3).is_err());
    }

    #[test]
    fn range_inclusive_ref() {
        let arr = &[6, 12, 5];

        assert_eq!(Ok(&arr[0..=1]), arr.eget(0..=1));
        assert_eq!(Ok(&arr[1..=2]), arr.eget(1..=2));
        assert!(arr.eget(1..=3).is_err());
    }

    #[test]
    fn range_inclusive_mut() {
        let arr = &mut [6, 12, 5];
        let clone = &mut [6, 12, 5];

        assert_eq!(Ok(&mut clone[0..=1]), arr.eget_mut(0..=1));
        assert_eq!(Ok(&mut clone[1..=2]), arr.eget_mut(1..=2));
        assert!(arr.eget_mut(1..=3).is_err());
    }

    #[test]
    fn range_to_inclusive() {
        let arr = [6, 12, 5];

        assert_eq!(Ok(&arr[..=1]), arr.eget(..=1));
        assert_eq!(Ok(&arr[..=2]), arr.eget(..=2));
        assert!(arr.eget(..=3).is_err());
    }

    #[test]
    fn range_to_inclusive_ref() {
        let arr = &[6, 12, 5];

        assert_eq!(Ok(&arr[..=1]), arr.eget(..=1));
        assert_eq!(Ok(&arr[..=2]), arr.eget(..=2));
        assert!(arr.eget(..=3).is_err());
    }

    #[test]
    fn range_to_inclusive_mut() {
        let arr = &mut [6, 12, 5];
        let clone = &mut [6, 12, 5];

        assert_eq!(Ok(&mut clone[..=1]), arr.eget_mut(..=1));
        assert_eq!(Ok(&mut clone[..=2]), arr.eget_mut(..=2));
        assert!(arr.eget_mut(..=3).is_err());
    }
}
