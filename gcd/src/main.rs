use owi_sym::{Symbolic, SymbolicInBounds};

fn test_u8_iso(x: u8, y: u8) {
    // owi_sym::assume(x < 10);
    // owi_sym::assume(y < x);
    assert_eq!(gcd::binary_u8(x, y), gcd::euclid_u8(x, y))
}

fn test_binary_u8_spec(x: u8, y: u8) {
    owi_sym::assume(x > 0);
    owi_sym::assume(y > 0);
    let gcd = gcd::binary_u8(x, y);
    assert_eq!(x % gcd, 0);
    assert_eq!(y % gcd, 0);
    owi_sym::assume(gcd < u8::MAX);
    let div = u8::symbol_in((gcd + 1)..x.min(y));
    assert!((x % div != 0) || (y % div != 0));
}

fn test_euclid_u8_spec(x: u8, y: u8) {
    owi_sym::assume(x > 0);
    owi_sym::assume(y > 0);
    let gcd = gcd::euclid_u8(x, y);
    assert_eq!(x % gcd, 0);
    assert_eq!(y % gcd, 0);
    owi_sym::assume(gcd < u8::MAX);
    let div = u8::symbol_in((gcd + 1)..x.min(y));
    assert!((x % div != 0) || (y % div != 0));
}


fn main() {
    owi_sym::harness::execute_symbolically(test_euclid_u8_spec as fn(_, _) -> _);
}
