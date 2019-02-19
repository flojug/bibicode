# bibicode

A crate and an app to convert any integer from one numeral system to another.

Two numeral systems must be defined :
- one for the input number
- and one for the output.

Any integer (of any length) can then be converted from one system to the other and vice-versa.

This library uses shift-adjust algorithm (and reversed shift-adjust) to convert numbers. Binary is used as a pivot radix. This method was described here : [Convert binary number to any base](https://www.edn.com/design/systems-design/4460458/Convert-binary-number-to-any-base).

It was named after french singer (and also mathematician) [Boby Lapointe](https://en.wikipedia.org/wiki/Boby_Lapointe) who invented the [Bibi-binary system](s://en.wikipedia.org/wiki/Bibi-binary) in 1968.

## Example
```rust
extern crate bibicode;

let dec = bibicode::NumeralSystem::new("", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
let bibi = bibicode::NumeralSystem::new("", vec!(vec!("HO", "HA", "HE", "HI", "BO", "BA", "BE", "BI", "KO", "KA", "KE", "KI", "DO", "DA", "DE", ))).unwrap();
let coder = bibicode::BibiCoder::new(dec, bibi);
let test = coder.swap("2000").unwrap();
assert_eq!(test, "BIDAHO");

let bibi = bibicode::NumeralSystem::new("", vec!(vec!("H", "B", "K", "D"), vec!("O", "A", "E", "I"))).unwrap();
let dec = bibicode::NumeralSystem::new("", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
let coder = bibicode::BibiCoder::new(dec, bibi);
let test = coder.swap("2000").unwrap();
assert_eq!(test, "BIDAHO");

// with prefixed numeral system
let dec = bibicode::NumeralSystem::new("", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
let hex = bibicode::NumeralSystem::new("0x", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"))).unwrap();
let coder = bibicode::BibiCoder::new(dec, hex);
let test = coder.swap("2000").unwrap();
assert_eq!(test, "0x7d0");

let dec = bibicode::NumeralSystem::new("", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9"))).unwrap();
let hex = bibicode::NumeralSystem::new("0x", vec!(vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"))).unwrap();
let coder = bibicode::BibiCoder::new(hex, dec);
let test = coder.swap("0x7d0").unwrap();
assert_eq!(test, "2000");

// will also work
let test = coder.swap("7d0").unwrap();
assert_eq!(test, "2000");
```

## Example : using application

Use `cargo run` to build and run the application or `cargo build` to build it in debug mode.

```shell
$ cargo build
...

$ ./target/debug/bibicode 1111111111111111 -f bin -t hex
0xffff

$ ./target/debug/bibicode ffff -t bin -f hex
0b1111111111111111

$ ./target/debug/bibicode ffffffffffffffffffffffffffffffff -t dec -f hex
340282366920938463463374607431768211455

$ ./target/debug/bibicode 340282366920938463463374607431768211455 -f dec -t hex
0xffffffffffffffffffffffffffffffff

$ cat ./examples/bibi.json
{
    "prefix":"",
    "digits":["HO", "HA", "HE", "HI", "BO", "BA", "BE", "BI", "KO", "KA", "KE", "KI", "DO", "DA", "DE", "DI"]
}


$ ./target/debug/bibicode 340282366920938463463374607431768211455 -f dec -t ./examples/bibi.json
DIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDI

$ ./target/debug/bibicode 5454366920938463463375407431768211455 -f dec -t ./examples/bibi.json
BOHAKEBIKAHODAHIKADIDEDAKAKOBOKOHADIBEDADOBEHOKOHIDAHADIDIDIDI

$ cat ./examples/budu.json
{
    "prefix":"",
    "digits":["B","K","D","F","G","J","L","M","N","P","R","S","T","V","X","Z"], ["a", "i","o","u"]
}

$ ./target/debug/bibicode 5454366920938463463375407431768211455 -f dec -t ./examples/budu.json
KaKoPuPaFiFoMuXuLiNiDaKuVoVuKoBoBuVaMuZuZu

$ ./target/debug/bibicode 5454366920938463463375407431768211455 -f dec -t utf8
■♢♥♢♥♤♣♧★○■☆♠◇♥♧♠♡♣♤♠⚐♦♡■⚐♥♤◀⚐♣◇●◁◀♡♦♢

$ ./target/debug/bibicode 5454366920938463463375407431768211455 -f dec -t chin
㑈㹨㕫㕃㽃㹼㷶㓱㸿㰪㯿
```


