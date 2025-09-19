# bibicode

A crate and an app to convert any natural number from one numeral system to another.

Two numeral systems must be defined :
- one for the input number
- and one for the output.

Any natural number (of any length) can then be converted from one system to the other and vice-versa.

This library uses shift-adjust algorithm (and reversed shift-adjust) to convert numbers. Binary is used as a pivot radix. This method was described here : [Convert binary number to any base](https://www.edn.com/design/systems-design/4460458/Convert-binary-number-to-any-base).

It was named after french singer (and also mathematician) [Boby Lapointe](https://en.wikipedia.org/wiki/Boby_Lapointe) who invented the [Bibi-binary system](s://en.wikipedia.org/wiki/Bibi-binary) in 1968.

The following numeral systems are pre-integrated into the application and can be used :
- dec for decimal
- hex for hexadecimal
- bin for binary
- base58 for base 58 as used in bitcoin address representation
- bibi for bibi-binary as defined by Boby Lapointe, the inspirator of this application
- budu for an experimental numeral system which is well readable


## Example : using crate
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

```shell
$ cargo install
...


$ bibicode 1111111111111111 -f bin -t hex
0xffff

$ bibicode 1111111111111111 100100010100000001 -f bin -t hex
0xffff 0x24501

$ bibicode ffff -t bin -f hex
0b1111111111111111

$ bibicode ffffffffffffffffffffffffffffffff -t dec -f hex
340282366920938463463374607431768211455

$ bibicode 340282366920938463463374607431768211455 -f dec -t hex
0xffffffffffffffffffffffffffffffff


```

Numeral System can be represented by a json file :

```shell
$ cat ./examples/bibi.json
{
    "digits":["HO", "HA", "HE", "HI", "BO", "BA", "BE", "BI", "KO",
        "KA", "KE", "KI", "DO", "DA", "DE", "DI"]
}

$ bibicode 340282366920938463463374607431768211455 -f dec -t ./examples/bibi.json
DIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDIDI

$ bibicode 5454366920938463463375407431768211455 -f dec -t ./examples/bibi.json
BOHAKEBIKAHODAHIKADIDEDAKAKOBOKOHADIBEDADOBEHOKOHIDAHADIDIDIDI

$ cat ./examples/budu.json
{
    "digits": [["B","K","D","F","G","J","L","M","N","P","R","S","T","V","X","Z"], ["a", "i","o","u"]]
}

$ bibicode 5454366920938463463375407431768211455 -f dec -t ./examples/budu.json
KaKoPuPaFiFoMuXuLiNiDaKuVoVuKoBoBuVaMuZuZu
```

A prefix can be used to tag the output number :

```shell
$ cat ./examples/hex.json
{
    "prefix":"0x",
    "digits":["0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"]
}

$ bibicode 340282366920938463463374607431768211455 -f dec -t ./examples/hex.json
0xffffffffffffffffffffffffffffffff
```

Numeral systems json files can be stored into the XDG directory linked with the application. If so, the numeral system is directly known by bibicode after the name of the json file :

```shell
$ cat ~/.local/share/bibicode/budu2.json
{
    "digits": [["B","K","D","F","G","J","L","M","N","P","R","S","T","V","X","Z"], ["a", "i","o","u"]]
}

$ bibicode 5454366920938463463375407431768211455 -f dec -t budu2
KaKoPuPaFiFoMuXuLiNiDaKuVoVuKoBoBuVaMuZuZu
```

By default, decimal system will be used if none is given. However, bibicode will try to guess the numeral system from the prefix of the first input number.

```shell
$ bibicode 5454366920938463463375407431768211455 -f dec -t hex
0x41a790d39fed98481f6dc6083d1ffff

$ # use default dec as input :
$ bibicode 5454366920938463463375407431768211455 -t hex
0x41a790d39fed98481f6dc6083d1ffff

$ cat ~/.local/share/bibicode/budu.json
{
    "prefix": "budu-",
    "digits": [["B","K","D","F","G","J","L","M","N","P","R","S","T","V","X","Z"], ["a", "i","o","u"]]
}

$ # will use budu as input (because of the prefix) and default dec as output
$ bibicode budu-KaKoPuPaFiFoMuXuLiNiDaKuVoVuKoBoBuVaMuZuZu
5454366920938463463375407431768211455

$ bibicode 5454366920938463463375407431768211455 -t budu
budu-KaKoPuPaFiFoMuXuLiNiDaKuVoVuKoBoBuVaMuZuZu
```

Concat option can be used to concat output numbers if several numbers are present on the entry :

```shell
$ bibicode 324443242 76575432423 -f dec -t hex
0x13569c6a 0x11d4405ee7
$ bibicode 324443242 76575432423 -f dec -t hex --concat
0x13569c6a11d4405ee7
```

Regex can be given to bibicode to describe the format of the number.

Example converting a bitcoin address to decimal (first byte is version number) :
```shell
$ cat ./examples/base58.json
{
    "digits": ["1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "J", "K", "L", "M", "N", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]
}
$ bibicode 17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem -f ./examples/base58.json -t dec --regex "^(\\d{1})([1-9A-HJ-NP-Za-km-z]*)$"
0 1746223458145969276700901827421233769953772985502309560518
```

Use --concat option to concat the output :
```shell
$ bibicode 17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem -f ./examples/base58.json -t dec --regex "^(\\d{1})([1-9A-HJ-NP-Za-km-z]*)$" --concat
01746223458145969276700901827421233769953772985502309560518
```

And revert operation :
```shell
$ bibicode 01746223458145969276700901827421233769953772985502309560518 -f dec -t ./examples/base58.json  --regex "^(\\d{1})(\\d{58})$"
17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem
```

General prefix, separator and suffix can be given directly to the output of bibicode.

Example swapping HTML RGB colors with hexadecimal :
```shell
$ cat ./examples/hexcol.json
{
    "prefix":"#",
    "digits":["0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"]
}

$ bibicode "#e135ad" -f ./examples/hexcol.json -t dec -s "," -p "rgb(" -x ")" -r "([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})"
rgb(225,53,173)

$ bibicode "rgb(225,53,173)" -f dec -t ./examples/hexcol.json -r "^rgb\\((\\d{1,3}),(\\d{1,3}),(\\d{1,3})\\)$" --concat
#e135ad
```


