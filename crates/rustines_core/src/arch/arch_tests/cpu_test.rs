#[cfg(test)]
mod tests {
    use crate::arch::arch_tests::test_common::tests::setup_tests;

    #[test]
    fn test_push_pop_peek() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.push8(&mut bus, 0xAB);

        assert_eq!(0xAB, cpu.peek8(&mut bus));

        let v = cpu.pop8(&mut bus);

        assert_eq!(0xAB, v);

        cpu.push8(&mut bus, 0xBC);
        cpu.push8(&mut bus, 0xCD);

        assert_eq!(0xCD, cpu.peek8(&mut bus));

        let v = cpu.pop8(&mut bus);

        assert_eq!(0xCD, v);

        assert_eq!(0xBC, cpu.peek8(&mut bus));

        let v = cpu.pop8(&mut bus);

        assert_eq!(0xBC, v);
    }
}
