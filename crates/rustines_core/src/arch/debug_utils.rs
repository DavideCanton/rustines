use std::fs::File;
use std::io::{self, BufWriter, Write};

use crate::MirroringType;
use crate::arch::bus::Bus;
use crate::arch::cpu::Cpu;
use crate::arch::mappers::mapper::Mapper;
use crate::arch::ppu::{OamSprite, Ppu, get_color_index};
use crate::arch::utils::generate_ppm;
use crate::utils::bit_utils::{BitCount, BitIndex, extract_bits_shift, hex};

pub fn dump_pattern_tables(mapper: &dyn Mapper, scale: usize, save_file: bool) -> Vec<u8> {
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
                    let byte_low = mapper.fetch_ppu(addr);
                    let byte_high = mapper.fetch_ppu(addr + 8);

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

    if save_file {
        let mut file = File::create("pattern_table.ppm").expect("Failed to create file");
        let data = generate_ppm(width, height, &buf, true);
        file.write_all(&data).expect("Failed to write file");
    }

    buf
}

pub fn debug_dump_nametables(bus: &Bus) {
    let mapper: &dyn Mapper = bus.mapper_ref();
    let ppu = bus.ppu();

    let nt1 = 0x2000;
    let nt2 = match mapper.mirroring_mode() {
        MirroringType::Horizontal => 0x2800,
        MirroringType::Vertical => 0x2400,
    };

    for (nt, base) in [nt1, nt2].iter().enumerate() {
        println!("\n=== DUMP NAMETABLE {nt} (0x{base:04X}) ===");

        print!("     ");
        for col in 0..32 {
            print!("{:02X} ", col);
        }
        println!("\n{}", "-".repeat(100));

        for row in 0..30 {
            print!("{:02X} | ", row);

            for col in 0..32 {
                let rel_addr = row * 32 + col;
                let ppu_address = base + rel_addr;
                let tile_index = ppu.vram_read(ppu_address, mapper);
                print!("{:02X} ", tile_index);
            }
            println!();
        }
        println!("=================================\n");
    }
}

pub fn debug_dump_attribute_tables(bus: &Bus) {
    let mapper: &dyn Mapper = bus.mapper_ref();
    let ppu = bus.ppu();

    let at1 = 0x23C0;
    let at2 = match mapper.mirroring_mode() {
        MirroringType::Horizontal => 0x2BC0,
        MirroringType::Vertical => 0x27C0,
    };

    for (at, base) in [at1, at2].into_iter().enumerate() {
        println!("\n=== DUMP ATTRIBUTE TABLE {at} (0x{base:04X}) ===");

        print!("     ");
        for col in 0..32 {
            print!("{:02X} ", col);
        }
        println!("\n{}", "-".repeat(100));

        for row in 0..30 {
            print!("{:02X} | ", row);

            for col in 0..32 {
                let attribute_address = base + ((row << 1) + (col >> 2)) as u16;
                let attribute = ppu.vram_read(attribute_address, mapper);
                let shift = ((row & 0x02) << 1) | (col & 0x02);
                let palette_index = (attribute >> shift) & 0x03;
                print!("{:>2} ", palette_index);
            }
            println!();
        }
        println!("=================================\n");
    }
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

pub fn debug_dump_oam(ppu: &Ppu) {
    println!("Scanline: {}", ppu.scanline);
    println!("=== DUMP OAM ===");
    print_oam(ppu.oam_data());
    println!("\n=== SECONDARY OAM (cycle {}) ===", ppu.cycle);
    print_oam(ppu.secondary_oam_data());
    println!("\n===============\n");
    println!("=== SHIFTERS ===");
    for s in ppu.sprite_shifters {
        println!("{}", s);
    }
    println!("\n===============\n");
}

fn print_oam<const N: usize>(oam: &[u8; N]) {
    for (i, sprite_bytes) in oam.as_chunks::<4>().0.iter().enumerate() {
        let sprite: &OamSprite = bytemuck::cast_ref(sprite_bytes);
        println!(
            "Sprite {} = {:?} {}",
            i,
            sprite,
            format_sprite_bytes(sprite_bytes)
        );
    }
}

fn format_sprite_bytes(bytes: &[u8]) -> String {
    bytes.iter().copied().map(hex).collect::<Vec<_>>().join("-")
}

pub fn debug_dump_state(bus: &Bus, cpu: &Cpu) {
    // CPU
    println!("--- CPU ---");
    println!("A = {:#04X}", cpu.registers.a_reg);
    println!("X = {:#04X}", cpu.registers.x_reg);
    println!("Y = {:#04X}", cpu.registers.y_reg);
    println!("P = {}", cpu.registers.p_to_str());
    println!("PC = {:#04X}", cpu.registers.pc);
    println!("SP = {:#04X}", cpu.registers.sp);
    println!(
        "INT = {}",
        vec![
            if cpu.pending_irq_execution { "IRQ" } else { "" },
            if cpu.pending_nmi_execution { "NMI" } else { "" },
            if cpu.pending_rst_execution { "RST" } else { "" }
        ]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
    );
    // PPU
    println!("--- PPU ---");
    let ppu = bus.ppu();
    println!("PPUCTRL = {:?}", ppu.ctrl);
    println!("PPUMASK = {:?}", ppu.mask);
    println!("PPUSTATUS = {:?}", ppu.status);
    println!("scanline={} cycle={}", ppu.scanline, ppu.cycle);
    println!("V = {:?}", ppu.v_reg);
    println!("X = {}", ppu.x_reg);
    println!("T = {:?}", ppu.t_reg);
    println!("BG = {:?}", ppu.bg_data);
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
