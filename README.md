
A crate to convert any integer from one numeral system to another.

Two numeral systems must be defined : one for the input number and one for the output.
Any integer (of any length) can then be converted from one system to the other and vice-versa.
This library uses an extension of double dabble algorithm (and reverse double dabble) to convert numbers. Binary is used as a pivot radix.
It was named after french singer (and also mathematician) [Boby Lapointe](https://en.wikipedia.org/wiki/Boby_Lapointe) who invented the [Bibi-binary system](https://en.wikipedia.org/wiki/Bibi-binary) in 1968.
## Exemple
       extern crate bibicode;
       let dec = bibicode::NumeralSystem::new(vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
       let bibi = bibicode::NumeralSystem::new(vec!(vec!("HO", "HA", "HE", "HI", "BO", "BA", "BE", "BI", "KO", "KA", "KE", "KI", "DO", "DA", "DE", "DI"))).unwrap();
       let coder = bibicode::BibiCoder::new(dec, bibi);
       let test = coder.swap("2000").unwrap();
       assert_eq!(test, "BIDAHO");
       let bibi = bibicode::NumeralSystem::new(vec!(vec!("H", "B", "K", "D"), vec!("O", "A", "E", "I"))).unwrap();
       let dec = bibicode::NumeralSystem::new(vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
       let coder = bibicode::BibiCoder::new(dec, bibi);
       let test = coder.swap("2000").unwrap();
       assert_eq!(test, "BIDAHO");
