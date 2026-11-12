use std::path::Path;

use anyhow::bail;
use arrayref::array_ref;
use log::{debug, error, info};

use crate::{
    arch::{
        mappers::{mapper::MapperBox, mapper_factory::instantiate_mapper},
        rom_structs::{HEADER, INesHeader, NesRom, TRAINER_SIZE},
    },
    utils::named::Named,
};

const HEADER_SIZE: usize = 16;

/// Size of the data following the header: optional trainer, PRG ROM and CHR ROM.
fn expected_rom_size(header: &INesHeader) -> usize {
    let trainer = if header.has_trainer() {
        TRAINER_SIZE
    } else {
        0
    };
    trainer + header.prg_rom_size() + header.chr_rom_size()
}

pub trait Loader: Named {
    fn load_rom(&self, content: &[u8]) -> std::io::Result<Vec<u8>>;

    fn load_header(&self, buf: &[u8; 16]) -> anyhow::Result<INesHeader> {
        let header: INesHeader = buf.into();

        if &header.header != HEADER {
            error!("Found unexpected {:?} header", header.header);
            bail!("Invalid header!".to_string());
        }

        if header.prg_rom_banks() == 0 {
            error!("Found header without PRG ROM banks");
            bail!("Invalid header: the ROM has no PRG ROM banks");
        }

        info!("Mapper: {}", header.mapping_number());
        debug!("Found header: {:?}", header);
        debug!("Mirroring: {:?}", header.mirroring_type());

        Ok(header)
    }

    fn load_rom_struct(
        &self,
        file_path: Option<&Path>,
        content: &[u8],
    ) -> anyhow::Result<(NesRom, MapperBox)> {
        let mut buf = self
            .load_rom(content)
            .map_err(|e| anyhow::anyhow!(format!("Error during load: {}", e)))?;

        if buf.len() < HEADER_SIZE {
            bail!(
                "Invalid ROM: expected at least {HEADER_SIZE} bytes for the header, found {}",
                buf.len()
            );
        }

        let header = self.load_header(array_ref![buf, 0, HEADER_SIZE])?;

        let _: Vec<u8> = buf.drain(0..HEADER_SIZE).collect();

        let expected_size = expected_rom_size(&header);
        if buf.len() < expected_size {
            error!(
                "ROM is truncated: expected {expected_size} bytes after the header, found {}",
                buf.len()
            );
            bail!(
                "Invalid ROM: expected {expected_size} bytes after the header, found {}",
                buf.len()
            );
        }

        let mapper = instantiate_mapper(&header, buf).map_err(|err| {
            anyhow::anyhow!(format!(
                "Can't decode mapping number {}: {}",
                header.mapping_number(),
                err
            ))
        })?;

        Ok((NesRom::new(file_path, header), mapper))
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use test_case::test_case;

    use crate::{
        arch::rom_structs::{CHR_ROM_BANK_SIZE, HEADER, PRG_ROM_BANK_SIZE, TRAINER_SIZE},
        loaders::{flat_loader::FlatLoader, loader::Loader},
    };

    const ROM_SIZE: usize = PRG_ROM_BANK_SIZE * 2 + CHR_ROM_BANK_SIZE;

    fn rom(trainer: bool, data_size: usize) -> Vec<u8> {
        let mut rom = header_bytes(2).to_vec();
        if trainer {
            rom[6] |= 1 << 2;
        }
        rom.resize(rom.len() + data_size, 0);
        rom
    }

    fn header_bytes(prg_rom_banks: u8) -> [u8; 16] {
        let mut buf = [0u8; 16];
        buf[0..4].copy_from_slice(HEADER);
        buf[4] = prg_rom_banks;
        buf[5] = 1;
        buf
    }

    #[test]
    fn load_header_valid() {
        let header = FlatLoader.load_header(&header_bytes(2)).unwrap();
        assert_eq!(header.prg_rom_banks(), 2);
        assert_eq!(header.chr_rom_banks(), 1);
    }

    #[test]
    fn load_header_invalid_magic() {
        let mut buf = header_bytes(2);
        buf[3] = 0;
        let err = FlatLoader.load_header(&buf).unwrap_err();
        assert_eq!(err.to_string(), "Invalid header!");
    }

    #[test]
    fn load_header_no_prg_banks() {
        let err = FlatLoader.load_header(&header_bytes(0)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid header: the ROM has no PRG ROM banks"
        );
    }

    #[test_case(false, ROM_SIZE; "exact size")]
    #[test_case(false, ROM_SIZE + 100; "trailing data")]
    #[test_case(true, TRAINER_SIZE + ROM_SIZE; "with trainer")]
    fn load_rom_struct_valid(trainer: bool, data_size: usize) {
        let (rom, mapper) = FlatLoader
            .load_rom_struct(
                Some(&PathBuf::from_str("foo/bar").unwrap()),
                &rom(trainer, data_size),
            )
            .unwrap();
        assert_eq!(rom.header.prg_rom_banks(), 2);
        assert_eq!(rom.file_path, PathBuf::from_str("foo/bar").ok());
        assert_eq!(mapper.name(), "Mapper0");
    }

    #[test_case(false, 0)]
    #[test_case(false, ROM_SIZE - 1)]
    #[test_case(true, ROM_SIZE)]
    #[test_case(true, TRAINER_SIZE + ROM_SIZE - 1)]
    fn load_rom_struct_truncated(trainer: bool, data_size: usize) {
        let expected = if trainer {
            TRAINER_SIZE + ROM_SIZE
        } else {
            ROM_SIZE
        };
        let result = FlatLoader.load_rom_struct(None, &rom(trainer, data_size));
        assert_eq!(
            result.err().unwrap().to_string(),
            format!("Invalid ROM: expected {expected} bytes after the header, found {data_size}")
        );
    }

    #[test_case(0)]
    #[test_case(15)]
    fn load_rom_struct_shorter_than_header(size: usize) {
        let result = FlatLoader.load_rom_struct(None, &header_bytes(2)[..size]);
        assert_eq!(
            result.err().unwrap().to_string(),
            format!("Invalid ROM: expected at least 16 bytes for the header, found {size}")
        );
    }
}
