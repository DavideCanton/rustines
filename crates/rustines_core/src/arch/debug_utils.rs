use std::fs::File;
use std::io::{self, BufWriter, Write};

use crate::arch::bus::Bus;
use crate::arch::mappers::mapper::Mapper;
use crate::arch::ppu::{Ppu, Sprite};
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
                    let tile_offset = (table_index * 0x1000) + (tile_y * 16 * 16) + (tile_x * 16);

                    let addr_low = (tile_offset + pixel_y) as u16;
                    let addr_high = (tile_offset + pixel_y + 8) as u16;

                    let byte_low = mapper.fetch_chr_rom(addr_low);
                    let byte_high = mapper.fetch_chr_rom(addr_high);

                    let y = (tile_y * 8 + pixel_y) * scale;

                    for pixel_x in 0..8 {
                        let bit_shift: BitIndex = (7 - (pixel_x as u8)).try_into().unwrap();

                        let bit_low = extract_bits_shift(byte_low, bit_shift, BitCount::Bit1);
                        let bit_high = extract_bits_shift(byte_high, bit_shift, BitCount::Bit1);

                        let color_index = (bit_high << 1) | bit_low;
                        let color: u8 = color_index * 85;

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
    let oam = bus.ppu().oam_data();
    for i in 0..64 {
        let sprite = Sprite::from_oam_index(oam, i);
        println!("Sprite {} = {:?}", i, sprite);
    }
    println!("\n===============\n");
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
