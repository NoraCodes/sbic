extern crate sbrain;
extern crate clap;

use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use std::env::args;
use std::error::Error;

use clap::{App, Arg};

/// Exit from the program on unrecoverable failure.
fn efail(s: String) -> ! {
    println!("FATAL: {}", s);
    std::process::exit(1);
}


struct ProgramInputs {
    sourcefile: PathBuf,
    inputfile: Option<PathBuf>,
    cycles: Option<u32>,
    source: String,
    input: Vec<u8>,
    bytes_output: bool,
    verbose: bool,
}


fn get_inputs() -> ProgramInputs {

    let matches = App::new("SBrain Interpreter and Compiler")
                          .version(env!("CARGO_PKG_VERSION"))
                          .author("Leo Tindall <lfstindall@gmail.com>")
                          .about("Executes and compiles SBrain programs")
                          .arg(Arg::with_name("SOURCE")
                               .help("The source file to operate on")
                               .required(false)
                               .index(1))
                          .arg(Arg::with_name("input")
                               .short("i")
                               .long("input")
                               .help("The file to use for the program's input")
                               .value_name("FILE")
                               .takes_value(true))
                          .arg(Arg::with_name("cycles")
                               .short("c")
                               .long("cycles")
                               .help("The maximum number of cycles for which to run")
                               .value_name("CYCLES")
                               .takes_value(true))
                          .arg(Arg::with_name("bytes_output")
                               .short("b")
                               .long("bytes-output")
                               .help("Display output as decimal bytes, not lossy text."))
                          .arg(Arg::with_name("verbose")
                               .short("v")
                               .long("verbose")
                               .help("Display more info."))
                          .get_matches();

    let sourcefile: PathBuf = matches.value_of("SOURCE")
                            .expect("You must provide a source file")
                            .into();

    let inputfile: Option<PathBuf> = matches.value_of("input")
                            .and_then(|s| Some(s.into()));

    let cycles = matches.value_of("cycles")
        .and_then(
            |s| Some(u32::from_str_radix(s, 10).expect("Invalid number of cycles.")));
    
    // Now read the actual files
    let mut source = String::new();
    File::open(&sourcefile)
        .unwrap_or_else(|e| efail(format!("Couldn't open {}: {}", 
                                   sourcefile.display(),
                                   e.description())))
        .read_to_string(&mut source)
        .unwrap_or_else(|e| efail(format!("Couldn't read {}: {}",
                                   sourcefile.display(),
                                   e.description())));

    // Input will be an empty vec unless some is provided.
    let mut input = Vec::<u8>::new();
    if let Some(inputfile) = inputfile.clone() {
        File::open(&inputfile)
            .unwrap_or_else(|e| efail(format!("Couldn't open {}: {}",
                                       inputfile.display(),
                                       e.description())))
            .read_to_end(&mut input)
            .unwrap_or_else(|e| efail(format!("Couldn't read {}: {}",
                                       inputfile.display(),
                                       e.description())));
    }
    
    ProgramInputs {
        sourcefile: sourcefile,
        inputfile: inputfile,
        cycles: cycles,
        source: source,
        input: input,
        bytes_output: matches.is_present("bytes_output"),
        verbose: matches.is_present("verbose"),
    }


}

fn main() {
    let inputs = get_inputs();

    let mut machine = sbrain::SBrainVM::new(
        Some(inputs.input.iter().map(|n| *n as u32).collect()));
    let (program_tape, data_tape) = sbrain::source_to_tapes(&inputs.source);
    machine.load_program(&program_tape).unwrap();
    machine.load_data(&data_tape).unwrap();

    machine.run(inputs.cycles);

    if inputs.bytes_output {
        println!("{:?}", machine.get_output());
    } else {
        println!("{}", &sbrain::tape_to_string(machine.get_output()));
    }
}
