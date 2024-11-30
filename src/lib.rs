#![crate_name = "sum_range"]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]
//! This crate provides a trait for calculating the sum of numbers in a range.
//!
//! The trait is implemented for the following types:
//! - [`Range`]
//! - [`RangeInclusive`]
//! - [`RangeTo`]
//! - [`RangeToInclusive`]
//!
//! The trait provides three methods:
//! - [`sum_range`][crate::SumRange::sum_range]
//! - [`sum_odd_range`][crate::SumRange::sum_odd_range]
//! - [`sum_even_range`][crate::SumRange::sum_even_range]
//!
//! The methods calculate the sum of the numbers in the range,
//! the sum of the odd numbers in the range, and the sum of the even numbers in the range, respectively.
//!
//! <div class="warning">
//!
//! **NOTE:** For [`RangeTo`] and [`RangeToInclusive`], signed integer types are not supported.
//!
//! </div>
//!
//! # Examples
//!
//! ```rust
//! use sum_range::SumRange;
//!
//! assert_eq!((1..11).sum_range(), 55u8);
//! assert_eq!((0..=12).sum_range(), 78u8);
//! assert_eq!((-10..12).sum_odd_range(), 11i16);
//! assert_eq!((-5..=5).sum_even_range(), 0i32);
//! ```

use core::ops::{Div, Mul, Rem, Sub};
use core::ops::{Range, RangeInclusive, RangeTo, RangeToInclusive};
use num::{Integer, Unsigned};
use num_convert::IntoAs;

/// A trait for calculating the sum of numbers in a range.
pub trait SumRange<T> {
    /// The sum of the numbers in the range.
    ///
    /// # Panics
    ///
    /// Panic when the calculation overflows in debug mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sum_range::SumRange;
    ///
    /// assert_eq!((1..11).sum_range(), 55u32);
    /// assert_eq!((-10..=10).sum_range(), 0i32);
    /// ```
    fn sum_range(&self) -> T;

    /// The sum of the odd numbers in the range.
    ///
    /// # Panics
    ///
    /// Panic when the calculation overflows in debug mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sum_range::SumRange;
    ///
    /// assert_eq!((3..15).sum_odd_range(), 48u16);
    /// assert_eq!((2..14).sum_odd_range(), 48u16);
    /// ```
    fn sum_odd_range(&self) -> T;

    /// The sum of the even numbers in the range.
    ///
    /// # Panics
    ///
    /// Panic when the calculation overflows in debug mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sum_range::SumRange;
    ///
    /// assert_eq!((-2..=12).sum_even_range(), 40i64);
    /// assert_eq!((-3..=13).sum_even_range(), 40i64);
    /// ```
    fn sum_even_range(&self) -> T;
}

impl<T> SumRange<T> for Range<T>
where
    T: Integer + Copy,
    u8: IntoAs<T>,
{
    #[inline]
    fn sum_range(&self) -> T {
        if self.start >= self.end {
            return 0.into_as();
        }

        (self.end + self.start - 1.into_as()) * (self.end - self.start) / 2.into_as()
    }

    #[inline]
    fn sum_odd_range(&self) -> T {
        if self.start > self.end {
            return 0.into_as();
        }

        let tmp_end = if self.end >= 0.into_as() {
            self.end
        } else {
            self.end - 1.into_as()
        } / 2.into_as();

        let tmp_start = if self.start >= 0.into_as() {
            self.start
        } else {
            self.start - 1.into_as()
        } / 2.into_as();

        tmp_end * tmp_end - tmp_start * tmp_start
    }

    #[inline]
    fn sum_even_range(&self) -> T {
        if self.start >= self.end {
            return 0.into_as();
        }

        (if self.end % 2.into_as() == 0.into_as() {
            (self.end - 1.into_as()) * (self.end - 1.into_as())
        } else {
            self.end * self.end
        }) / 4.into_as()
            - sum_even_range_to(self.start)
    }
}

impl<T> SumRange<T> for RangeInclusive<T>
where
    T: Integer + Copy,
    u8: IntoAs<T>,
{
    #[inline]
    fn sum_range(&self) -> T {
        if self.start() > self.end() {
            return 0.into_as();
        }

        let start = *self.start();
        let end = *self.end();

        (end + start) * (end - start + 1.into_as()) / 2.into_as()
    }

    #[inline]
    fn sum_odd_range(&self) -> T {
        if self.start() > self.end() {
            return 0.into_as();
        }

        let start = *self.start();
        let end = *self.end();

        let tmp_end = if end >= 0.into_as() {
            end + 1.into_as()
        } else {
            end
        } / 2.into_as();

        let tmp_start = if start >= 0.into_as() {
            start
        } else {
            start - 1.into_as()
        } / 2.into_as();

        (tmp_end + tmp_start) * (tmp_end - tmp_start)
    }

    #[inline]
    fn sum_even_range(&self) -> T {
        if self.start() > self.end() {
            return 0.into_as();
        }

        let end = *self.end();
        let start = *self.start();

        (if end % 2.into_as() == 0.into_as() {
            (end + 1.into_as()) * (end + 1.into_as())
        } else {
            end * end
        }) / 4.into_as()
            - sum_even_range_to(start)
    }
}

impl<T> SumRange<T> for RangeTo<T>
where
    T: Unsigned + Copy,
    u8: IntoAs<T>,
{
    #[inline]
    fn sum_range(&self) -> T {
        let start = 0.into_as();
        if start == self.end {
            return 0.into_as();
        }

        ((self.end - 1.into_as()) * self.end) / 2.into_as()
    }

    #[inline]
    fn sum_odd_range(&self) -> T {
        let tmp = self.end / 2.into_as();
        tmp * tmp
    }

    #[inline]
    fn sum_even_range(&self) -> T {
        sum_even_range_to(self.end)
    }
}

impl<T> SumRange<T> for RangeToInclusive<T>
where
    T: Unsigned + Copy,
    u8: IntoAs<T>,
{
    #[inline]
    fn sum_range(&self) -> T {
        self.end * (self.end + 1.into_as()) / 2.into_as()
    }

    #[inline]
    fn sum_odd_range(&self) -> T {
        let tmp = (self.end + 1.into_as()) / 2.into_as();
        tmp * tmp
    }

    #[inline]
    fn sum_even_range(&self) -> T {
        (if self.end % 2.into_as() == 0.into_as() {
            (self.end + 1.into_as()) * (self.end + 1.into_as())
        } else {
            self.end * self.end
        }) / 4.into_as()
    }
}

#[inline]
fn sum_even_range_to<T>(end: T) -> T
where
    T: Div<Output = T> + Mul<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + Copy,
    u8: IntoAs<T>,
{
    let zero = 0.into_as();
    if zero == end {
        return zero;
    }

    (if end % 2.into_as() == 0.into_as() {
        (end - 1.into_as()) * (end - 1.into_as())
    } else {
        end * end - 1.into_as()
    }) / 4.into_as()
}
