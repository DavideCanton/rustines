use crate::{
    arch::{mappers::mapper::Mapper, rom_structs::MirroringType},
    renderer::Renderer,
};

#[derive(Debug)]
struct Sprite {
    x: u8,
    y: u8,
    tile: u8,
    attr: u8,
}

pub struct Ppu {
    nametables: [u8; 2048],
    palette_table: [u8; 32],
    oam_data: [u8; 256],

    ctrl: u8,
    mask: u8,
    status: u8,

    vram_address: u16,
    temp_address: u16,
    oam_addr: u8,
    fine_x: u8,
    address_latch: u8,
    data_buffer: u8,

    scanline: i16,
    cycle: i16,

    nmi_interrupt: bool,
    frame_ready: bool,

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
            oam_addr: 0,
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

    pub fn palette_table(&self) -> &[u8; 32] {
        &self.palette_table
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

    pub fn cpu_write(&mut self, reg_index: u8, value: u8, mapper: &dyn Mapper) {
        match reg_index {
            0 => self.ctrl = value,
            1 => self.mask = value,
            2 => {}
            3 => {
                self.oam_addr = value;
            }
            4 => {
                self.oam_data[self.oam_addr as usize] = value;
                self.oam_addr = self.oam_addr.wrapping_add(1);
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
                self.vram_write(self.vram_address, value, mapper);

                let increment = if (self.ctrl & 0x04) != 0 { 32 } else { 1 };
                self.vram_address = self.vram_address.wrapping_add(increment);
            }
            _ => unreachable!(),
        }
    }

    pub fn cpu_read(&mut self, reg_index: u16, mapper: &dyn Mapper) -> u8 {
        match reg_index {
            2 => {
                let res = self.status;
                self.status &= 0x7F;
                self.address_latch = 0;
                res
            }
            7 => {
                let mut data = self.data_buffer;
                self.data_buffer = self.vram_read(self.vram_address, mapper);

                if self.vram_address >= 0x3F00 {
                    data = self.data_buffer;
                }
                self.vram_address += if (self.ctrl & 0x04) != 0 { 32 } else { 1 };
                data
            }
            _ => 0,
        }
    }

    pub fn vram_read(&self, mut addr: u16, mapper: &dyn Mapper) -> u8 {
        addr &= 0x3FFF;

        match addr {
            0x0000..=0x1FFF => 0,
            0x2000..=0x3EFF => {
                let idx = self.mirror_nametable_addr(addr, mapper.mirroring_mode());
                self.nametables[idx]
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

    fn vram_write(&mut self, mut addr: u16, value: u8, mapper: &dyn Mapper) {
        addr &= 0x3FFF;
        match addr {
            0x0000..=0x1FFF => {
                // TODO
                // if mapper.has_chr_ram() {
                //     chr_memory[address as usize] = data;
                // } else {
                // }
            }
            0x2000..=0x3EFF => {
                let idx = self.mirror_nametable_addr(addr, mapper.mirroring_mode());
                self.nametables[idx] = value;
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

    fn render_scanline(&mut self, mapper: &dyn Mapper) {
        // TODO
        // if chr_rom.is_empty() {
        //     return;
        // }

        let y = self.scanline as usize;
        let tile_y = (y / 8) as u16;
        let pixel_y = (y % 8) as u16;

        let base_nametable_addr = 0x2000 + ((self.ctrl & 0x03) as u16 * 0x0400);

        let visible_sprites = self.get_sprites_on_scanline();

        for x in 0..=255 {
            let tile_x = (x / 8) as u16;
            let pixel_x = (x % 8) as u16;

            let nametable_index = tile_y * 32 + tile_x;
            let tile_id = self.vram_read(base_nametable_addr + nametable_index, mapper) as u16;

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

            let palette_color_id = self.vram_read(0x3F00 + color_index as u16, mapper);

            let background_rgb = NES_PALETTE[(palette_color_id & 0x3F) as usize];

            // --- Logica Sprite ---
            let mut pixel_color = background_rgb;

            for sprite in &visible_sprites {
                if x >= sprite.x && x < sprite.x + 8 {
                    let mut pixel_x = (x - sprite.x) as u16;
                    let mut pixel_y = (y as u8 - sprite.y) as u16;

                    if (sprite.attr & 0x40) != 0 {
                        pixel_x = 7 - pixel_x;
                    }
                    if (sprite.attr & 0x80) != 0 {
                        pixel_y = 7 - pixel_y;
                    }

                    let pattern_table_base = if (self.ctrl & 0x08) != 0 {
                        0x1000
                    } else {
                        0x0000
                    };
                    let tile_addr = pattern_table_base + (sprite.tile as u16 * 16) + pixel_y;

                    let byte1 = mapper.fetch_chr_rom(tile_addr);
                    let byte2 = mapper.fetch_chr_rom(tile_addr + 8);

                    let bit1 = (byte1 >> (7 - pixel_x)) & 0x01;
                    let bit2 = (byte2 >> (7 - pixel_x)) & 0x01;
                    let sprite_pixel_bits = (bit2 << 1) | bit1;

                    if sprite_pixel_bits != 0 {
                        let palette_num = (sprite.attr & 0x03) as u16;
                        let palette_addr = 0x3F10 + (palette_num * 4) + sprite_pixel_bits as u16;
                        let color_id = self.vram_read(palette_addr, mapper);
                        let is_behind = (sprite.attr & 0x20) != 0;
                        if !is_behind || color_index == 0 {
                            pixel_color = NES_PALETTE[(color_id & 0x3F) as usize];
                            break;
                        }
                    }
                }
            }

            self.renderer.render_pixel(x as usize, y, pixel_color)
        }
    }

    fn mirror_nametable_addr(&self, addr: u16, mode: MirroringType) -> usize {
        let vram_index = ((addr & 0x2FFF) - 0x2000) as usize;

        match mode {
            MirroringType::Horizontal => {
                if vram_index < 2048 {
                    vram_index % 1024
                } else {
                    (vram_index % 1024) + 1024
                }
            }
            MirroringType::Vertical => vram_index % 2048,
            // _ => vram_index % 2048,
        }
    }

    fn get_sprites_on_scanline(&self) -> Vec<Sprite> {
        let scanline = self.scanline as u16;

        let mut visible = Vec::new();
        let sprite_height = if (self.ctrl & 0x20) != 0 { 16 } else { 8 };

        for i in 0..64 {
            let y = self.oam_data[i * 4] as u16;
            if y <= scanline && scanline < (y + sprite_height) {
                visible.push(Sprite {
                    y: self.oam_data[i * 4],
                    tile: self.oam_data[i * 4 + 1],
                    attr: self.oam_data[i * 4 + 2],
                    x: self.oam_data[i * 4 + 3],
                });
            }
            if visible.len() >= 8 {
                break;
            }
        }
        visible
    }

    pub(crate) fn dma_copy(&mut self, buf: &[u8]) {
        self.oam_data.copy_from_slice(buf);
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
