
// Copyright ⓒ 2019 Florent Jugla
//
// Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//
//
// ! # Bibicode
//!
//! This crate can be used to convert any integer from one numeral system to another.
//!
//! Two numeral systems must be defined : one for the input number and one for the output.
//!
//! Any integer (of any length) can then be converted from one system to the other and vice-versa.
//!
//! This library uses an extension of double dabble algorithm (and reverse double dabble) to convert numbers. Binary is used as a pivot radix. This method was described here : [Convert binary number to any base](https://www.edn.com/design/systems-design/4460458/Convert-binary-number-to-any-base).
//!
//! It was named after french singer (and also mathematician) [Boby Lapointe](https://en.wikipedia.org/wiki/Boby_Lapointe) who invented the [Bibi-binary system](https://en.wikipedia.org/wiki/Bibi-binary) in 1968.
//!
//! ## Exemple
//!
//!        extern crate bibicode;
//!
//!        let dec = bibicode::NumeralSystem::new("", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
//!        let bibi = bibicode::NumeralSystem::new("", vec!(vec!("HO", "HA", "HE", "HI", "BO", "BA", "BE", "BI", "KO", "KA", "KE", "KI", "DO", "DA", "DE", "DI"))).unwrap();
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




use std::fmt;
use std::f64;

extern crate serde_derive;
use serde_derive::{Serialize, Deserialize};


#[derive(Debug)]
pub enum BibiError {
    /// Malformed numeral system : all digits must have the same length and be unique
    BadNumeralSystem,
    /// One digit given in the entry was not found in numeral system
    EntryMismatchWithNumeralSystem,
    /// Non existent pre-defined numeral system
    BadTagNumeralSystem,
}


/// Define a numeral system by enumerating all the digits. The first digit is zero. The radix is equal to the number of digits. One digit can have any number of characters but all digits must have the same length.
#[derive(Serialize, Deserialize, Debug)]
pub struct NumeralSystem {
    prefix: String,
    len_digit: usize,
    digits: Vec<String>,
}

impl NumeralSystem {

    // static method to find out a numeral system by its prefix given a number
    pub fn autodetect<'a>(number: &str, nums: Vec<&'a NumeralSystem>) -> Option<&'a NumeralSystem> {
        let res: Vec<&'a NumeralSystem> = nums.into_iter().filter(|ns| (ns.prefix.len()>0)&&(ns.prefix[..]==number[0..ns.prefix.len()]) ).collect();
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
     pub fn new_from_strings(prefix: String, entry: Vec<Vec<String>>) -> Result<NumeralSystem, BibiError> {
        let entry_str: Vec<Vec<&str>> = entry.iter().map(|v| v.iter().map(|s| &**s).collect()).collect();
        NumeralSystem::new(&prefix[..], entry_str)
    }

    // internal method to build system from vec of vec
    fn new_rec(prefix: &str, entry: &Vec<Vec<&str>>, index: usize) -> Result<NumeralSystem, BibiError> {

        let len_digit;
        let mut digits: Vec<String> = vec!();

        let first_entry = &entry[index];

        match first_entry.first() {
            None => return Err(BibiError::BadNumeralSystem),
            Some(digit) => len_digit = digit.len()
        }

        if len_digit==0 {
            return Err(BibiError::BadNumeralSystem);
        }

        let mut sub_num_sys = NumeralSystem { prefix:String::from(prefix), len_digit: 0, digits: vec!() };
        if index < entry.len()-1 {
            sub_num_sys = NumeralSystem::new_rec(prefix, entry, index+1)?;
        }

        for digit in first_entry {
            if digit.len() != len_digit {
                return Err(BibiError::BadNumeralSystem);
            }

            if sub_num_sys.len_digit > 0 {
                for digit2 in &sub_num_sys.digits[..] {
                    digits.push(String::from(*digit) + &String::from(&digit2[..]));
                }
            } else {
                digits.push(String::from(*digit));
            }
        }

        // check for unique digits
        let mut compdigs = digits.clone();
        compdigs.sort();
        let len1 = compdigs.len();
        compdigs.dedup();
        let len2 = compdigs.len();
        if len1 != len2 {
            return Err(BibiError::BadNumeralSystem);
        }

        Ok(NumeralSystem { prefix: String::from(prefix), len_digit: len_digit + sub_num_sys.len_digit, digits: digits })
    }


    /// Returns pre-defined numeral systems :
    /// - dec for decimal
    /// - hex for hexadecimal
    /// - bibi for "bibi" as defined by Boby Lapointe
    /// - bin for binary
    /// - budu for a test system easy to read
    /// - utf8 for a test system with UTF8 characters
    /// - base58 for base58 as used in bitcoin
    pub fn new_from_tag(tag: &str) -> Result<NumeralSystem, BibiError> {
        match tag {
            "bin" => NumeralSystem::new("0b", vec!(vec!("0", "1"))),
            "oct" => NumeralSystem::new("0o", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7"))),
            "dec" => NumeralSystem::new("", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))),
            "hex" => NumeralSystem::new("0x", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"))),
            "bibi" => NumeralSystem::new("", vec!(vec!("HO", "HA", "HE", "HI", "BO", "BA", "BE", "BI", "KO", "KA", "KE", "KI", "DO", "DA", "DE", "DI"))),
            "budu" => NumeralSystem::new("", vec!(vec!("B","K","D","F","G","J","L","M","N","P","R","S","T","V","X","Z"), vec!("a", "i","o","u") )),
            "utf8" => NumeralSystem::new("", vec!(vec!("\u{25a0}", "\u{25c0}", "\u{25cf}", "\u{2660}", "\u{2665}", "\u{2666}", "\u{2663}", "\u{2691}", "\u{25c6}", "\u{2605}"), vec!("\u{25a1}", "\u{25c1}", "\u{25cb}", "\u{2664}", "\u{2661}", "\u{2662}", "\u{2667}", "\u{2690}", "\u{25c7}", "\u{2606}"))),
            "base58" => NumeralSystem::new("", vec!(vec!("1","2","3","4","5","6","7","8","9","A","B","C","D","E","F","G","H","J","K","L","M","N","P","Q","R","S","T","U","V","W","X","Y","Z","a","b","c","d","e","f","g","h","i","j","k","m","n","o","p","q","r","s","t","u","v","w","x","y","z"))),
            _ => Err(BibiError::BadTagNumeralSystem),
        }
    }

    /// Returns the digit at the position usize
    pub fn get_digit(&self, which: usize) -> Option<String> {
        if which < self.digits.len() {
            return Some(self.digits[which].clone());
        }
        None
    }

    /// Returns the legth of a digit (all digits have the same length)
    pub fn len(&self) -> usize { self.digits.len() }

    /// Return the radix of this numeral system (= number of digits in numeral system)
    pub fn radix(&self) -> usize { self.len() }
}

impl fmt::Display for NumeralSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp = String::from("");
        let mut sep = String::from("");
        for digit in self.digits.iter() {
            disp = disp + &sep + &digit;
            sep = String::from(", ");
        }
        write!(f, "{}", disp)
    }
}


#[derive(Debug)]
struct BCDLike {
    len_bits: usize,
    len_nibble: usize,
    nibbles: Vec<bool>
}

impl BCDLike {
    fn new(len: usize) -> BCDLike {
        BCDLike { len_nibble: len, nibbles: vec!(), len_bits: 0 }
    }

    // push most significant bit
    fn push_msb(&mut self, new_bit: bool) {
        self.nibbles.insert(0, new_bit);
        self.len_bits = self.len_bits + 1;
    }

    // push least significant bit
    fn push_lsb(&mut self, new_bit: bool) {
        self.nibbles.push(new_bit);
        self.len_bits = self.len_bits + 1;
    }

    fn shift_lsb(&mut self) -> bool {
        self.len_bits = self.len_bits - 1;
        self.nibbles.pop().unwrap()
    }

    fn shift_msb(&mut self) -> bool {
        self.len_bits = self.len_bits - 1;
        self.nibbles.remove(0)
    }

    fn get_nb_nibbles(&self) -> usize {
        let rel = if self.len_bits % self.len_nibble > 0 {1} else {0};
        (self.len_bits / self.len_nibble) + rel
    }

    fn add_nibble(&mut self, which: usize) {
        let nb_bits = (which + 1) * self.len_nibble;
        if nb_bits > self.len_bits {
            for _ in self.len_bits..nb_bits {
                self.push_lsb(false);
            }
        }
    }

    fn set_val_to_nibble(&mut self, which: usize, val: u32, extend: bool) {
        if extend {
            self.add_nibble(which);
        }
        let pos = which * self.len_nibble;
        let mut digidx = val;
        for idx in (0..self.len_nibble).rev() {
            if pos+idx < self.len_bits {
                self.nibbles[pos+idx] = if (digidx%2) == 0 {false} else {true};
            }
            digidx = digidx / 2;
        }
    }

    fn get_val_from_nibble(&mut self, which: usize) -> u32 {
        let pos = which * self.len_nibble;
        let mut pow = 1;
        let mut val = 0;
        for idx in (0..self.len_nibble).rev() {
            if pos+idx < self.len_bits {
                val = val + if self.nibbles[pos+idx] {pow} else {0};
            }
            pow = pow * 2;
        }
        val
    }

    fn adjust(&mut self, ceil: u32, inc: u32) {
        let rel = self.len_nibble - (self.len_bits % self.len_nibble);
        for _ in 0..rel {
            self.push_msb(false);
        }
        for idx in (0..self.get_nb_nibbles()).rev() {
            let mut val = self.get_val_from_nibble(idx);
            if val >= ceil {
                val = val + inc;
                self.set_val_to_nibble(idx, val, false);
            }
        }
        for _ in 0..rel {
            let val = self.shift_msb();
            if val {
                self.push_msb(true);
                break;
            }
        }
    }

    fn rev_adjust(&mut self, ceil: u32, inc: u32) {
        let rel = self.len_nibble - (self.len_bits % self.len_nibble);
        for _ in 0..rel {
            self.push_msb(false);
        }
        for idx in 0..self.get_nb_nibbles() {
            let mut val = self.get_val_from_nibble(idx);
            if val >= ceil + inc {
                val = val - inc;
                self.set_val_to_nibble(idx, val, false);
            }
        }
        for _ in 0..rel {
            self.shift_msb();
        }
    }
}

impl fmt::Display for BCDLike {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp = String::from("[");
        let mut print = self.nibbles.clone();
        let rel = self.len_nibble - (self.len_bits % self.len_nibble);
        for _ in 0..rel {
            print.insert(0, false);
        }
        for idx in 0..print.len() {
            if (idx>0) && (idx%self.len_nibble==0) {
                disp = disp + &String::from("][");
            }
            disp = disp + (if print[idx] {"1"} else {"0"});
        }
        disp = disp + &String::from("]");
        write!(f, "{}", disp)
    }
}

#[derive(Debug)]
struct BinaryNumber {
    digits: BCDLike
}

impl BinaryNumber {

    fn new() -> BinaryNumber {
        BinaryNumber { digits: BCDLike::new(1) }
    }

    fn push_msb(&mut self, new_bit: bool) {
        self.digits.push_msb(new_bit)
    }

    fn shift_msb(&mut self) -> bool {
        self.digits.shift_msb()
    }

}

impl fmt::Display for BinaryNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.digits)
    }
}


/// Convert any number from one numeral system to the other.
#[derive(Debug)]
pub struct BibiCoder {
    numsys_in: NumeralSystem,
    numsys_out: NumeralSystem,
}

impl BibiCoder
{
    /// Build a coder from numsys_in numeral system to numsys_out
    pub fn new(numsys_in: NumeralSystem, numsys_out: NumeralSystem) -> BibiCoder  {
        BibiCoder{numsys_in, numsys_out}
    }

    /// Swap an integer coded in numsys_in system to numsys_out
    pub fn swap(&self, entry: &str) -> Result<String, BibiError> {
        let pivot: BinaryNumber = self.elbbad_elduob(entry)?;
        self.double_dabble(pivot)
    }

    // compute BCD like numbers into binary
    fn elbbad_elduob(&self, entry: &str) -> Result<BinaryNumber, BibiError> {

        // erase the prefix if present
        let rel_entry: &str;
        if (self.numsys_in.prefix.len()>0) && (self.numsys_in.prefix[..]==entry[0..self.numsys_in.prefix.len()]) {
            rel_entry = &entry[self.numsys_in.prefix.len()..];
        } else {
            rel_entry = entry;
        }

        let radix = self.numsys_in.len() as u32;
        //println!("radix {:?}", radix);
        let radix_f64 = radix as f64;

        let len_nibble = radix_f64.log2().ceil() as usize;
        //println!("len_nibble {:?}", len_nibble);
        let mut bcd_like = BCDLike::new(len_nibble);

        let mut pivot = BinaryNumber::new();

        // compute nibbles from the entry
        for i in 0..(rel_entry.len()/self.numsys_in.len_digit) {
            let digit = String::from(&rel_entry[i*self.numsys_in.len_digit..(i*self.numsys_in.len_digit)+self.numsys_in.len_digit]);
            if !self.numsys_in.digits.contains(&digit) {
                return Err(BibiError::EntryMismatchWithNumeralSystem);
            }
            let mut digidx: u32;
            match self.numsys_in.digits.iter().position(|digit_in_numsys| &digit == digit_in_numsys) {
                None => return Err(BibiError::EntryMismatchWithNumeralSystem),
                Some(index) => digidx = index as u32,
            }
            bcd_like.set_val_to_nibble(i, digidx, true);
        }
        // compute binary from nibbles
        // reversed double dabble
        let ceil_value = (radix_f64 / 2.0).ceil() as u32;
        let inc = ((2 as u32).pow(len_nibble as u32) - radix) / 2;

        while bcd_like.len_bits > 0 {
            let bit = bcd_like.shift_lsb();
            pivot.push_msb(bit);
            bcd_like.rev_adjust(ceil_value, inc);
        }

        Ok(pivot)
    }

    // compute  binary numbers into BCD like
    fn double_dabble(&self, mut pivot: BinaryNumber) -> Result<String, BibiError> {
        let radix = self.numsys_out.len() as u32;
        let radix_f64 = radix as f64;
        let len_nibble = radix_f64.log2().ceil() as usize;
        let mut bcd_like = BCDLike::new(len_nibble);

        let ceil_value = (radix_f64 / 2.0).ceil() as u32;
        let inc = ((2 as u32).pow(len_nibble as u32) - radix) / 2;
        let len_bits = pivot.digits.len_bits;
        for _ in 0..len_bits {
            bcd_like.adjust(ceil_value, inc);
            let bit = pivot.shift_msb();
            bcd_like.push_lsb(bit);
        }
        let mut ret = self.numsys_out.prefix.clone();
        let mut begin = true;

        let rel = bcd_like.len_nibble - (bcd_like.len_bits % bcd_like.len_nibble);
        for _ in 0..rel {
            bcd_like.push_msb(false);
        }

        for idx in 0..bcd_like.get_nb_nibbles() {
            let val = bcd_like.get_val_from_nibble(idx) as usize;
             if begin && (val==0) {
                continue;
            }
            begin =false;
            ret = ret + &self.numsys_out.get_digit(val).unwrap();
        }

        Ok(ret)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeral_system() {

        let test = NumeralSystem::new("", vec!(vec!("0", "1", "2", "2")));
        assert!(test.is_err(), "Test if double digit in the same entry forbidden.");

        let test = NumeralSystem::new("", vec!(vec!("0", "1", "2", "22")));
        assert!(test.is_err(), "Test if digits with different length forbidden.");

        let test = NumeralSystem::new("", vec!(vec!("0", "1", "2"), vec!("0", "1", "2", "22")));
        assert!(test.is_err(), "Test if digits with different length forbidden.");

        let test = NumeralSystem::new("", vec!(vec!("0", "1", "2"), vec!("0", "1", "2", "2")));
        assert!(test.is_err(), "Test if digits with same length OK");

        let a = vec!("B","K","D","F","G","J","L","M","N","P","R","S","T","V","X","Z");
        let b = vec!("a", "i","o","u");
        let len1 = a.len() * b.len();
        let test = NumeralSystem::new("", vec!(a, b)).unwrap();
        let len2 = test.digits.len();
        assert!(len1==len2, "Test size of numeral system");
    }

    #[test]
    fn test_bibicoder() {

        let dec = NumeralSystem::new("", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
        let bibi = NumeralSystem::new("", vec!(vec!("HO", "HA", "HE", "HI", "BO", "BA", "BE", "BI", "KO", "KA", "KE", "KI", "DO", "DA", "DE", "DI"))).unwrap();

        let dec_to_bibi = BibiCoder::new(dec, bibi);

        let test = dec_to_bibi.swap("2000").unwrap();
        assert_eq!(test, "BIDAHO", "test conversion");

        //let bibi = NumeralSystem::new("", vec!(vec!("H", "B", "K", "D"), vec!("O", "A", "E", "I"))).unwrap();
        let test = dec_to_bibi.swap("2000").unwrap();
        assert_eq!(test, "BIDAHO", "test conversion");

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
        assert_eq!(test, "65535", "test conversion");

        let test = hex_to_bin.swap("f0ff").unwrap();
        assert_eq!(test, "0b1111000011111111", "test conversion");

        let test = dec_to_hex.swap("324439924324324235436544328757654635345424324543").unwrap();
        assert_eq!(test, "0x38d463ad8fa67a74d6e9a610158623c60d2297bf", "test conversion");

        let test = hex_to_dec.swap("38d463ad8fa67a74d6e9a610158623c60d2297bf").unwrap();
        assert_eq!(test, "324439924324324235436544328757654635345424324543", "test conversion");

        let sys = NumeralSystem::new("", vec!(vec!("B","K","D","F","G","J","L","M","N","P","R","S","T","V","X","Z"), vec!("a-", "i-","o-","u-"))).unwrap();

        let hex = NumeralSystem::new_from_tag("hex").unwrap();
        let hex_to_sys = BibiCoder::new(hex, sys);
        let test = hex_to_sys.swap("de0b295669a9fd93d5f28d9ec85e40f4cb697bae").unwrap();
        let test2 = hex_to_sys.swap("0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae").unwrap();

        assert_eq!(test, "Fi-Xa-Du-Do-Ji-Li-Ri-Ro-Mu-Vo-Gu-Vi-Mu-Do-Fi-Pu-Sa-Ni-Mo-Ga-Fu-Gu-Du-Lo-Ju-So-So-");
        assert_eq!(test2, "Fi-Xa-Du-Do-Ji-Li-Ri-Ro-Mu-Vo-Gu-Vi-Mu-Do-Fi-Pu-Sa-Ni-Mo-Ga-Fu-Gu-Du-Lo-Ju-So-So-");

        let sys = NumeralSystem::new("", vec!(vec!("B","K","D","F","G","J","L","M","N","P","R","S","T","V","X","Z"), vec!("a-", "i-","o-","u-"))).unwrap();
        let hex = NumeralSystem::new_from_tag("hex").unwrap();
        let sys_to_hex = BibiCoder::new(sys, hex);
        let test = sys_to_hex.swap("Fi-Xa-Du-Do-Ji-Li-Ri-Ro-Mu-Vo-Gu-Vi-Mu-Do-Fi-Pu-Sa-Ni-Mo-Ga-Fu-Gu-Du-Lo-Ju-So-So-").unwrap();
        assert_eq!(test, "0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae");
    }

    #[test]
    fn test_autodetect() {

        //let dec = NumeralSystem::new("dec", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
        let hex = NumeralSystem::new_from_tag("hex").unwrap();
        let dec = NumeralSystem::new_from_tag("dec").unwrap();
        let bin = NumeralSystem::new_from_tag("dec").unwrap();

        let test = NumeralSystem::autodetect("0x4324ae34", vec!(&hex, &dec));
        assert!(test.is_some());

        let test = NumeralSystem::autodetect("0b0101101", vec!(&hex, &dec));
        assert!(test.is_none());

        let test = NumeralSystem::autodetect("0b0101101", vec!(&hex, &dec, &bin));
        assert!(test.is_none());
    }
}
