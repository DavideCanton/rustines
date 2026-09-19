use std::fs::File;
use std::io::{self, BufWriter, Write};

use crate::arch::bus::Bus;
use crate::arch::mappers::mapper::Mapper;
use crate::arch::ppu::{Ppu, Sprite, get_color_index};
use crate::utils::bit_utils::{BitCount, BitIndex, extract_bits_shift};

pub fn dump_pattern_tables(mapper: &dyn Mapper, scale: usize) -> Vec<u8> {
    assert!(scale > 0, "Pattern table scale must be greater than zero");

    let width = 256 * scale;
    let height = 128 * scale;
    let mut buf = vec![0; width * height * 4];

    for table_index in 0..2 {
        let table_x = table_index * 128 * scale;
        for tile_y in 0..16 {
            for tile_x in 0..16 {
                for pixel_y in 0..8 {
                    let y = (tile_y * 8 + pixel_y) * scale;

                    let tile_offset = (table_index * 0x1000) + (tile_y * 16 * 16) + (tile_x * 16);
                    let addr = (tile_offset + pixel_y) as u16;
                    let byte_low = mapper.fetch_chr_rom(addr);
                    let byte_high = mapper.fetch_chr_rom(addr + 8);

                    for pixel_x in 0..8 {
                        let color: u8 = get_color_index(byte_low, byte_high, pixel_x as u8) * 85;
                        let x = table_x + (tile_x * 8 + pixel_x) * scale;
                        for block_y in 0..scale {
                            for block_x in 0..scale {
                                let pixel_offset = ((y + block_y) * width + x + block_x) * 4;

                                buf[pixel_offset] = color;
                                buf[pixel_offset + 1] = color;
                                buf[pixel_offset + 2] = color;
                                buf[pixel_offset + 3] = 0xFF;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut file = File::create("pattern_table.ppm").expect("Failed to create file");
    let data = generate_ppm(width, height, &buf, true);
    file.write_all(&data).expect("Failed to write file");

    buf
}

pub fn debug_dump_nametable(bus: &Bus) {
    println!("\n=== DUMP NAMETABLE 0 (0x2000) ===");

    print!("    ");
    for col in 0..32 {
        print!("{:02X} ", col);
    }
    println!("\n----{}", "---".repeat(32));

    let ppu = bus.ppu();
    let mapper: &dyn Mapper = bus.mapper_ref();

    for row in 0..30 {
        print!("{:02X} | ", row);

        for col in 0..32 {
            let rel_addr = row * 32 + col;
            let ppu_address = 0x2000 + rel_addr;

            let cleared_addr = (ppu_address - 0x2000) & 0b0000_1111_1111_1111;
            let vram_index = cleared_addr & 0b0000_0111_1111_1111;
            let tile_index = ppu.vram_read(vram_index, mapper);

            if tile_index == 0x00 || tile_index == 0x20 {
                print!(".. ");
            } else {
                print!("{:02X} ", tile_index);
            }
        }
        println!();
    }
    println!("=================================\n");
}

pub fn debug_dump_palette(bus: &Bus) {
    println!("=== DUMP PALETTE ===");
    for (i, color) in bus.ppu().palette_table().iter().enumerate() {
        print!("{:02X} ", color);
        if (i + 1) % 4 == 0 {
            print!("| ");
        }
    }
    println!("\n====================\n");
}

pub fn debug_dump_oam(bus: &Bus) {
    println!("=== DUMP OAM ===");
    // SAFETY: self.oam_data has always a length multiple of 4
    let oam = unsafe { bus.ppu().oam_data().as_chunks_unchecked::<4>() };
    for (i, chunk) in oam.iter().enumerate() {
        let sprite: Sprite = (*chunk).into();
        println!("Sprite {} = {:?}", i, sprite);
    }
    println!("\n===============\n");
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

#[macro_export]
macro_rules! trace_ret {
    ( $expr: expr ) => {{
        let s = stringify!($expr);
        let v = $expr;
        log::trace!("{} = {}", s, v);
        v
    }};
}
