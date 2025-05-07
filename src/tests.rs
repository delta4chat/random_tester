use crate::*;

#[test]
fn zero_data() {
    let mut test = dbg!(Entest::new());
    dbg!(test.update(b"\x00".repeat(200).as_ref()));
    dbg!(test.finalize());
}
