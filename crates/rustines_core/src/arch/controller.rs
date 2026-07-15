use log::trace;

pub struct NesController {
    number: u8,
    buttons: [bool; 8],
    shift_register: u8,
    strobe: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum NesKey {
    A = 0,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
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

    pub fn write(&mut self, value: u8) {
        self.strobe = !value.is_multiple_of(2);

        if self.strobe {
            let mut temp = 0u8;
            for i in 0..8 {
                if self.buttons[i] {
                    temp |= 1 << i;
                }
            }
            self.shift_register = temp;
        }
    }

    pub fn read(&mut self) -> u8 {
        let value = (self.shift_register & 1) | 0x40;

        if !self.strobe {
            self.shift_register >>= 1;
            self.shift_register |= 0x80;
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
}
