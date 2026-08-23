use rustines_core::utils::bit_utils::{BitCount, BitIndex};

const INDEX: BitIndex = BitIndex::new(7);
const COUNT: BitCount = BitCount::new(8);

#[test]
fn accepts_valid_const_values() {
    let _: BitIndex = INDEX;
    let _: BitCount = COUNT;
}

#[test]
fn rejects_invalid_const_values() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/invalid_bit_*.rs");
}
