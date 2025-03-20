# xkcd-password-gen

[xkcd 936](https://xkcd.com/936/)
![xkcd 936 image](https://imgs.xkcd.com/comics/password_strength_2x.png)

## Inspiration

- https://www.xkpasswd.net/
- https://metacpan.org/pod/Crypt::HSXKPasswd

If you need a memorable password quick, I have used [www.xkpasswd.net](https://www.xkpasswd.net) for some time now.
I decided I wanted to write my own take on their password generator that I can compile to a small binary.

## Resources

Wordlist courtesy of the Electronic Frontier Foundation
- https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt

## Dependencies

- rand = "0.9" [(docs)](https://docs.rs/rand/latest/rand/)
- getopts = "0.2" [(docs)](https://docs.rs/getopts/latest/getopts/)

## TODO

### Features

- config files
- presets
- statistics (entropy)
- custom wordlists
- feature flag to not include default wordlist
- gui
- long help or manpage
- choice between ThreadRng and OSRng
- change RNG default to OSRng
- explore reducing binary size more in "small" profile
- symmetrical padding option, eg `*#$[PASSWORD]$#*`

### Housekeeping

- documentation and comments
- more automated tests
- CI
- releases w/ signed binaries
- license
- logging?
- macros
- start updating version

## Checklist Before Release

- [ ] README.md up-to-date?
- [ ] Cargo.toml package version bumped?

### Items for CI to worry about

- [ ] `cargo fmt` + `cargo clippy`
- [ ] `cargo test`
- [ ] `./run_bench.sh`

## Examples ( as of [b08a313](https://github.com/Raymi306/xkcd-password-gen/tree/b08a313bfed1113cc140ebbfe9a050df7abfe8bb) )

```
Usage: xkcd-password-gen [options]

Options:
    -h, --help
    -c, --count NUM, default=1
                        how many passwords to make
    -w, --word-count NUM, default=4
                        number of words
    -m, --word-min-length NUM, default=4
                        minimum length of a chosen word
    -M, --word-max-length NUM, default=11
                        maximum length of a chosen word
    -W, --word-transformation TYPE, default=alternating-lower-upper
                        transformation to apply to the selected words
    -b, --digits-before NUM, default=2
                        number of digits to prepend
    -a, --digits-after NUM, default=2
                        number of digits to append
    -T, --padding-type TYPE, default=fixed
                        how to pad
    -l, --padding-length NUM, default=2 for fixed, 42 for adaptive
                        how much to pad
    -p, --padding-character CHOICES, default="!@$%^&*-_+=:|~?/.;"
                        list of characters to choose from
    -s, --separator CHOICES, default="!@$%^&*-_+=:|~?/.;"
                        list of characters to choose from

types are case insensitive

WORD TRANSFORMATIONS:
    none
    lower                   (correct horse battery staple)
    upper                   (CORRECT HORSE BATTERY STAPLE)
    capitalize-first        (Correct Horse Battery Staple)
    capitalize-last         (correcT horsE batterY staplE)
    capitalize-not-first    (cORRECT hORSE bATTERY sTAPLE)
    alternating-lower-upper (correct HORSE battery STAPLE)
    alternating-upper-lower (CORRECT horse BATTERY staple)
    random-upper-lower      (correct HORSE battery staple)

PADDING TYPES:
    none
    fixed    (add padding-length padding-characters to front and back)
    adaptive (if unpadded password is less than padding-length, pad to length)
```

```
$ xkcd-password-gen -c 3
||47@amicably@JUDGE@enlarged@DECK@11||
~~12_satisfied_KINSHIP_purebred_ESSAY_70~~
__24?expanse?PAYCHECK?naturist?STEADIER?08__
```

## [Benchmarks](benchmarks)

Kind of silly to measure time given that this isn't an application where performance is critical, but still fun.

Binary size is more relevant.

### Tooling

[hyperfine](https://github.com/sharkdp/hyperfine)
