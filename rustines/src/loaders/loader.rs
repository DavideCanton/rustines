use std::fs::File;

use arrayref::array_ref;
use log::{debug, error, info};

use crate::arch::{
    mappers::mapper_factory::instantiate_mapper,
    rom_structs::{INesHeader, NesRom},
};
use crate::utils::named::Named;

pub trait Loader: Named {
    fn load_rom(&self, file: &mut File) -> std::io::Result<Vec<u8>>;

    fn load_header(&self, buf: &[u8]) -> Result<INesHeader, String> {
        let header = INesHeader::from_bytes(array_ref![buf[0..16], 0, 16]);

        if &header.header != b"NES\x1A" {
            error!("Found unexpected {:?} header", header.header);
            return Err("Invalid header!".to_string());
        }

        info!("Mapper: {}", header.mapping_number());
        debug!("Found header: {:?}", header);

        Ok(header)
    }

    fn load_rom_struct(&self, file: &mut File) -> Result<NesRom, String> {
        let buf = self
            .load_rom(file)
            .map_err(|e| format!("Error during load: {}", e))?;

        if cfg!(debug) {
            std::fs::write("dump.hex", &buf).unwrap();
        }

        let header = self.load_header(&buf)?;
        let mapping_number = header.mapping_number();

        let mapper = instantiate_mapper(mapping_number)
            .ok_or_else(|| format!("Can't decode mapping number {}", mapping_number))?;

        let prg_banks = mapper.load_prg_rom(&buf, &header);

        Ok(NesRom::new(header, prg_banks, buf.len()))
    }
}
