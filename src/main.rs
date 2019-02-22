
#[macro_use]

extern crate clap;
use clap::{App};

use std::process;
use std::fs;
use std::fs::File;
use std::path::Path;

extern crate bibicode;
use bibicode::{NumeralSystem, BibiCoder, BibiError};


extern crate serde_derive;
use serde_derive::{Serialize, Deserialize};



fn num_from_path(path: &str) -> Result<NumeralSystem, BibiError> {

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FakeNumeralSystem {
        #[serde(default)]
        prefix: String,
        digits: Vec<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FakeNumeralSystem2 {
        #[serde(default)]
        prefix: String,
        digits: Vec<String>,
    }

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

    let test: Result<FakeNumeralSystem, _> = serde_json::from_str(&contents);
    match test {
        Ok(fakenum) => return NumeralSystem::new_from_strings(fakenum.prefix, fakenum.digits),
        Err(_) => {
            let test: Result<FakeNumeralSystem2, _> = serde_json::from_str(&contents);
            match test {
                Ok(fakenum) => return NumeralSystem::new_from_strings(fakenum.prefix, vec!(fakenum.digits)),
                Err(_) => return Err(BibiError::BadNumeralSystem)
            }
        }
    };
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

    let mut to: NumeralSystem = match res {
        Ok(numsys) => numsys,
        Err(err) => {
            eprintln!("error: {:?}", err);
            process::exit(1);
        }
    };

    let input_numbers: Vec<_> = matches.values_of("INPUT_NUMBER").unwrap().collect();

    let mut res = String::from("");

    if matches.is_present("concat") {
        res = to.get_prefix();
        to.set_prefix("");
    }

    let coder = BibiCoder::new(from, to);

    for input_number in input_numbers.iter() {
        let output_number = match coder.swap(input_number) {
            Ok(on) => on,
            Err(err) => {
                eprintln!("error: {:?}", err);
                process::exit(1);
            }
        };
        if matches.is_present("concat") {
            res = res + &output_number;
        } else {
            res = res + &output_number + &String::from(" ");
        }
    }

    println!("{}", res);
    process::exit(0);
}

