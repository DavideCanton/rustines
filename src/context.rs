pub struct Context {
    pub disassemble: bool,
    pub rom_name: String
}

impl Context {
    pub fn build_context(matches: &clap::ArgMatches) -> Context {
        Context {
            disassemble: matches.occurrences_of("disassemble") > 0,
            rom_name: matches.value_of("INPUT").unwrap().to_string(),
        }
    }
}