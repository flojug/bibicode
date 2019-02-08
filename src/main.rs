
#[macro_use]

extern crate clap;
use clap::{App};

use std::process;
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

    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => {
            return Err(BibiError::BadNumeralSystem);
        }
    };

    let ret: NumeralSystem = serde_json::from_str(&contents).unwrap();

    Ok(ret)
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

    let from: NumeralSystem = match res {
        Ok(numsys) => numsys,
        Err(err) => {
            eprintln!("error: {:?}", err);
            process::exit(1);
        }
    };

    let strto = matches.value_of("to").unwrap_or("dec");
    if Path::new(strto).exists() {
        res = num_from_path(strto);
    } else {
        res = NumeralSystem::new_from_tag(strto);
    }

    let to: NumeralSystem = match res {
        Ok(numsys) => numsys,
        Err(err) => {
            eprintln!("error: {:?}", err);
            process::exit(1);
        }
    };

    let input_numbers: Vec<_> = matches.values_of("INPUT_NUMBER").unwrap().collect();

    let coder = BibiCoder::new(from, to);

    let mut res = String::from("");
    for input_number in input_numbers.iter() {
        let output_number = match coder.swap(input_number) {
            Ok(on) => on,
            Err(err) => {
                eprintln!("error: {:?}", err);
                process::exit(1);
            }
        };
        res = res + &output_number + &String::from(" ");
    }

    println!("{}", res);
    process::exit(0);
}

