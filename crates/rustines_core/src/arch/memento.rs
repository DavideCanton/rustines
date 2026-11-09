use std::array::from_fn;

use crate::utils::bit_utils::to_u16;

pub trait MementoReadAndFill {
    fn read_and_fill(&mut self, data: &mut MementoReader);
}

pub trait MementoWriter {
    fn write(&mut self, value: u8);
    fn write_many(&mut self, values: &[u8]);
    fn write_memento_ref<M: MementoWriteable>(&mut self, value: &M)
    where
        Self: Sized,
    {
        value.write_memento(self);
    }
    fn write_memento<M: MementoWriteable>(&mut self, value: M)
    where
        Self: Sized,
    {
        self.write_memento_ref(&value);
    }
}

impl MementoWriter for Vec<u8> {
    fn write(&mut self, value: u8) {
        self.push(value);
    }

    fn write_many(&mut self, values: &[u8]) {
        self.extend_from_slice(values);
    }
}

pub trait MementoReadable {
    fn read_memento(reader: &mut MementoReader) -> Self;
}

pub trait MementoWriteable {
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
        let [b1, b2, b3, b4] = reader.read_many();
        ((b4 as u32) << 24) | ((b3 as u32) << 16) | ((b2 as u32) << 8) | (b1 as u32)
    }
}

impl MementoWriteable for u32 {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        writer.write_many(&self.to_le_bytes());
    }
}

impl MementoReadable for u64 {
    fn read_memento(reader: &mut MementoReader) -> Self {
        let mut cur: u64 = 0;
        let values: [u8; 8] = reader.read_many();

        for (i, val) in values.into_iter().enumerate() {
            cur |= (val as u64) << i;
        }

        cur
    }
}

impl MementoWriteable for u64 {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        writer.write_many(&self.to_le_bytes());
    }
}

impl MementoReadable for usize {
    fn read_memento(reader: &mut MementoReader) -> Self {
        let bytes = usize::MAX.bit_width() >> 3;
        let mut cur: usize = 0;

        for i in 0..bytes {
            let val = reader.next().unwrap() as usize;
            cur |= val << i;
        }

        cur
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
            _ => panic!("Invalid"),
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
                let _ = data.read_memento::<u8>();
                v.read_and_fill(data);
            }
            None => {
                // skip the 0
                let _ = data.read_memento::<u8>();
            }
        }
    }
}

pub struct MementoReader<'a> {
    data: &'a [u8],
    pointer: usize,
}

impl<'a> MementoReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pointer: 0 }
    }

    fn next(&mut self) -> Option<u8> {
        if self.pointer == self.data.len() {
            None
        } else {
            let v = self.data[self.pointer];
            self.pointer += 1;
            Some(v)
        }
    }

    fn read_many<const N: usize>(&mut self) -> [u8; N] {
        from_fn(|_| self.next().unwrap())
    }

    fn read_many_dyn(&mut self, size: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(self.next().unwrap());
        }
        v
    }

    pub fn read_memento<M: MementoReadable>(&mut self) -> M {
        M::read_memento(self)
    }
}
