use crate::{
    arch::memento::{
        MementoReadAndFill, MementoReader, MementoResult, MementoWriteable, MementoWriter,
    },
    utils::bit_utils::{BitCount, BitIndex, BitManip},
};

pub struct Dmc {
    irq_enable: bool,
    irq_active: bool,
    loop_flag: bool,
    freq: u8,
    direct_load: u8,
    address: u16,
    bytes_remaining: u16,
    bits_remaining: u8,
    sample_length: u16,
    timer: u16,
    enabled: bool,

    sample_buffer: Option<u8>,
    shift_register: u8,
    silence: bool,
    output_level: u8,
    current_address: u16,
}

impl Dmc {
    pub fn new() -> Self {
        Self {
            irq_enable: false,
            irq_active: false,
            loop_flag: false,
            freq: 0,
            direct_load: 0,
            address: 0,
            bytes_remaining: 0,
            bits_remaining: 0,
            sample_length: 0,
            timer: PERIODS[0],
            enabled: false,
            output_level: 0,
            sample_buffer: None,
            shift_register: 0,
            silence: true,
            current_address: 0,
        }
    }

    pub fn tick_timer(&mut self) {
        if !self.enabled {
            return;
        }

        if self.timer > 0 {
            self.timer -= 1;
        } else {
            self.timer = self.get_period();
            self.clock_output_unit();
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if !enabled {
            self.bytes_remaining = 0;
        } else {
            self.restart_sample();
        }
    }

    fn get_period(&self) -> u16 {
        PERIODS[self.freq as usize]
    }

    pub fn update_1(&mut self, value: u8) {
        self.irq_enable = value.get_bit(BitIndex::_7);
        self.loop_flag = value.get_bit(BitIndex::_6);
        self.freq = value.extract_bits_mask_lsb(BitCount::_4);

        if !self.irq_enable {
            self.irq_active = false;
        }
    }

    pub fn update_2(&mut self, value: u8) {
        self.direct_load = value.extract_bits_mask_lsb(BitCount::_7);
        self.output_level = self.direct_load;
    }

    pub fn update_3(&mut self, value: u8) {
        self.address = 0xC000 | ((value as u16) << 6);
    }

    pub fn update_4(&mut self, value: u8) {
        self.sample_length = (value as u16 * 16) + 1;
        if !self.enabled {
            self.bytes_remaining = self.sample_length;
        }
    }

    fn restart_sample(&mut self) {
        self.current_address = self.address;
        self.bytes_remaining = self.sample_length;
    }

    pub fn irq_active(&self) -> bool {
        self.irq_active
    }

    pub fn set_irq_active(&mut self, value: bool) {
        self.irq_active = value;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn output(&self) -> u8 {
        self.output_level
    }

    pub fn needs_dma(&self) -> bool {
        self.enabled && self.sample_buffer.is_none() && self.bytes_remaining > 0
    }

    pub fn current_address(&self) -> u16 {
        self.current_address
    }

    pub fn load_sample(&mut self, value: u8) {
        self.sample_buffer = Some(value);

        self.current_address = if self.current_address == 0xFFFF {
            0x8000
        } else {
            self.current_address + 1
        };

        self.bytes_remaining -= 1;

        if self.bytes_remaining == 0 {
            if self.loop_flag {
                self.restart_sample();
            } else if self.irq_enable {
                self.irq_active = true;
            }
        }
    }

    fn clock_output_unit(&mut self) {
        if self.bits_remaining == 0 {
            self.bits_remaining = 8;

            if let Some(sample) = self.sample_buffer.take() {
                self.shift_register = sample;
                self.silence = false;
            } else {
                self.silence = true;
            }
        }

        if !self.silence {
            if self.shift_register & 1 != 0 {
                if self.output_level <= 125 {
                    self.output_level += 2;
                }
            } else if self.output_level >= 2 {
                self.output_level -= 2;
            }
        }

        self.shift_register >>= 1;
        self.bits_remaining -= 1;
    }
}

impl MementoWriteable for Dmc {
    fn write_memento(&self, writer: &mut dyn MementoWriter) -> MementoResult {
        self.irq_enable.write_memento(writer)?;
        self.irq_active.write_memento(writer)?;
        self.loop_flag.write_memento(writer)?;
        self.freq.write_memento(writer)?;
        self.direct_load.write_memento(writer)?;
        self.address.write_memento(writer)?;
        self.bytes_remaining.write_memento(writer)?;
        self.bits_remaining.write_memento(writer)?;
        self.sample_length.write_memento(writer)?;
        self.timer.write_memento(writer)?;
        self.enabled.write_memento(writer)?;
        self.sample_buffer.write_memento(writer)?;
        self.shift_register.write_memento(writer)?;
        self.silence.write_memento(writer)?;
        self.output_level.write_memento(writer)?;
        self.current_address.write_memento(writer)?;
        Ok(())
    }
}

impl MementoReadAndFill for Dmc {
    fn read_and_fill(&mut self, data: &mut MementoReader) -> MementoResult {
        self.irq_enable = data.read_memento()?;
        self.irq_active = data.read_memento()?;
        self.loop_flag = data.read_memento()?;
        self.freq = data.read_memento()?;
        self.direct_load = data.read_memento()?;
        self.address = data.read_memento()?;
        self.bytes_remaining = data.read_memento()?;
        self.bits_remaining = data.read_memento()?;
        self.sample_length = data.read_memento()?;
        self.timer = data.read_memento()?;
        self.enabled = data.read_memento()?;
        self.sample_buffer = data.read_memento()?;
        self.shift_register = data.read_memento()?;
        self.silence = data.read_memento()?;
        self.output_level = data.read_memento()?;
        self.current_address = data.read_memento()?;
        Ok(())
    }
}

const PERIODS: [u16; 16] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
];
