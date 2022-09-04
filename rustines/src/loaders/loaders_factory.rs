use crate::loaders::{flat_loader::FlatLoader, loader::Loader, zip_loader::ZipLoader};
use log::info;

pub fn decode_loader(extension: &str) -> Box<dyn Loader> {
    let loader: Box<dyn Loader> = match extension {
        "zip" => Box::new(ZipLoader),
        _ => Box::new(FlatLoader),
    };

    info!("Selected loader {}", loader.name());

    loader
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_detects_zip_correctly() {
        let ext = "zip";
        let loader = decode_loader(ext);

        assert_eq!(loader.name(), "ZipLoader");
    }

    #[test]
    fn it_detects_empty_correctly() {
        let ext = "";
        let loader = decode_loader(ext);

        assert_eq!(loader.name(), "FlatLoader");
    }

    #[test]
    fn it_detects_nes_correctly() {
        let ext = "nes";
        let loader = decode_loader(ext);

        assert_eq!(loader.name(), "FlatLoader");
    }
}
