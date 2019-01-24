
#[macro_use]

extern crate clap;
use clap::{App};

use std::fs;
use std::fs::File;
use std::path::Path;

extern crate bibicode;
use bibicode::{NumeralSystem, BibiCoder, BibiError};


fn num_from_path(path: &str) -> Result<NumeralSystem, BibiError> {

    match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            return Err(BibiError::BadNumeralSystem);
        }
    };

    let mut entry: Vec<Vec<String>> = vec!();
    let mut digits: Vec<String> = vec!();

    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => {
            return Err(BibiError::BadNumeralSystem);
        }
    };

    for line in contents.lines() {
        if line == "===" {
            entry.push(digits);
            digits = vec!();
        } else {
            digits.push(String::from(line));
        }
    }
    entry.push(digits);

    NumeralSystem::new_from_strings(entry)
}


fn main() {

    let yaml = load_yaml!("bibic.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut res;
    let strfrom = matches.value_of("from").unwrap_or("dec");
    if Path::new(strfrom).exists() {
        res = num_from_path(strfrom);
    } else {
        res = NumeralSystem::new_from_tag(strfrom);
    }

    let from: NumeralSystem;
    match res {
        Ok(num) => from = num,
        Err(_) => {
            return;
        }
    };

    let strto = matches.value_of("to").unwrap_or("dec");
    if Path::new(strto).exists() {
        res = num_from_path(strto);
    } else {
        res = NumeralSystem::new_from_tag(strto);
    }

    let to: NumeralSystem;
    match res {
        Ok(num) => to = num,
        Err(_) => {
            return;
        }
    };

    let input_number = matches.value_of("INPUT_NUMBER").unwrap();

    let coder = BibiCoder::new(from, to);
    let output_number = coder.swap(input_number).unwrap();
    println!("{}", output_number);
}



