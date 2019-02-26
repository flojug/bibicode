
#[macro_use]

extern crate clap;
use clap::{App};

//use std::process;
use std::fs;
use std::fs::File;
use std::path::Path;

extern crate bibicode;
use bibicode::{NumeralSystem, BibiCoder, BibiError};

extern crate xdg;

extern crate serde_derive;
use serde_derive::{Serialize, Deserialize};

extern crate indexmap;
use indexmap::map::IndexMap;

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


fn main() -> Result<(), BibiError> {

    let xdg_dirs = xdg::BaseDirectories::with_prefix("bibicode").unwrap();

    let json_files = xdg_dirs.list_data_files("");
    let mut xdg_nums: IndexMap<&str, &str> = IndexMap::new();
    for json_file in json_files.iter() {
        let filename = json_file.file_stem();
        if let Some(num) = filename {
            xdg_nums.entry(num.to_str().unwrap()).or_insert(json_file.to_str().unwrap());
        }
    }

    let yaml = load_yaml!("bibic.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let init_num = |entry: &str| -> Result<NumeralSystem, BibiError> {
        if Path::new(entry).exists() {
            match num_from_path(entry) {
                Ok(num) => return Ok(num),
                Err(err) => return Err(err),
            }
        } else {
            match NumeralSystem::new_from_tag(entry) {
                Ok(num) => return Ok(num),
                Err(_) => {
                    // try xdgs files
                    if xdg_nums.contains_key(entry) {
                        match num_from_path(xdg_nums[entry]) {
                            Ok(num) => return Ok(num),
                            Err(err) => return Err(err),
                        }
                    } else {
                        return Err(BibiError::BadNumeralSystem);
                    }
                }
            }
        }
    };

    let strfrom = matches.value_of("from").unwrap_or("dec");
    let from: NumeralSystem = match init_num(strfrom) {
        Ok(num) => num,
        Err(err) => return Err(err),
    };

    let strto = matches.value_of("to").unwrap_or("dec");
    let mut to: NumeralSystem = match init_num(strto) {
        Ok(num) => num,
        Err(err) => return Err(err),
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
                return Err(err);
            }
        };
        if matches.is_present("concat") {
            res = res + &output_number;
        } else {
            res = res + &output_number + &String::from(" ");
        }
    }

    println!("{}", res);
    Ok(())
}

