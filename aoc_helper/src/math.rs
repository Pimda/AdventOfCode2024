/// Returns the modulus of value, but it's always returned on the same side of 0 as modulus
/// # Examples
///```
///use aoc_helper::math::positive_mod;
///assert_eq!(1, positive_mod(-3, 4));
///assert_eq!(3, positive_mod(3, 4));
///assert_eq!(-1, positive_mod(3, -4));
///assert_eq!(-3, positive_mod(-3, -4));
///```
pub fn positive_mod(value: i32, modulus: i32) -> i32 {
    ((value % modulus) + modulus) % modulus
}

///Returns true if the tuples completely overlap
/// # Examples
///```
///use aoc_helper::math::ranges_fully_overlap;
///assert_eq!(true, ranges_fully_overlap(&(0, 6), &(2, 4)));
///assert_eq!(true, ranges_fully_overlap(&(2, 4), &(0, 6)));
///```
///Returns false if the tuples have a partial or no overlap, independent of the order of the tuples
///```
///use aoc_helper::math::ranges_fully_overlap;
///assert_eq!(false, ranges_fully_overlap(&(0, 3), &(3, 6)));
///assert_eq!(false, ranges_fully_overlap(&(3, 6), &(0, 3)));
///assert_eq!(false, ranges_fully_overlap(&(0, 3), &(4, 7)))
///```
#[must_use]
pub fn ranges_fully_overlap(lhs: &(i32, i32), rhs: &(i32, i32)) -> bool {
    let (ll, lu) = lhs;
    let (rl, ru) = rhs;
    ll <= rl && lu >= ru || rl <= ll && ru >= lu
}

///Returns true if the tuples at least have a partial overlap, independent of the order of the tuples
/// # Examples
///```
///use aoc_helper::math::ranges_partially_overlap;
///assert_eq!(true, ranges_partially_overlap(&(0, 3), &(3, 6)));
///assert_eq!(true, ranges_partially_overlap(&(3, 6), &(0, 3)));
///```
///Returns true if the tuples completely overlap, independent of the order of the tuples
///```
///use aoc_helper::math::ranges_partially_overlap;
///assert_eq!(true, ranges_partially_overlap(&(0, 6), &(2, 4)));
///assert_eq!(true, ranges_partially_overlap(&(2, 4), &(0, 6)));
///```
///Returns false if the tuples have no overlap
///```
///use aoc_helper::math::ranges_partially_overlap;
///assert_eq!(false, ranges_partially_overlap(&(0, 3), &(4, 7)))
///```
#[must_use]
pub fn ranges_partially_overlap(lhs: &(i32, i32), rhs: &(i32, i32)) -> bool {
    let (ll, lu) = lhs;
    let (rl, ru) = rhs;
    ll <= rl && lu >= rl || ll <= ru && lu >= ru || ll < rl && lu > ru || rl < ll && ru > lu
}
