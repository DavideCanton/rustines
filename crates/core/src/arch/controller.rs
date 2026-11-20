use log::trace;

use crate::{
    MementoReadable, MementoWriteable,
    utils::bit_utils::{BitIndex as BI, BitManip},
};
use serde::{Deserialize, Serialize};

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NesKey {
    A = 0,
    B = 1,
    Select = 2,
    Start = 3,
    Up = 4,
    Down = 5,
    Left = 6,
    Right = 7,
}

impl TryFrom<u16> for NesKey {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use NesKey::*;
        match value {
            0 => Ok(A),
            1 => Ok(B),
            2 => Ok(Select),
            3 => Ok(Start),
            4 => Ok(Up),
            5 => Ok(Down),
            6 => Ok(Left),
            7 => Ok(Right),
            v => Err(v.to_string()),
        }
    }
}

impl MementoReadable for NesKey {
    fn read_memento(reader: &mut crate::MementoReader) -> crate::MementoResult<Self>
    where
        Self: Sized,
    {
        let v: u16 = reader.read_memento()?;
        let conv = NesKey::try_from(v).map_err(|e| reader.read_error(e))?;
        Ok(conv)
    }
}

impl MementoWriteable for NesKey {
    fn write_memento(&self, writer: &mut crate::MementoWriter) {
        (*self as u16).write_memento(writer);
    }
}

pub struct NesController {
    number: u8,
    buttons: [bool; 8],
    shift_register: u8,
    strobe: bool,
}

impl NesController {
    pub fn new(number: u8) -> Self {
        Self {
            number,
            buttons: [false; 8],
            shift_register: 0,
            strobe: false,
        }
    }

    pub fn buttons(&self) -> &[bool; 8] {
        &self.buttons
    }

    pub fn reset(&mut self) {
        self.buttons.fill(false);
        self.shift_register = 0;
        self.strobe = false;
    }

    pub fn write(&mut self, value: u8) {
        self.strobe = !value.is_multiple_of(2);

        if self.strobe {
            let mut temp = 0u8;
            for (i, btn) in self.buttons.iter().enumerate() {
                if *btn {
                    temp |= 1 << i;
                }
            }
            self.shift_register = temp;
        }
    }

    pub fn read(&mut self) -> u8 {
        let value = self.peek_state();

        if !self.strobe {
            self.shift_register = (self.shift_register >> 1).set_bit(BI::_7, true);
        }

        value
    }

    pub fn pressed(&mut self, key: NesKey) {
        trace!("Controller {}, pressed {key:?}", self.number);
        self.buttons[key as usize] = true;
    }

    pub fn released(&mut self, key: NesKey) {
        trace!("Controller {}, released {key:?}", self.number);
        self.buttons[key as usize] = false;
    }

    pub fn peek_state(&self) -> u8 {
        (self.shift_register & 1).set_bit(BI::_6, true)
    }
}
