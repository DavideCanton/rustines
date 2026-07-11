use std::fs::File;
use std::io::{self, BufWriter, Write};

use crate::arch::bus::Bus;
use crate::arch::mappers::mapper::Mapper;
use crate::arch::ppu::Ppu;

pub fn dump_pattern_tables(mapper: &dyn Mapper) -> io::Result<()> {
    for table_index in 0..2 {
        let file_name = format!("pattern_table_{}.ppm", table_index);
        let file = File::create(&file_name)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3")?;
        writeln!(writer, "128 128")?;
        writeln!(writer, "255")?;

        for tile_y in 0..16 {
            for pixel_y in 0..8 {
                for tile_x in 0..16 {
                    let tile_offset = (table_index * 0x1000) + (tile_y * 16 * 16) + (tile_x * 16);

                    let addr_low = (tile_offset + pixel_y) as u16;
                    let addr_high = (tile_offset + pixel_y + 8) as u16;

                    let byte_low = mapper.fetch_chr_rom(addr_low);
                    let byte_high = mapper.fetch_chr_rom(addr_high);

                    for pixel_x in 0..8 {
                        let bit_shift = 7 - pixel_x;

                        let bit_low = (byte_low >> bit_shift) & 0x01;
                        let bit_high = (byte_high >> bit_shift) & 0x01;

                        let color_index = (bit_high << 1) | bit_low;

                        let grayscale_val = match color_index {
                            0 => 0,
                            1 => 85,
                            2 => 170,
                            3 => 255,
                            _ => unreachable!(),
                        };

                        write!(
                            writer,
                            "{} {} {} ",
                            grayscale_val, grayscale_val, grayscale_val
                        )?;
                    }
                }
                writeln!(writer)?;
            }
        }
        println!("Dump completato: {}", file_name);
    }
    Ok(())
}

pub fn debug_dump_nametable(bus: &mut Bus) {
    println!("\n=== DUMP NAMETABLE 0 (0x2000) ===");

    print!("    ");
    for col in 0..32 {
        print!("{:02X} ", col);
    }
    println!("\n----{}", "---".repeat(32));

    for row in 0..30 {
        print!("{:02X} | ", row);

        for col in 0..32 {
            let rel_addr = row * 32 + col;
            let ppu_address = 0x2000 + rel_addr;

            let tile_index = debug_read_nametable_byte(bus.ppu(), bus.mapper(), ppu_address);

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

fn debug_read_nametable_byte(ppu: &Ppu, mapper: &dyn Mapper, address: u16) -> u8 {
    let cleared_addr = (address - 0x2000) & 0x0FFF;
    let vram_index = cleared_addr & 0x07FF;
    ppu.vram_read(vram_index, mapper)
}

pub fn debug_dump_palette(bus: &mut Bus) {
    println!("=== DUMP PALETTE ===");
    for (i, color) in bus.ppu_mut().palette_table.iter().enumerate() {
        print!("{:02X} ", color);
        if (i + 1) % 4 == 0 {
            print!("| ");
        }
    }
    println!("\n====================\n");
}
