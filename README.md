
A crate and an app to convert any integer from one numeral system to another.

Two numeral systems must be defined : one for the input number and one for the output.
Any integer (of any length) can then be converted from one system to the other and vice-versa.
This library uses an extension of double dabble algorithm (and reverse double dabble) to convert numbers. Binary is used as a pivot radix.
It was named after french singer (and also mathematician) [Boby Lapointe](https://en.wikipedia.org/wiki/Boby_Lapointe) who invented the [Bibi-binary system](https://en.wikipedia.org/wiki/Bibi-binary) in 1968.

### Example : code using crate

```rust
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
```

### Example : using application

```bash
       $ bibicode 1111111111111111 -f bin -t hex
       ffff

       $ bibicode ffff -t bin -f hex
       1111111111111111

       $ bibicode ffffffffffffffffffffffffffffffff -t dec -f hex
       340282366920938463463374607431768211455

       $ bibicode 340282366920938463463374607431768211455 -f dec -t hex
       ffffffffffffffffffffffffffffffff

       $ cat > /tmp/bibi.txt
       H
       B
       K
       D
       ===
       A
       E
       I
       O

       $ bibicode 340282366920938463463374607431768211455 -f dec -t /tmp/bibi.txt
       DODODODODODODODODODODODODODODODODODODODODODODODODODODODODODODODO

       $ cat > /tmp/bibi2.txt
       H
       B
       K
       D
       ===
       A.
       E.
       I.
       O.

       $ bibicode 5454366920938463463375407431768211455 -f dec -t /tmp/bibi2.txt
       BA.HE.KI.BO.KE.HA.DE.HO.KE.DO.DI.DE.KE.KA.BA.KA.HE.DO.BI.DE.DA.BI.HA.KA.HO.DE.HE.DO.DO.DO.DO.

       $ bibicode 5454366920938463463375407431768211455 -f dec -t utf8
       ■♢♥♢♥♤♣♧★○■☆♠◇♥♧♠♡♣♤♠⚐♦♡■⚐♥♤◀⚐♣◇●◁◀♡♦♢
```



