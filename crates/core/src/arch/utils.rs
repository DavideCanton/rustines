use crate::{MementoReadable, MementoWriteable};

#[derive(Debug, Clone, Copy)]
pub struct Flag(bool);

impl Flag {
    pub fn toggle(&mut self) -> bool {
        self.0 = !self.0;
        self.0
    }

    pub fn set(&mut self) {
        self.0 = true;
    }

    pub fn is_enabled(&self) -> bool {
        self.0
    }

    pub fn ternary<T>(&self, ok: T, or: T) -> T {
        if self.0 { ok } else { or }
    }
}

impl From<bool> for Flag {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<Flag> for bool {
    fn from(value: Flag) -> Self {
        value.0
    }
}

impl MementoWriteable for Flag {
    fn write_memento(&self, writer: &mut crate::MementoWriter) {
        self.0.write_memento(writer);
    }
}

impl MementoReadable for Flag {
    fn read_memento(reader: &mut crate::MementoReader) -> crate::MementoResult<Self>
    where
        Self: Sized,
    {
        let v: bool = reader.read_memento()?;
        Ok(Self::from(v))
    }
}

pub fn generate_ppm(width: usize, height: usize, data: &[u8], skip_fourth: bool) -> Vec<u8> {
    let mut vec = Vec::new();

    vec.extend(format!("P6\n{} {}\n255\n", width, height).as_bytes());

    for (i, v) in data.iter().enumerate() {
        if i % 4 != 3 || !skip_fourth {
            vec.push(*v);
        }
    }

    vec
}
