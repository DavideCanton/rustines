//! Memento serialization and deserialization module.
//!
//! This module provides a framework for reading and writing primitive and complex types
//! in a sequential binary format using the `MementoReadable`, `MementoWriteable`,
//! and `MementoReadAndFill` traits.

use std::array::from_fn;

use crate::utils::bit_utils::to_u16;

/// A trait for types that can be populated in-place from a [`MementoReader`].
pub trait MementoReadAndFill {
    /// Reads data from the `MementoReader` and fills the current instance.
    fn read_and_fill(&mut self, data: &mut MementoReader);
}

/// A trait for types that can write bytes to a destination.
pub trait MementoWriter {
    /// Writes a single byte to the writer.
    fn write(&mut self, value: u8);
    /// Writes a slice of bytes to the writer.
    fn write_many(&mut self, values: &[u8]);
}

impl MementoWriter for Vec<u8> {
    fn write(&mut self, value: u8) {
        self.push(value);
    }

    fn write_many(&mut self, values: &[u8]) {
        self.extend_from_slice(values);
    }
}

/// A trait for types that can be deserialized from a [`MementoReader`].
pub trait MementoReadable {
    /// Creates a new instance of the type by reading from the `MementoReader`.
    fn read_memento(reader: &mut MementoReader) -> Self;
}

/// A trait for types that can be serialized into a [`MementoWriter`].
pub trait MementoWriteable {
    /// Writes the binary representation of the type into the `MementoWriter`.
    fn write_memento(&self, writer: &mut dyn MementoWriter);
}

impl MementoReadable for u8 {
    fn read_memento(reader: &mut MementoReader) -> Self {
        reader.next().unwrap()
    }
}

impl MementoWriteable for u8 {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        writer.write(*self);
    }
}

impl MementoReadable for u16 {
    fn read_memento(reader: &mut MementoReader) -> Self {
        let [b1, b2] = reader.read_many();
        to_u16(b1, b2)
    }
}

impl MementoWriteable for u16 {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        writer.write_many(&self.to_le_bytes());
    }
}

impl MementoReadable for u32 {
    fn read_memento(reader: &mut MementoReader) -> Self {
        u32::from_le_bytes(reader.read_many())
    }
}

impl MementoWriteable for u32 {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        writer.write_many(&self.to_le_bytes());
    }
}

impl MementoReadable for u64 {
    fn read_memento(reader: &mut MementoReader) -> Self {
        u64::from_le_bytes(reader.read_many())
    }
}

impl MementoWriteable for u64 {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        writer.write_many(&self.to_le_bytes());
    }
}

impl MementoReadable for usize {
    fn read_memento(reader: &mut MementoReader) -> Self {
        usize::from_le_bytes(reader.read_many())
    }
}

impl MementoWriteable for usize {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        writer.write_many(&self.to_le_bytes());
    }
}

impl MementoReadable for bool {
    fn read_memento(reader: &mut MementoReader) -> Self {
        !matches!(reader.next().unwrap(), 0)
    }
}

impl MementoWriteable for bool {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        writer.write(*self as u8);
    }
}

impl<const N: usize, M: MementoReadable> MementoReadable for [M; N] {
    fn read_memento(reader: &mut MementoReader) -> Self {
        from_fn(|_| M::read_memento(reader))
    }
}

impl<M: MementoWriteable> MementoWriteable for &[M] {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        for v in *self {
            v.write_memento(writer);
        }
    }
}

impl<const N: usize, M: MementoWriteable> MementoWriteable for [M; N] {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        for v in self {
            v.write_memento(writer);
        }
    }
}

impl MementoReadable for Vec<u8> {
    fn read_memento(reader: &mut MementoReader) -> Self {
        let len: usize = reader.read_memento();
        reader.read_many_dyn(len)
    }
}

impl MementoWriteable for Vec<u8> {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        self.len().write_memento(writer);
        writer.write_many(self);
    }
}

impl<T: MementoWriteable> MementoWriteable for Option<T> {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        match self {
            Some(v) => {
                1u8.write_memento(writer);
                v.write_memento(writer);
            }
            None => 0u8.write_memento(writer),
        }
    }
}

impl<T: MementoReadable> MementoReadable for Option<T> {
    fn read_memento(reader: &mut MementoReader) -> Self {
        match reader.next().unwrap() {
            1 => Some(T::read_memento(reader)),
            0 => None,
            _ => panic!("Invalid Option tag"),
        }
    }
}

impl<T: MementoWriteable, U: MementoWriteable> MementoWriteable for (T, U) {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        self.0.write_memento(writer);
        self.1.write_memento(writer);
    }
}

impl<T: MementoReadable, U: MementoReadable> MementoReadable for (T, U) {
    fn read_memento(reader: &mut MementoReader) -> Self {
        (T::read_memento(reader), U::read_memento(reader))
    }
}

impl<M: MementoReadAndFill> MementoReadAndFill for Option<M> {
    fn read_and_fill(&mut self, data: &mut MementoReader) {
        match self {
            Some(v) => {
                // skip the 1
                let _tag = data.read_memento::<u8>();
                v.read_and_fill(data);
            }
            None => {
                // skip the 0
                let _tag = data.read_memento::<u8>();
            }
        }
    }
}

/// A reader that provides an interface for reading bytes and deserializing types from a byte slice.
pub struct MementoReader<'a> {
    data: &'a [u8],
    pointer: usize,
}

impl<'a> MementoReader<'a> {
    /// Creates a new [`MementoReader`] from a byte slice.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pointer: 0 }
    }

    /// Reads the next byte from the reader. Returns `None` if the end of the data is reached.
    fn next(&mut self) -> Option<u8> {
        if self.pointer == self.data.len() {
            None
        } else {
            let v = self.data[self.pointer];
            self.pointer += 1;
            Some(v)
        }
    }

    /// Reads exactly `N` bytes from the reader into an array.
    ///
    /// # Panics
    /// Panics if the reader runs out of bytes before `N` bytes are read.
    fn read_many<const N: usize>(&mut self) -> [u8; N] {
        from_fn(|_| self.next().unwrap())
    }

    /// Reads a dynamic number of bytes from the reader into a [`Vec<u8>`].
    ///
    /// # Panics
    /// Panics if the reader runs out of bytes before `size` bytes are read.
    fn read_many_dyn(&mut self, size: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(self.next().unwrap());
        }
        v
    }

    /// Deserializes a type `M` from the reader using the [`MementoReadable`] trait.
    pub fn read_memento<M: MementoReadable>(&mut self) -> M {
        M::read_memento(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        let val: u8 = 42;
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: u8 = reader.read_memento();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_u16() {
        let val: u16 = 12345;
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: u16 = reader.read_memento();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_u32() {
        let val: u32 = 0xDEADBEEF;
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: u32 = reader.read_memento();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_u64() {
        let val: u64 = 0x123456789ABCDEF0;
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: u64 = reader.read_memento();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_usize() {
        let val: usize = 12345678;
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: usize = reader.read_memento();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_bool() {
        let val: bool = true;
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: bool = reader.read_memento();
        assert_eq!(val, decoded);

        let val: bool = false;
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: bool = reader.read_memento();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_array() {
        let val: [u8; 3] = [1, 2, 3];
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: [u8; 3] = reader.read_memento();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_vec_u8() {
        let val: Vec<u8> = vec![10, 20, 30, 40];
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: Vec<u8> = reader.read_memento();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_option() {
        // Test Some
        let val: Option<u32> = Some(123);
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: Option<u32> = reader.read_memento();
        assert_eq!(val, decoded);

        // Test None
        let val: Option<u32> = None;
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: Option<u32> = reader.read_memento();
        assert_eq!(val, decoded);
    }

    #[test]
    fn test_tuple() {
        let val: (u8, u16) = (1, 2);
        let mut writer = Vec::new();
        val.write_memento(&mut writer);
        let mut reader = MementoReader::new(&writer);
        let decoded: (u8, u16) = reader.read_memento();
        assert_eq!(val, decoded);
    }

    // Helper struct to test MementoReadAndFill
    #[derive(Debug, PartialEq)]
    struct TestFill(u32);

    impl MementoReadAndFill for TestFill {
        fn read_and_fill(&mut self, data: &mut MementoReader) {
            self.0 = data.read_memento();
        }
    }
    impl MementoWriteable for TestFill {
        fn write_memento(&self, writer: &mut dyn MementoWriter) {
            self.0.write_memento(writer);
        }
    }
    impl MementoReadable for TestFill {
        fn read_memento(reader: &mut MementoReader) -> Self {
            Self(reader.read_memento())
        }
    }

    #[test]
    fn test_read_and_fill_option() {
        let mut val: Option<TestFill> = Some(TestFill(42));
        let mut writer = Vec::new();

        // Write using write_memento (which uses the tag 1 for Some)
        val.write_memento(&mut writer);

        let mut reader = MementoReader::new(&writer);

        // Now use read_and_fill to populate the existing instance
        val.read_and_fill(&mut reader);

        assert_eq!(val, Some(TestFill(42)));
    }
}
