extern crate sbrain;
extern crate toml;
#[macro_use]
extern crate serde_derive;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::env::args;

#[derive(Debug, Clone, Deserialize)]
struct Configuration {
    pub input: Vec<u32>,
    pub source: String,
    pub max_runtime: Option<u32>
}

fn read_config(filename: &Path) -> Configuration {

    let mut config = String::new();
    let mut f = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {panic!(format!("Failed to open file {}: {}", filename.to_string_lossy(), e));}
    };

    f.read_to_string(&mut config).expect("Could not read from file.");

    let config: Configuration = match toml::from_str(&config) {
        Ok(c) => c,
        Err(e) => {panic!(format!("Failed to parse configuration file: {}", e))}
    };

    config
}

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() != 2 {
        panic!("You must specify a configuration file!")
    }

    let config = read_config(&Path::new(&args[1]));

    let mut machine = sbrain::SBrainVM::new(Some(config.input.clone()));
    let (program_tape, data_tape) = sbrain::source_to_tapes(&config.source);
    machine.load_program(&program_tape).unwrap();
    machine.load_data(&data_tape).unwrap();

    machine.run(config.max_runtime);

    println!("{:?}", machine.get_output());
}
