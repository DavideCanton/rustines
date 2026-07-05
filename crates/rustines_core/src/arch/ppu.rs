use log::info;

use crate::{arch::mappers::mapper::Mapper, renderer::Renderer};

pub struct Ppu {
    pub nametables: [u8; 2048],
    pub palette_table: [u8; 32],
    pub oam_data: [u8; 256],

    pub ctrl: u8,
    pub mask: u8,
    pub status: u8,

    pub vram_address: u16,
    pub temp_address: u16,
    pub fine_x: u8,
    pub address_latch: u8,
    pub data_buffer: u8,

    pub scanline: i16,
    pub cycle: i16,

    pub nmi_interrupt: bool,
    pub frame_ready: bool,

    renderer: Box<dyn Renderer>,
}

impl Ppu {
    pub fn new(renderer: Box<dyn Renderer>) -> Self {
        Self {
            nametables: [0; 2048],
            palette_table: [0; 32],
            oam_data: [0; 256],

            ctrl: 0,
            mask: 0,
            status: 0,

            vram_address: 0,
            temp_address: 0,
            fine_x: 0,
            address_latch: 0,
            data_buffer: 0,

            scanline: -1,
            cycle: 0,

            nmi_interrupt: false,
            frame_ready: false,
            renderer,
        }
    }

    pub fn nmi_requested(&self) -> bool {
        self.nmi_interrupt
    }

    pub fn clear_nmi(&mut self) {
        self.nmi_interrupt = false;
    }

    pub fn frame_ready(&self) -> bool {
        self.frame_ready
    }

    pub fn clear_frame_ready(&mut self) {
        self.frame_ready = false;
    }

    pub fn renderer(&mut self) -> &mut dyn Renderer {
        self.renderer.as_mut()
    }

    pub fn tick(&mut self, mapper: &mut dyn Mapper) {
        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline >= 261 {
                self.scanline = -1;
                self.frame_ready = true;
                self.renderer.draw();
            }
        }

        if self.scanline >= 0 && self.scanline <= 239 && self.cycle == 256 {
            self.render_scanline(mapper);
        }

        if self.scanline == 241 && self.cycle == 1 {
            self.status |= 0x80;
            if (self.ctrl & 0x80) != 0 {
                self.nmi_interrupt = true;
            }
        }

        if self.scanline == -1 && self.cycle == 1 {
            self.status &= 0x7F;
            self.nmi_interrupt = false;
        }
    }

    pub fn cpu_write(&mut self, reg_index: u16, value: u8) {
        match reg_index {
            0 => self.ctrl = value,
            1 => self.mask = value,
            2 => {}
            3 => { /* Scrittura indirizzo OAM (0x2003) */ }
            4 => {
                self.oam_data[42] = value;
            }
            5 => {
                if self.address_latch == 0 {
                    self.fine_x = value & 0x07;
                    self.temp_address = (self.temp_address & 0xFFE0) | ((value >> 3) as u16);
                    self.address_latch = 1;
                } else {
                    self.temp_address = (self.temp_address & 0x8C1F)
                        | (((value & 0x07) as u16) << 12)
                        | (((value & 0xF8) as u16) << 2);
                    self.address_latch = 0;
                }
            }
            6 => {
                if self.address_latch == 0 {
                    self.temp_address =
                        (self.temp_address & 0x00FF) | (((value & 0x3F) as u16) << 8);
                    self.address_latch = 1;
                } else {
                    self.temp_address = (self.temp_address & 0xFF00) | (value as u16);
                    self.vram_address = self.temp_address;
                    self.address_latch = 0;
                }
            }
            7 => {
                // info!("VRAM write: {:#X}, {:#X}", self.vram_address, value);
                self.vram_write(self.vram_address, value);

                let increment = if (self.ctrl & 0x04) != 0 { 32 } else { 1 };
                self.vram_address = self.vram_address.wrapping_add(increment);
            }
            _ => unreachable!(),
        }
    }

    pub fn cpu_read(&mut self, reg_index: u16) -> u8 {
        match reg_index {
            2 => {
                info!("CPU reads PPUSTATUS. Status: {:#X}", self.status);
                let res = self.status;
                self.status &= 0x7F;
                self.address_latch = 0;
                res
            }
            7 => {
                let mut data = self.data_buffer;
                self.data_buffer = self.vram_read(self.vram_address);

                if self.vram_address >= 0x3F00 {
                    data = self.data_buffer;
                }
                self.vram_address += if (self.ctrl & 0x04) != 0 { 32 } else { 1 };
                data
            }
            _ => 0,
        }
    }

    pub fn vram_read(&self, mut addr: u16) -> u8 {
        addr &= 0x3FFF;

        match addr {
            0x0000..=0x1FFF => 0,
            0x2000..=0x3EFF => {
                let vram_index = (addr & 0x0FFF) as usize;
                self.nametables[vram_index % 2048]
            }
            0x3F00..=0x3FFF => {
                let mut palette_addr = (addr & 0x001F) as usize;
                if palette_addr == 0x0010
                    || palette_addr == 0x0014
                    || palette_addr == 0x0018
                    || palette_addr == 0x001C
                {
                    palette_addr -= 0x0010;
                }
                self.palette_table[palette_addr]
            }
            _ => 0,
        }
    }

    fn vram_write(&mut self, mut addr: u16, value: u8) {
        addr &= 0x3FFF;
        match addr {
            0x0000..=0x1FFF => {}
            0x2000..=0x3EFF => {
                let vram_index = (addr & 0x0FFF) as usize;
                self.nametables[vram_index % 2048] = value;
            }
            0x3F00..=0x3FFF => {
                let mut palette_addr = (addr & 0x001F) as usize;
                if palette_addr == 0x0010
                    || palette_addr == 0x0014
                    || palette_addr == 0x0018
                    || palette_addr == 0x001C
                {
                    palette_addr -= 0x0010;
                }
                self.palette_table[palette_addr] = value;
            }
            _ => {}
        }
    }

    fn render_scanline(&mut self, mapper: &mut dyn Mapper) {
        // TODO
        // if chr_rom.is_empty() {
        //     return;
        // }

        let y = self.scanline as usize;
        let tile_y = (y / 8) as u16;
        let pixel_y = (y % 8) as u16;

        let base_nametable_addr = 0x2000 + ((self.ctrl & 0x03) as u16 * 0x0400);

        for x in 0..256 {
            let tile_x = (x / 8) as u16;
            let pixel_x = (x % 8) as u16;

            let nametable_index = tile_y * 32 + tile_x;
            let tile_id = self.vram_read(base_nametable_addr + nametable_index) as u16;

            let pattern_table_base = if (self.ctrl & 0x10) != 0 {
                0x1000
            } else {
                0x0000
            };
            let tile_addr = pattern_table_base + (tile_id * 16) + pixel_y;

            let byte1 = mapper.fetch_chr_rom(tile_addr);
            let byte2 = mapper.fetch_chr_rom(tile_addr + 8);

            let bit1 = (byte1 >> (7 - pixel_x)) & 0x01;
            let bit2 = (byte2 >> (7 - pixel_x)) & 0x01;
            let color_index = (bit2 << 1) | bit1;

            let palette_color_id = self.vram_read(0x3F00 + color_index as u16);

            let rgb_color = NES_PALETTE[(palette_color_id & 0x3F) as usize];

            self.renderer.render_pixel(x, y, rgb_color)
        }
    }
}

// 64 colori RGB precalcolati per la PPU del NES
const NES_PALETTE: [u32; 64] = [
    0x545454FF, 0x001E74FF, 0x081090FF, 0x300088FF, 0x440064FF, 0x5C0030FF, 0x540400FF, 0x3C1800FF,
    0x202A00FF, 0x083A00FF, 0x004000FF, 0x003C24FF, 0x00325CFF, 0x000000FF, 0x000000FF, 0x000000FF,
    0x989698FF, 0x084CC4FF, 0x303CE4FF, 0x5C1EDFFF, 0x8814B4FF, 0xA01478FF, 0x9C2028FF, 0x843C00FF,
    0x605A00FF, 0x347200FF, 0x187C00FF, 0x047858FF, 0x0068ACFF, 0x000000FF, 0x000000FF, 0x000000FF,
    0xECEEECFF, 0x4C9AF4FF, 0x788CF4FF, 0xB06CF4FF, 0xE45CE4FF, 0xF45CB4FF, 0xF46D64FF, 0xE48C24FF,
    0xC4AA00FF, 0x90C200FF, 0x68D224FF, 0x4CD278FF, 0x4CC2D4FF, 0x3C3C3CFF, 0x000000FF, 0x000000FF,
    0xECEEECFF, 0xA8CCF4FF, 0xBCC4F4FF, 0xD4B4F4FF, 0xECB0ECFF, 0xF4B0D4FF, 0xF4B8B4FF, 0xECC490FF,
    0xE4D080FF, 0xCCDC80FF, 0xBCE290FF, 0xACE2B4FF, 0xACDAECFF, 0xA8A8A8FF, 0x000000FF, 0x000000FF,
];
