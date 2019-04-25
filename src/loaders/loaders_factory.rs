use crate::loaders::flat_loader::FlatLoader;
use crate::loaders::loader::Loader;
use crate::loaders::zip_loader::ZipLoader;
use log::{info, log};

pub struct LoadersFactory;

impl LoadersFactory {
    pub fn decode(extension: &str) -> Box<dyn Loader> {
        let loader: Box<dyn Loader> = match extension {
            "zip" => Box::new(ZipLoader::new()),
            _ => Box::new(FlatLoader::new()),
        };

        info!("Selected loader {}", loader.name());

        loader
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_detects_zip_correctly() {
        let ext = "zip";
        let loader = LoadersFactory::decode(ext);

        assert!(loader.as_any().is::<ZipLoader>());
    }

    #[test]
    fn it_detects_empty_correctly() {
        let ext = "";
        let loader = LoadersFactory::decode(ext);

        assert!(loader.as_any().is::<FlatLoader>());
    }

    #[test]
    fn it_detects_nes_correctly() {
        let ext = "nes";
        let loader = LoadersFactory::decode(ext);

        assert!(loader.as_any().is::<FlatLoader>());
    }
}
