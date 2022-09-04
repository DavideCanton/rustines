pub struct Context {
    pub subcommand: String,
    pub rom_name: String
}

impl Context {
    pub fn build_context(matches: &clap::ArgMatches) -> Context {
        Context {
            subcommand: matches.subcommand().unwrap().0.to_owned(),
            rom_name: matches.value_of("INPUT").unwrap().to_string(),
        }
    }
}