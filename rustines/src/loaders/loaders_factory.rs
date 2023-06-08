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
    use test_case::test_case;

    use super::decode_loader;

    #[test_case("zip", "ZipLoader"; "zip")]
    #[test_case("", "FlatLoader"; "empty")]
    #[test_case("nes", "FlatLoader"; "nes")]
    fn test_ext(ext: &str, name: &str) {
        let loader = decode_loader(ext);

        assert_eq!(loader.name(), name);
    }
}
