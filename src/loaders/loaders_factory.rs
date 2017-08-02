use loaders::loaders::Loader;
use loaders::flat_loader::FlatLoader;
use loaders::zip_loader::ZipLoader;

pub struct LoadersFactory{}

impl LoadersFactory {
    pub fn decode(extension: &str) -> Box<Loader> {
        match extension {
            "zip" => Box::new(ZipLoader::new()),
            _     => Box::new(FlatLoader::new())
        }
    }
}