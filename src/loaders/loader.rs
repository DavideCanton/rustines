use crate::arch::{
    mappers::mapper_factory::instantiate_mapper,
    rom_structs::{Header, NesRom},
};
use arrayref::array_ref;
use log::{error, info, log};
use std;
use std::{any::Any, fs::File};

pub trait Loader: Any {
    fn load_rom(&self, file: &mut File) -> std::io::Result<Vec<u8>>;
    fn name(&self) -> String;
    fn as_any(&self) -> &Any;

    fn load_header(&self, buf: &[u8]) -> Result<Header, String> {
        let header = Header::from_bytes(array_ref![buf[0..16], 0, 16]);

        if &header.header != b"NES\x1A" {
            error!("Found unexpected {:?} header", header.header);
            return Err("Invalid header!".to_string());
        }

        info!("Mapper: {}", header.mapping_number());

        Ok(header)
    }

    fn load_rom_struct(&self, file: &mut File) -> Result<NesRom, String> {
        let buf = self
            .load_rom(file)
            .map_err(|e| format!("Error during load: {}", e))?;

        let header = self.load_header(&buf)?;
        let mapping_number = header.mapping_number();

        let mapper = instantiate_mapper(mapping_number)
            .ok_or_else(|| format!("Can't decode mapping number {}", mapping_number))?;

        let prg_banks = mapper.load_prg_rom(&buf, &header);

        Ok(NesRom::new(header, prg_banks, buf.len()))
    }
}
