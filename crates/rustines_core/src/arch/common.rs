pub(crate) fn replace<const N: usize>(array: &mut [u8; N], i: usize, v: u8) -> u8 {
    let old = array[i];
    array[i] = v;
    old
}

#[cfg(test)]
mod tests {
    use super::replace;

    #[test]
    fn test_replace() {
        let mut v = [1, 2, 3];
        let o = replace(&mut v, 0, 4);
        assert_eq!(o, 1);
        assert_eq!(v, [4, 2, 3]);
    }
}
