// Copyright ⓒ 2019 Florent Jugla
//
// Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//
//
// ! # Bibicode
//!
//! This crate can be used to convert any natural number from one numeral system to another.
//!
//! Two numeral systems must be defined : one for the input number and one for the output.
//!
//! Any natural number (of any length) can then be converted from one system to the other and vice-versa.
//!
//! This library uses an extension of shift-adjust algorithm (and reversed shift-adjust) to convert numbers. Binary is used as a pivot radix. This method was described here : [Convert binary number to any base](https://www.edn.com/design/systems-design/4460458/Convert-binary-number-to-any-base).
//!
//! It was named after french singer (and also mathematician) [Boby Lapointe](https://en.wikipedia.org/wiki/Boby_Lapointe) who invented the [Bibi-binary system](https://en.wikipedia.org/wiki/Bibi-binary) in 1968.
//!
//! ## Exemple
//!
//!        extern crate bibicode;
//!
//!        let dec = bibicode::NumeralSystem::new("", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
//!        let bibi = bibicode::NumeralSystem::new("",  vec!(vec!("HO", "HA", "HE", "HI", "BO", "BA", "BE", "BI", "KO", "KA", "KE", "KI", "DO", "DA", "DE", "DI"))).unwrap();
//!        let coder = bibicode::BibiCoder::new(dec, bibi);
//!        let test = coder.swap("2000").unwrap();
//!        assert_eq!(test, "BIDAHO");
//!
//!        let bibi = bibicode::NumeralSystem::new("", vec!(vec!("H", "B", "K", "D"), vec!("O", "A", "E", "I"))).unwrap();
//!        let dec = bibicode::NumeralSystem::new("", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();

//!        let coder = bibicode::BibiCoder::new(dec, bibi);
//!        let test = coder.swap("2000").unwrap();
//!        assert_eq!(test, "BIDAHO");
//!
//!        // with prefixed numeral system
//!        let dec = bibicode::NumeralSystem::new("", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
//!        let hex = bibicode::NumeralSystem::new("0x", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"))).unwrap();
//!        let coder = bibicode::BibiCoder::new(dec, hex);
//!        let test = coder.swap("2000").unwrap();
//!        assert_eq!(test, "0x7d0");
//!
//!        let dec = bibicode::NumeralSystem::new("", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
//!        let hex = bibicode::NumeralSystem::new("0x", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"))).unwrap();
//!        let coder = bibicode::BibiCoder::new(hex, dec);
//!        let test = coder.swap("0x7d0").unwrap();
//!        assert_eq!(test, "2000");
//!        // will also work
//!        let test = coder.swap("7d0").unwrap();
//!        assert_eq!(test, "2000");

use std::char;
use std::fmt;

extern crate indexmap;
use indexmap::map::IndexMap;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

#[derive(Debug)]
pub enum BibiError {
    /// Malformed numeral system : all digits must have the same length and be unique
    BadNumeralSystem,
    /// One digit given in the entry was not found in numeral system
    EntryMismatchWithNumeralSystem,
    RegexMismatchWithEntry,
    /// Non existent pre-defined numeral system
    BadTagNumeralSystem,
    /// Bad regular expression
    BadRegularExpression,
}

/// Define a numeral system by enumerating all the digits. The first digit is zero. The radix is equal to the number of digits. One digit can have any number of characters but all digits must have the same length.
#[derive(Debug)]
pub struct NumeralSystem {
    prefix: String,
    len_digit: usize,
    digits: IndexMap<String, u32>,
    // maintains reverse list of digits to find them quickly
    revdigits: IndexMap<u32, String>,
}

impl NumeralSystem {
    // static method to find out a numeral system by its prefix given a number
    pub fn autodetect<'a>(number: &str, nums: Vec<&'a NumeralSystem>) -> Option<&'a NumeralSystem> {
        let res: Vec<&'a NumeralSystem> = nums
            .into_iter()
            .filter(|ns| (ns.prefix.len() > 0) && (ns.prefix[..] == number[0..ns.prefix.len()]))
            .collect();
        if res.len() == 1 {
            return Some(res[0]);
        }
        None
    }

    /// Returns new numeral system from the strings given. If several vecs are given to the function, figits will be made by a combination of all vecs.
    /// - Exemple for decimal system entry must be vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))
    ///
    pub fn new(prefix: &str, entry: Vec<Vec<&str>>) -> Result<NumeralSystem, BibiError> {
        return NumeralSystem::new_rec(prefix, &entry, 0);
    }

    /// Same as ::new but from vec of strings.
    pub fn new_from_strings(
        prefix: String,
        entry: Vec<Vec<String>>,
    ) -> Result<NumeralSystem, BibiError> {
        let entry_str: Vec<Vec<&str>> = entry
            .iter()
            .map(|v| v.iter().map(|s| &**s).collect())
            .collect();
        NumeralSystem::new(&prefix[..], entry_str)
    }

    // internal method to build system from vec of vec
    fn new_rec(
        prefix: &str,
        entry: &Vec<Vec<&str>>,
        index: usize,
    ) -> Result<NumeralSystem, BibiError> {
        let mut len_digit;
        let mut digits: IndexMap<String, u32> = IndexMap::new();
        let mut revdigits: IndexMap<u32, String> = IndexMap::new();

        let first_entry = &entry[index];

        match first_entry.first() {
            None => return Err(BibiError::BadNumeralSystem),
            Some(digit) => len_digit = digit.len(),
        }

        if len_digit == 0 {
            return Err(BibiError::BadNumeralSystem);
        }

        let mut sub_num_sys = None;
        if index < entry.len() - 1 {
            sub_num_sys = Some(NumeralSystem::new_rec(prefix, entry, index + 1)?);
        }

        let mut cpt: u32 = 0;
        for digit in first_entry {
            if digit.len() != len_digit {
                return Err(BibiError::BadNumeralSystem);
            }
            if let Some(num) = sub_num_sys {
                for digit2 in num.digits.keys() {
                    digits
                        .entry(String::from(*digit) + &String::from(&digit2[..]))
                        .or_insert(cpt);
                    revdigits.insert(cpt, String::from(*digit) + &String::from(&digit2[..]));
                    cpt = cpt + 1;
                }
                sub_num_sys = Some(num);
            } else {
                digits.entry(String::from(*digit)).or_insert(cpt);
                revdigits.insert(cpt, String::from(*digit));
                cpt = cpt + 1;
            }
        }

        // check for unique digits
        if cpt != digits.len() as u32 {
            return Err(BibiError::BadNumeralSystem);
        }

        if let Some(num) = sub_num_sys {
            len_digit = len_digit + num.len_digit;
        }

        Ok(NumeralSystem {
            prefix: String::from(prefix),
            len_digit: len_digit,
            digits: digits,
            revdigits: revdigits,
        })
    }

    /// Returns pre-defined numeral systems :
    /// - dec for decimal
    /// - hex for hexadecimal
    /// - bibi for "bibi" as defined by Boby Lapointe
    /// - bin for binary
    /// - budu for a test system easy to read
    /// - utf8 for a test system with UTF8 characters
    /// - base58 for base58 as used in bitcoin
    ///
    /// UTILISER lazy-static
    fn get_tags() -> IndexMap<&'static str, (String, Vec<Vec<String>>)> {
        let chin_factory = || -> Vec<Vec<String>> {
            let mut ret: Vec<Vec<String>> = vec![vec![]];
            for x in 0x3400..0x4000 {
                let ptrc = char::from_u32(x).unwrap().to_string();
                ret[0].push(ptrc);
            }
            ret
        };
        let mut tags: IndexMap<&'static str, (String, Vec<Vec<String>>)> = IndexMap::new();
        tags.insert(
            "bin",
            (
                String::from("0b"),
                vec![vec![String::from("0"), String::from("1")]],
            ),
        );
        tags.insert(
            "oct",
            (
                String::from("0o"),
                vec![vec![
                    String::from("0"),
                    String::from("1"),
                    String::from("2"),
                    String::from("3"),
                    String::from("4"),
                    String::from("5"),
                    String::from("6"),
                    String::from("7"),
                ]],
            ),
        );
        tags.insert(
            "dec",
            (
                String::from(""),
                vec![vec![
                    String::from("0"),
                    String::from("1"),
                    String::from("2"),
                    String::from("3"),
                    String::from("4"),
                    String::from("5"),
                    String::from("6"),
                    String::from("7"),
                    String::from("8"),
                    String::from("9"),
                ]],
            ),
        );
        tags.insert(
            "hex",
            (
                String::from("0x"),
                vec![vec![
                    String::from("0"),
                    String::from("1"),
                    String::from("2"),
                    String::from("3"),
                    String::from("4"),
                    String::from("5"),
                    String::from("6"),
                    String::from("7"),
                    String::from("8"),
                    String::from("9"),
                    String::from("a"),
                    String::from("b"),
                    String::from("c"),
                    String::from("d"),
                    String::from("e"),
                    String::from("f"),
                ]],
            ),
        );
        tags.insert(
            "bibi",
            (
                String::from(""),
                vec![vec![
                    String::from("HO"),
                    String::from("HA"),
                    String::from("HE"),
                    String::from("HI"),
                    String::from("BO"),
                    String::from("BA"),
                    String::from("BE"),
                    String::from("BI"),
                    String::from("KO"),
                    String::from("KA"),
                    String::from("KE"),
                    String::from("KI"),
                    String::from("DO"),
                    String::from("DA"),
                    String::from("DE"),
                    String::from("DI"),
                ]],
            ),
        );
        tags.insert(
            "budu",
            (
                String::from(""),
                vec![
                    vec![
                        String::from("B"),
                        String::from("K"),
                        String::from("D"),
                        String::from("F"),
                        String::from("G"),
                        String::from("J"),
                        String::from("L"),
                        String::from("M"),
                        String::from("N"),
                        String::from("P"),
                        String::from("R"),
                        String::from("S"),
                        String::from("T"),
                        String::from("V"),
                        String::from("X"),
                        String::from("Z"),
                    ],
                    vec![
                        String::from("a"),
                        String::from("i"),
                        String::from("o"),
                        String::from("u"),
                    ],
                ],
            ),
        );
        tags.insert(
            "utf8",
            (
                String::from(""),
                vec![
                    vec![
                        String::from("\u{25a0}"),
                        String::from("\u{25c0}"),
                        String::from("\u{25cf}"),
                        String::from("\u{2660}"),
                        String::from("\u{2665}"),
                        String::from("\u{2666}"),
                        String::from("\u{2663}"),
                        String::from("\u{2691}"),
                        String::from("\u{25c6}"),
                        String::from("\u{2605}"),
                    ],
                    vec![
                        String::from("\u{25a1}"),
                        String::from("\u{25c1}"),
                        String::from("\u{25cb}"),
                        String::from("\u{2664}"),
                        String::from("\u{2661}"),
                        String::from("\u{2662}"),
                        String::from("\u{2667}"),
                        String::from("\u{2690}"),
                        String::from("\u{25c7}"),
                        String::from("\u{2606}"),
                    ],
                ],
            ),
        );
        tags.insert(
            "base58",
            (
                String::from(""),
                vec![vec![
                    String::from("1"),
                    String::from("2"),
                    String::from("3"),
                    String::from("4"),
                    String::from("5"),
                    String::from("6"),
                    String::from("7"),
                    String::from("8"),
                    String::from("9"),
                    String::from("A"),
                    String::from("B"),
                    String::from("C"),
                    String::from("D"),
                    String::from("E"),
                    String::from("F"),
                    String::from("G"),
                    String::from("H"),
                    String::from("J"),
                    String::from("K"),
                    String::from("L"),
                    String::from("M"),
                    String::from("N"),
                    String::from("P"),
                    String::from("Q"),
                    String::from("R"),
                    String::from("S"),
                    String::from("T"),
                    String::from("U"),
                    String::from("V"),
                    String::from("W"),
                    String::from("X"),
                    String::from("Y"),
                    String::from("Z"),
                    String::from("a"),
                    String::from("b"),
                    String::from("c"),
                    String::from("d"),
                    String::from("e"),
                    String::from("f"),
                    String::from("g"),
                    String::from("h"),
                    String::from("i"),
                    String::from("j"),
                    String::from("k"),
                    String::from("m"),
                    String::from("n"),
                    String::from("o"),
                    String::from("p"),
                    String::from("q"),
                    String::from("r"),
                    String::from("s"),
                    String::from("t"),
                    String::from("u"),
                    String::from("v"),
                    String::from("w"),
                    String::from("x"),
                    String::from("y"),
                    String::from("z"),
                ]],
            ),
        );
        tags.insert("chin", (String::from(""), chin_factory()));
        tags
    }

    pub fn get_prefixes_from_tags() -> HashMap<String, String> {
        let mut ret = HashMap::new();
        let tags = NumeralSystem::get_tags();
        for (tag, tuple) in tags.iter() {
            let pref = tuple.0.to_string();
            if !pref.is_empty() {
                ret.insert(pref, tag.to_string());
            }
        }
        ret
    }

    pub fn new_from_tag(tag: &str) -> Result<NumeralSystem, BibiError> {
        let tags = NumeralSystem::get_tags();
        if tags.contains_key(tag) {
            let prefix = tags[tag].0.clone();
            let vecd = tags[tag].1.clone();
            Ok(NumeralSystem::new_from_strings(prefix, vecd)?)
        } else {
            Err(BibiError::BadTagNumeralSystem)
        }
    }

    /// Returns the legth of a digit (all digits have the same length)
    pub fn len(&self) -> usize {
        self.digits.len()
    }

    /// Return the radix of this numeral system (= number of digits in numeral system)
    pub fn radix(&self) -> usize {
        self.len()
    }

    pub fn get_prefix(&self) -> String {
        self.prefix.clone()
    }

    pub fn set_prefix(&mut self, prefix: &str) {
        self.prefix = String::from(prefix);
    }
}

impl fmt::Display for NumeralSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp = String::from("");
        let mut sep = String::from("");
        for digit in self.digits.keys() {
            disp = disp + &sep + digit;
            sep = String::from(", ");
        }
        write!(f, "{}", disp)
    }
}

/// Convert any number from one numeral system to the other.
#[derive(Debug)]
pub struct BibiCoder {
    numsys_in: NumeralSystem,
    numsys_out: NumeralSystem,
}

impl BibiCoder {
    /// Build a coder from numsys_in numeral system to numsys_out
    pub fn new(numsys_in: NumeralSystem, numsys_out: NumeralSystem) -> BibiCoder {
        BibiCoder {
            numsys_in,
            numsys_out,
        }
    }

    /// find all numbers in entry from regular expression
    pub fn extract_numbers(entry: &str, reg: &str) -> Result<Vec<String>, BibiError> {
        let mut numbers: Vec<String> = vec![];

        if !reg.is_empty() {
            let re: Regex = match Regex::new(&reg[..]) {
                Ok(res) => res,
                Err(_) => return Err(BibiError::BadRegularExpression),
            };
            let caps = match re.captures(entry) {
                Some(caps) => caps,
                None => return Err(BibiError::RegexMismatchWithEntry),
            };
            for number in caps.iter().skip(1) {
                if let Some(numberok) = number {
                    numbers.push(numberok.as_str().to_string());
                }
            }
        } else {
            numbers.push(entry.to_string());
        }

        Ok(numbers)
    }

    /// Swap an natural number coded in numsys_in system to numsys_out
    pub fn swap(&self, entry: &str) -> Result<String, BibiError> {
        let pivot = self.tsujda_tfihs(entry)?;
        self.shift_adjust(pivot)
    }

    // compute BCD  numbers into binary
    fn tsujda_tfihs(&self, entry: &str) -> Result<Vec<bool>, BibiError> {
        // erase the prefix if present
        let rel_entry: &str;
        if (self.numsys_in.prefix.len() > 0)
            && (entry.len() > self.numsys_in.prefix.len())
            && (self.numsys_in.prefix[..] == entry[0..self.numsys_in.prefix.len()])
        {
            rel_entry = &entry[self.numsys_in.prefix.len()..];
        } else {
            rel_entry = entry;
        }

        let radix = self.numsys_in.len() as u32;

        let mut bcd: Vec<u32> = vec![];
        let mut pivot: Vec<bool> = vec![];

        // compute bcd numbers from the entry
        for i in 0..(rel_entry.len() / self.numsys_in.len_digit) {
            let digit = String::from(
                &rel_entry[i * self.numsys_in.len_digit
                    ..(i * self.numsys_in.len_digit) + self.numsys_in.len_digit],
            );
            let digidx: u32 = match self.numsys_in.digits.get(&digit) {
                Some(d) => *d,
                None => return Err(BibiError::EntryMismatchWithNumeralSystem),
            };
            bcd.push(digidx);
        }

        // reverse shift adjust
        loop {
            let mut end = true;
            let mut rel = 0;
            for idx in 0..bcd.len() {
                let nb = bcd[idx] + rel * radix;
                rel = nb % 2;
                bcd[idx] = nb / 2;
                end = if bcd[idx] > 0 { false } else { end };
            }
            pivot.insert(0, if rel == 1 { true } else { false });
            if end {
                break;
            }
        }

        Ok(pivot)
    }

    // compute  binary numbers into BCD like
    fn shift_adjust(&self, mut pivot: Vec<bool>) -> Result<String, BibiError> {
        let radix = self.numsys_out.len() as u32;

        let len_bits = pivot.len();
        let mut bcdlike: Vec<u32> = vec![0];

        for _ in 0..len_bits {
            // shift
            let bit = pivot.remove(0);
            let mut rel = if bit { 1 } else { 0 };

            // adjust
            for idx in 0..bcdlike.len() {
                let mut val = (bcdlike[idx] * 2) + rel;
                rel = 0;
                if val >= radix {
                    val = val - radix;
                    rel = 1;
                }
                bcdlike[idx] = val;
            }
            if rel == 1 {
                bcdlike.push(rel);
            }
        }

        let mut ret = self.numsys_out.prefix.clone();
        for idx in (0..bcdlike.len()).rev() {
            let val = bcdlike[idx];
            ret = ret + self.numsys_out.revdigits.get(&val).unwrap();
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeral_system() {
        let test = NumeralSystem::new("", vec![vec!["0", "1", "2", "2"]]);
        assert!(test.is_err(), "test 1 1");

        let test = NumeralSystem::new("", vec![vec!["0", "1", "2", "22"]]);
        assert!(test.is_err(), "test 1 2");

        let test = NumeralSystem::new("", vec![vec!["0", "1", "2"], vec!["0", "1", "2", "22"]]);
        assert!(test.is_err(), "test 1 3");

        let test = NumeralSystem::new("", vec![vec!["0", "1", "2"], vec!["0", "1", "2", "2"]]);
        assert!(test.is_err(), "test 1 4");

        let a = vec![
            "B", "K", "D", "F", "G", "J", "L", "M", "N", "P", "R", "S", "T", "V", "X", "Z",
        ];
        let b = vec!["a", "i", "o", "u"];
        let len1 = a.len() * b.len();
        let test = NumeralSystem::new("", vec![a, b]).unwrap();
        let len2 = test.digits.len();
        assert!(len1 == len2, "test 1 5");
    }

    #[test]
    fn test_bibicoder() {
        let dec = NumeralSystem::new(
            "",
            vec![vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]],
        )
        .unwrap();
        let bibi = NumeralSystem::new(
            "",
            vec![vec![
                "HO", "HA", "HE", "HI", "BO", "BA", "BE", "BI", "KO", "KA", "KE", "KI", "DO", "DA",
                "DE", "DI",
            ]],
        )
        .unwrap();

        let dec_to_bibi = BibiCoder::new(dec, bibi);

        let test = dec_to_bibi.swap("2000").unwrap();
        assert_eq!(test, "BIDAHO", "test 2 1");

        //let bibi = NumeralSystem::new("", "", vec!(vec!("H", "B", "K", "D"), vec!("O", "A", "E", "I"))).unwrap();
        let test = dec_to_bibi.swap("2000").unwrap();
        assert_eq!(test, "BIDAHO", "test 2 2");

        let bin = NumeralSystem::new_from_tag("bin").unwrap();
        let hex = NumeralSystem::new_from_tag("hex").unwrap();
        let dec = NumeralSystem::new_from_tag("dec").unwrap();

        let hex_to_dec = BibiCoder::new(hex, dec);

        let dec = NumeralSystem::new_from_tag("dec").unwrap();
        let hex = NumeralSystem::new_from_tag("hex").unwrap();
        let dec_to_hex = BibiCoder::new(dec, hex);

        let hex = NumeralSystem::new_from_tag("hex").unwrap();
        let hex_to_bin = BibiCoder::new(hex, bin);

        let test = hex_to_dec.swap("ffff").unwrap();
        assert_eq!(test, "65535", "test 2 3");

        let test = hex_to_bin.swap("f0ff").unwrap();
        assert_eq!(test, "0b1111000011111111", "test 2 4");

        let test = dec_to_hex
            .swap("324439924324324235436544328757654635345424324543")
            .unwrap();

        assert_eq!(
            test, "0x38d463ad8fa67a74d6e9a610158623c60d2297bf",
            "test 2 5"
        );

        let test = dec_to_hex.swap("45641230").unwrap();
        assert_eq!(test, "0x2b86e0e", "test 2 6");

        let test = hex_to_dec
            .swap("38d463ad8fa67a74d6e9a610158623c60d2297bf")
            .unwrap();

        assert_eq!(
            test, "324439924324324235436544328757654635345424324543",
            "test 2 7"
        );

        let sys = NumeralSystem::new(
            "",
            vec![
                vec![
                    "B", "K", "D", "F", "G", "J", "L", "M", "N", "P", "R", "S", "T", "V", "X", "Z",
                ],
                vec!["a-", "i-", "o-", "u-"],
            ],
        )
        .unwrap();

        let hex = NumeralSystem::new_from_tag("hex").unwrap();
        let hex_to_sys = BibiCoder::new(hex, sys);
        let test = hex_to_sys
            .swap("de0b295669a9fd93d5f28d9ec85e40f4cb697bae")
            .unwrap();

        let test2 = hex_to_sys
            .swap("0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae")
            .unwrap();

        assert_eq!(
            test,
            "Fi-Xa-Du-Do-Ji-Li-Ri-Ro-Mu-Vo-Gu-Vi-Mu-Do-Fi-Pu-Sa-Ni-Mo-Ga-Fu-Gu-Du-Lo-Ju-So-So-",
            "test 2 8"
        );
        assert_eq!(
            test2,
            "Fi-Xa-Du-Do-Ji-Li-Ri-Ro-Mu-Vo-Gu-Vi-Mu-Do-Fi-Pu-Sa-Ni-Mo-Ga-Fu-Gu-Du-Lo-Ju-So-So-",
            "test 2 9"
        );

        let sys = NumeralSystem::new(
            "",
            vec![
                vec![
                    "B", "K", "D", "F", "G", "J", "L", "M", "N", "P", "R", "S", "T", "V", "X", "Z",
                ],
                vec!["a-", "i-", "o-", "u-"],
            ],
        )
        .unwrap();
        let hex = NumeralSystem::new_from_tag("hex").unwrap();
        let sys_to_hex = BibiCoder::new(sys, hex);
        let test = sys_to_hex
            .swap(
                "Fi-Xa-Du-Do-Ji-Li-Ri-Ro-Mu-Vo-Gu-Vi-Mu-Do-Fi-Pu-Sa-Ni-Mo-Ga-Fu-Gu-Du-Lo-Ju-So-So-",
            )
            .unwrap();

        assert_eq!(
            test, "0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae",
            "test 2 10"
        );
    }

    #[test]
    fn test_autodetect() {
        //let dec = NumeralSystem::new("dec", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
        let hex = NumeralSystem::new_from_tag("hex").unwrap();
        let dec = NumeralSystem::new_from_tag("dec").unwrap();
        let bin = NumeralSystem::new_from_tag("dec").unwrap();

        let test = NumeralSystem::autodetect("0x4324ae34", vec![&hex, &dec]);
        assert!(test.is_some(), "test 3 1");

        let test = NumeralSystem::autodetect("0b0101101", vec![&hex, &dec]);
        assert!(test.is_none(), "test 3 2");

        let test = NumeralSystem::autodetect("0b0101101", vec![&hex, &dec, &bin]);
        assert!(test.is_none(), "test 3 3");
    }
}
