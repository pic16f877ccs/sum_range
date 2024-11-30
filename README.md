## Sum_Range

This crate provides a trait for efficiently calculating the sum of numbers within a range, including support for odd and even number sums.

#### Supported Range Types

* `Range`
* `RangeInclusive`
* `RangeTo`
* `RangeToInclusive`

#### Trait Methods

* `sum_range`: Calculates the sum of all numbers in the range.
* `sum_odd_range`: Calculates the sum of all odd numbers in the range.
* `sum_even_range`: Calculates the sum of all even numbers in the range.

#### Note

* For `RangeTo` and `RangeToInclusive`, signed integer types are not supported.

#### Usage

```rust
use sum_range::SumRange;

assert_eq!((1..11).sum_range(), 55u8);
assert_eq!((0..=12).sum_range(), 78u8);
assert_eq!((-10..12).sum_odd_range(), 11i16);
assert_eq!((-5..=5).sum_even_range(), 0i32);
```

#### License

This crate is licensed under the MIT license.
