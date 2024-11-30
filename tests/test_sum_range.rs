use sum_range::SumRange;

#[test]
fn test_sum_range() {
    assert_eq!((0..0_u8).sum_range(), (0..0).sum());
    assert_eq!((0..0_i8).sum_range(), (0..0).sum());
    assert_eq!((0..1_i8).sum_range(), (0..1).sum());
    assert_eq!((0..1_u8).sum_range(), (0..1).sum());
    assert_eq!((-1..0_i8).sum_range(), (-1..0).sum());
    assert_eq!((-1..1_i8).sum_range(), (-1..1).sum());
    assert_eq!((0..10_u16).sum_range(), (0..10).sum());
    assert_eq!((10..10_u32).sum_range(), (10..10).sum());
    assert_eq!((10..10_i32).sum_range(), (10..10).sum());
    assert_eq!((10..11_u64).sum_range(), (10..11).sum());
    assert_eq!((10..1_usize).sum_range(), (10..1).sum());
    assert_eq!((10..1_isize).sum_range(), (10..1).sum());
    assert_eq!((2..11_u128).sum_range(), (2..11).sum());
    assert_eq!((-2..11_i8).sum_range(), (-2..11).sum());
    assert_eq!((2..-1_i16).sum_range(), (2..-1).sum());
    assert_eq!((-20..110_i32).sum_range(), (-20..110).sum());
    assert_eq!((-20..0_i64).sum_range(), (-20..0).sum());
    assert_eq!((-20..-10_isize).sum_range(), (-20..-10).sum());
    assert_eq!((-20..-20_i128).sum_range(), (-20..-20).sum());
    assert_eq!((-5..-20_i8).sum_range(), (-5..-20).sum());
}

#[test]
fn test_sum_range_inc() {
    assert_eq!((0..=0_u8).sum_range(), (0..=0).sum());
    assert_eq!((0..=0_i8).sum_range(), (0..=0).sum());
    assert_eq!((1..=1_u16).sum_range(), (1..=1).sum());
    assert_eq!((1..=1_i16).sum_range(), (1..=1).sum());
    assert_eq!((20..=100u32).sum_range(), (20..=100).sum());
    assert_eq!((200..=100u32).sum_range(), (200..=100).sum());
    assert_eq!((1..=0_u64).sum_range(), (1..=0).sum());
    assert_eq!((1..=0_i64).sum_range(), (1..=0).sum());
    assert_eq!((1..=2_usize).sum_range(), (1..=2).sum());
    assert_eq!((100..=1000_u128).sum_range(), (100..=1000).sum());
    assert_eq!((10..=-2_i8).sum_range(), (10..=-2).sum());
    assert_eq!((-1..=5_i16).sum_range(), (-1..=5).sum());
    assert_eq!((-5..=-20_i32).sum_range(), (-5..=-20).sum());
    assert_eq!((-20..=-20_i128).sum_range(), (-20..=-20).sum());
    assert_eq!((-20..=-10_isize).sum_range(), (-20..=-10).sum());
    assert_eq!((-5..=-5_i32).sum_range(), (-5..=-5).sum());
}

#[test]
fn test_sum_range_to() {
    assert_eq!((..0_u8).sum_range(), (0..0).sum());
    assert_eq!((..1_u16).sum_range(), (0..1).sum());
    assert_eq!((..11_u16).sum_range(), (0..11).sum());
    assert_eq!((..255_u32).sum_range(), (0..255).sum());
    assert_eq!((..2550_u64).sum_range(), (0..2550).sum());
    assert_eq!((..2500_usize).sum_range(), (0..2500).sum());
    assert_eq!((..25000_u128).sum_range(), (0..25000).sum());
}

#[test]
fn test_sum_range_to_inc() {
    assert_eq!((..=0_u8).sum_range(), (0..=0).sum());
    assert_eq!((..=1_u16).sum_range(), (0..=1).sum());
    assert_eq!((..=255_u32).sum_range(), (0..=255).sum());
    assert_eq!((..=2550_u64).sum_range(), (0..=2550).sum());
    assert_eq!((..=2500_usize).sum_range(), (0..=2500).sum());
    assert_eq!((..=25000_u128).sum_range(), (0..=25000).sum());
}

#[test]
fn test_sum_even_range() {
    assert_eq!(
        (0..0_u8).sum_even_range(),
        (0..0).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (0..10_u16).sum_even_range(),
        (0..10).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (10..10_u32).sum_even_range(),
        (10..10).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (10..11_u64).sum_even_range(),
        (10..11).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (10..1_usize).sum_even_range(),
        (10..1).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (2..11_u128).sum_even_range(),
        (2..11).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (200..110_u32).sum_even_range(),
        (200..110).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (201..110_u32).sum_even_range(),
        (201..110).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (201..111_u32).sum_even_range(),
        (201..111).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (200..111_u32).sum_even_range(),
        (200..111).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-2..11_i8).sum_even_range(),
        (-2..11).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-1..11_i8).sum_even_range(),
        (-1..11).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-1..12_i8).sum_even_range(),
        (-1..12).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-2..12_i8).sum_even_range(),
        (-2..12).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (2..-1_i16).sum_even_range(),
        (2..-1).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-20..110_i32).sum_even_range(),
        (-20..110).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (200..110_i32).sum_even_range(),
        (200..110).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (201..110_i32).sum_even_range(),
        (201..110).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (201..111_i32).sum_even_range(),
        (201..111).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (200..111_i32).sum_even_range(),
        (200..111).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-20..0_i64).sum_even_range(),
        (-20..0).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-20..-10_isize).sum_even_range(),
        (-20..-10).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-20..-20_i128).sum_even_range(),
        (-20..-20).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-5..-20_i8).sum_even_range(),
        (-5..-20).filter(|x| { x % 2 == 0 }).sum()
    );
}

#[test]
fn test_sum_even_range_inc() {
    assert_eq!(
        (0..=0_u8).sum_even_range(),
        (0..=0).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (0..=10_u16).sum_even_range(),
        (0..=10).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (10..=10_u32).sum_even_range(),
        (10..=10).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (10..=11_u64).sum_even_range(),
        (10..=11).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (10..=1_usize).sum_even_range(),
        (10..=1).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (2..=11_u128).sum_even_range(),
        (2..=11).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-2..=11_i8).sum_even_range(),
        (-2..=11).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-1..11_i8).sum_even_range(),
        (-1..11).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-1..12_i8).sum_even_range(),
        (-1..12).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-2..12_i8).sum_even_range(),
        (-2..12).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (2..=-1_i16).sum_even_range(),
        (2..=-1).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-20..=110_i32).sum_even_range(),
        (-20..=110).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-20..=0_i64).sum_even_range(),
        (-20..=0).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-20..=-10_isize).sum_even_range(),
        (-20..=-10).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-20..=-20_i128).sum_even_range(),
        (-20..=-20).filter(|x| { x % 2 == 0 }).sum()
    );
    assert_eq!(
        (-5..=-20_i8).sum_even_range(),
        (-5..=-20).filter(|x| { x % 2 == 0 }).sum()
    );
}

#[test]
fn test_sum_even_range_to() {
    assert_eq!((..0_u8).sum_even_range(), (0..0).step_by(2).sum::<_>());
    assert_eq!((..1_u16).sum_even_range(), (0..1).step_by(2).sum::<_>());
    assert_eq!((..2_u32).sum_even_range(), (0..2).step_by(2).sum::<_>());
    assert_eq!((..5_usize).sum_even_range(), (0..5).step_by(2).sum::<_>());
    assert_eq!((..6_u64).sum_even_range(), (0..6).step_by(2).sum::<_>());
    assert_eq!((..7_u128).sum_even_range(), (0..7).step_by(2).sum::<_>());
}

#[test]
fn test_sum_even_range_to_inc() {
    assert_eq!((..=0_u8).sum_even_range(), (0..=0).step_by(2).sum::<_>());
    assert_eq!((..=1_u16).sum_even_range(), (0..=1).step_by(2).sum::<_>());
    assert_eq!((..=2_u32).sum_even_range(), (0..=2).step_by(2).sum::<_>());
    assert_eq!((..=10_u64).sum_even_range(), (0..=10).step_by(2).sum::<_>());
    assert_eq!(
        (..=11_usize).sum_even_range(),
        (0..=11).step_by(2).sum::<_>()
    );
    assert_eq!(
        (..=12_u128).sum_even_range(),
        (0..=12).step_by(2).sum::<_>()
    );
}

#[test]
fn test_sum_odd_range() {
    assert_eq!(
        (0..0_u8).sum_odd_range(),
        (0..0).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (0..10_u16).sum_odd_range(),
        (0..10).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (10..10_u32).sum_odd_range(),
        (10..10).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (10..11_u64).sum_odd_range(),
        (10..11).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (10..1_usize).sum_odd_range(),
        (10..1).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (2..11_u128).sum_odd_range(),
        (2..11).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (-2..11_i8).sum_odd_range(),
        (-2..11).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (2..-1_i16).sum_odd_range(),
        (2..-1).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (-20..110_i32).sum_odd_range(),
        (-20..110).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (-20..0_i64).sum_odd_range(),
        (-20..0).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (-20..-10_isize).sum_odd_range(),
        (-20..-10).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (-20..-20_i128).sum_odd_range(),
        (-20..-20).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (-5..-20_i8).sum_odd_range(),
        (-5..-20).filter(|x| { x % 2 != 0 }).sum()
    );

    assert_eq!(
        (..0_u8).sum_odd_range(),
        (0..0).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..1_usize).sum_odd_range(),
        (0..1).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..2_u16).sum_odd_range(),
        (0..2).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..10_u32).sum_odd_range(),
        (0..10).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..11_u64).sum_odd_range(),
        (0..11).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..12_u128).sum_odd_range(),
        (0..12).filter(|x| { x % 2 != 0 }).sum()
    );
}

#[test]
fn test_sum_odd_range_to() {
    assert_eq!(
        (..0_u8).sum_odd_range(),
        (0..0).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..1_usize).sum_odd_range(),
        (0..1).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..2_u16).sum_odd_range(),
        (0..2).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..10_u32).sum_odd_range(),
        (0..10).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..11_u64).sum_odd_range(),
        (0..11).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..12_u128).sum_odd_range(),
        (0..12).filter(|x| { x % 2 != 0 }).sum()
    );
}

#[test]
fn test_sum_odd_range_to_inc() {
    assert_eq!(
        (..=0_u8).sum_odd_range(),
        (0..=0).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..=1_usize).sum_odd_range(),
        (0..=1).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..=2_u16).sum_odd_range(),
        (0..=2).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..=10_u32).sum_odd_range(),
        (0..=10).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..=11_u64).sum_odd_range(),
        (0..=11).filter(|x| { x % 2 != 0 }).sum()
    );
    assert_eq!(
        (..=12_u128).sum_odd_range(),
        (0..=12).filter(|x| { x % 2 != 0 }).sum()
    );
}
