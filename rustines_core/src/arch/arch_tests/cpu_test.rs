#[cfg(test)]
mod tests {
    use crate::arch::arch_tests::common::tests::setup_tests;

    #[test]
    fn test_push_pop_peek() {
        let mut cpu = setup_tests();

        cpu.push8(0xAB);

        assert_eq!(0xAB, cpu.peek8());

        let v = cpu.pop8();

        assert_eq!(0xAB, v);

        cpu.push8(0xBC);
        cpu.push8(0xCD);

        assert_eq!(0xCD, cpu.peek8());

        let v = cpu.pop8();

        assert_eq!(0xCD, v);

        assert_eq!(0xBC, cpu.peek8());

        let v = cpu.pop8();

        assert_eq!(0xBC, v);
    }
}
