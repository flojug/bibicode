#[macro_use]

extern crate clap;
use clap::App;

//use std::process;
use std::fs;
use std::fs::File;
use std::path::Path;
//use std::io;

extern crate bibicode;
use bibicode::{BibiCoder, BibiError, NumeralSystem};

extern crate xdg;

extern crate serde_derive;
use serde_derive::{Deserialize, Serialize};

extern crate indexmap;
use indexmap::map::IndexMap;
use std::collections::HashMap;

// get numeral system from file
// the file is a json description of a numeral system
// example :
// {
//    "prefix":"0x",
//    "digits":["0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"]
// }
// digits can be the combination of any arrays
// example :
// {  "digits":[["H", "B", "K", "D"],["O", "A", "E", "I"]] }
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

    if File::open(path).is_err() {
        return Err(BibiError::BadNumeralSystem);
    }

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
                Ok(fakenum) => {
                    return NumeralSystem::new_from_strings(fakenum.prefix, vec![fakenum.digits])
                }
                Err(_) => return Err(BibiError::BadNumeralSystem),
            }
        }
    };
}

// extract prefix from a file (file = description of numeral system)
// this function is used to get the prefix of numeral systems without
// instantiate the numeral system
fn extract_prefix_from_path(path: &str) -> Option<String> {
    if File::open(path).is_err() {
        return None;
    }
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => {
            return None;
        }
    };
    let test: serde_json::Value = serde_json::from_str(&contents).unwrap();
    if !test["prefix"].is_null() {
        return Some(test["prefix"].as_str().unwrap().to_string());
    }
    None
}

fn main() -> Result<(), BibiError> {
    let yaml = load_yaml!("bibic.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let known_prefixes_from_tags: HashMap<String, String> = NumeralSystem::get_prefixes_from_tags();

    let xdg_dirs = xdg::BaseDirectories::with_prefix("bibicode").unwrap();
    let mut known_prefixes_from_xdgs: HashMap<String, String> = HashMap::new();

    let json_files = xdg_dirs.list_data_files("");
    let mut xdg_nums: IndexMap<&str, &str> = IndexMap::new();
    for json_file in json_files.iter() {
        let filename = json_file.file_stem();
        if let Some(num) = filename {
            xdg_nums
                .entry(num.to_str().unwrap())
                .or_insert(json_file.to_str().unwrap());
            if let Some(prefix) = extract_prefix_from_path(json_file.to_str().unwrap()) {
                known_prefixes_from_xdgs.insert(prefix, json_file.to_str().unwrap().to_string());
            }
        }
    }
    // closure to build in and out numeral system
    let init_num = |entry: &str| -> Result<NumeralSystem, BibiError> {
        if Path::new(entry).exists() {
            return num_from_path(entry);
        } else {
            match NumeralSystem::new_from_tag(entry) {
                Ok(num) => return Ok(num),
                Err(_) => {
                    // try xdgs files
                    if xdg_nums.contains_key(entry) {
                        return num_from_path(xdg_nums[entry]);
                    } else {
                        return Err(BibiError::BadNumeralSystem);
                    }
                }
            }
        }
    };

    let strfrom = matches.value_of("from").unwrap_or("dec");
    let mut from: NumeralSystem;
    if matches.value_of("from").is_none() {
        // if entry num system not given, try to find it out
        // from the prefix of input number
        let input_numbers: Vec<_> = matches.values_of("INPUT_NUMBER").unwrap().collect();
        let number: &str = input_numbers.first().unwrap();
        from = init_num(strfrom)?;
        let mut prefok = false;
        for pref in known_prefixes_from_tags.keys() {
            if (pref.len() > 0) && (pref[..] == number[0..pref.len()]) {
                prefok = true;
                from = NumeralSystem::new_from_tag(&known_prefixes_from_tags[pref][..]).unwrap();
            }
        }
        if !prefok {
            for pref in known_prefixes_from_xdgs.keys() {
                if (pref.len() > 0) && (pref[..] == number[0..pref.len()]) {
                    prefok = true;
                    from = num_from_path(&known_prefixes_from_xdgs[pref][..]).unwrap();
                }
            }
        }
        if !prefok {
            from = NumeralSystem::new_from_tag("dec").unwrap();
        }
    } else {
        from = init_num(strfrom)?;
    }

    let strto = matches.value_of("to").unwrap_or("dec");
    let mut to: NumeralSystem = init_num(strto)?;
    let mut res = String::from("");

    if matches.is_present("concat") {
        res = to.get_prefix();
        to.set_prefix("");
    }


    let mut input_numbers: Vec<String> = vec![];

    if let Some(inb) = matches.values_of("INPUT_NUMBER") {
        //input_numbers = inb.unwrap();
        if matches.is_present("regex") {
            let reg = matches.value_of("regex").unwrap();
            for input_number in inb {
                let nums = BibiCoder::extract_numbers(input_number, reg)?;
                for nextnum in nums {
                    input_numbers.push(nextnum);
                }
            }
        } else {
            // no regex
            for inn in inb {
                input_numbers.push(String::from(inn));
            }
        }
    } else {
        // no input number read from stdin
        /*let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;*/
    }

    let coder = BibiCoder::new(from, to);

    let mut sep = "";
    if matches.is_present("outseparator") {
        sep = matches.value_of("outseparator").unwrap();
    }

    let mut length = input_numbers.len();
    for input_number in input_numbers.iter() {
        let output_number = coder.swap(input_number)?;
        if matches.is_present("concat") {
            res = res + &output_number;
        } else {
            res = res + &output_number;
            if length > 1 {
                res = res + &String::from(sep);
            }
        }
        length = length - 1;
    }

    let mut pref = "";
    if matches.is_present("outprefix") {
        pref = matches.value_of("outprefix").unwrap();
    }

    let mut suff = "";
    if matches.is_present("outsuffix") {
        suff = matches.value_of("outsuffix").unwrap();
    }

    println!("{}{}{}", pref, res, suff);
    Ok(())
}
