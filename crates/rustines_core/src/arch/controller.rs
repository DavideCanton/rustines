pub struct NesController {
    pub buttons: [bool; 8],
    pub shift_register: u8,
    pub strobe: bool,
}

impl NesController {
    pub fn new() -> Self {
        Self {
            buttons: [false; 8],
            shift_register: 0,
            strobe: false,
        }
    }

    pub fn write(&mut self, value: u8) {
        self.strobe = (value & 1) == 1;

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
}

impl Default for NesController {
    fn default() -> Self {
        Self::new()
    }
}
