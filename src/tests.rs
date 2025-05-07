use crate::*;

const LEN: usize = 10240;

#[test]
fn zero_data() {
    let all_zero_buf = [0u8; LEN];
    let ret = dbg!(Entest::test(&all_zero_buf));
    assert_eq!(ret.samples(), LEN as u64);
    assert_eq!(ret.chi(), &dec!(2611200.0));
    assert_eq!(ret.chi_prob(), &dec!(0.0));
    assert_eq!(ret.mc(), &dec!(4.0));
    assert_eq!(ret.mean(), &dec!(0.0));
    assert!(ret.sc().is_nan());
    assert_eq!(ret.shannon(), &dec!(0.0));
}

#[test]
fn std_random_state() {
    let mut std_random_state_buf = [0u8; LEN];
    let mut b;
    for chunk in std_random_state_buf.chunks_mut(8) {
        use std::hash::{Hasher, BuildHasher};
        use std::collections::hash_map::RandomState;
        b = RandomState::new().build_hasher().finish().to_ne_bytes();
        chunk[.. b.len()].copy_from_slice(&b);
    }
    let ret = dbg!(Entest::test(&std_random_state_buf));
    assert_eq!(ret.samples(), LEN as u64);
}

#[test]
fn predefined() {
    let predefined_data_buf = include_bytes!("tests.rand");

    let ret = dbg!(Entest::test(predefined_data_buf));
    assert_eq!(ret.samples(), LEN as u64);
    assert_eq!(ret.chi(), &dec!(293.0));
    assert_eq!(ret.chi_prob(), &dec!(0.05104028823948067937));
    assert_eq!(ret.mc(), &dec!(3.146541617819460727));
    assert_eq!(ret.mean(), &dec!(128.2646484375));
    assert_eq!(ret.sc(), &dec!(0.014631937126484728929));
    assert_eq!(ret.shannon(), &dec!(7.979134079303539237));
}
