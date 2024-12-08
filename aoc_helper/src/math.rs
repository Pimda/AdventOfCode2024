pub fn positive_mod(value: i32, modulus: i32) -> i32 {
    ((value % modulus) + modulus) % modulus
}

pub fn ranges_fully_overlap(lhs: &(i32, i32), rhs: &(i32, i32)) -> bool {
    let (ll, lu) = lhs;
    let (rl, ru) = rhs;
    ll <= rl && lu >= ru || rl <= ll && ru >= lu
}

pub fn ranges_partially_overlap(lhs: &(i32, i32), rhs: &(i32, i32)) -> bool {
    let (ll, lu) = lhs;
    let (rl, ru) = rhs;
    ll <= rl && lu >= rl || ll <= ru && lu >= ru || ll < rl && lu > ru || rl < ll && ru > lu
}
