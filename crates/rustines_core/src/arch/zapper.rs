pub struct Zapper {
    position: Option<(usize, usize)>,
    trigger_pressed: bool,
}

impl Zapper {
    pub fn new() -> Self {
        Self {
            position: None,
            trigger_pressed: false,
        }
    }

    pub fn set_input(&mut self, position: Option<(usize, usize)>, trigger_pressed: bool) {
        self.position = position;
        self.trigger_pressed = trigger_pressed;
    }

    pub fn position(&self) -> Option<(usize, usize)> {
        self.position
    }

    pub fn read(&self, light_detected: bool) -> u8 {
        let mut value = 0b0001_1000;

        if self.trigger_pressed {
            value &= !0b0001_0000;
        }
        if light_detected {
            value &= !0b0000_1000;
        }

        value
    }
}

impl Default for Zapper {
    fn default() -> Self {
        Self::new()
    }
}
