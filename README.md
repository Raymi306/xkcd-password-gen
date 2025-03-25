# xkcd-password-gen

![cargo-audit](https://github.com/Raymi306/xkcd-password-gen/actions/workflows/audit.yml/badge.svg)

[![xkcd 936 image](https://imgs.xkcd.com/comics/password_strength_2x.png)](https://xkcd.com/936/)

## Inspiration

- https://www.xkpasswd.net/
- https://metacpan.org/pod/Crypt::HSXKPasswd

If you need a memorable password quick, I have used [www.xkpasswd.net](https://www.xkpasswd.net) for some time now.
I decided I wanted to write my own take on their password generator that I can compile to a small binary.

## Resources

Wordlist courtesy of the Electronic Frontier Foundation
- https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt

## Examples

```
Usage: target/debug/xkcd-password-gen [options]

Options:
    -h, --help          
    -c, --count NUM, default=1
                        how many passwords to make
    -w, --word-count NUM, default=4
                        number of words
    -m, --word-min-length NUM, default=3
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
                        how to apply padding
    -l, --padding-length NUM, default=2 for fixed, 42 for adaptive
                        how much to pad
    -p, --padding-characters CHOICES, default="!@$%^&*-_+=:|~?/.;"
                        list of characters to choose from
    -s, --separators CHOICES, default="!@$%^&*-_+=:|~?/.;"
                        list of characters to choose from
    -r, --rng TYPE, default=os-rng
                        method of random number generation

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
    adaptive (if unpadded password is less than padding-length, append padding-characters to meet length)

RNG TYPES:
    os-rng (the system's native secure RNG)
    csprng (a reasonably secure userspace RNG)
```

```
$ xkcd-password-gen -c 3
||47@amicably@JUDGE@enlarged@DECK@11||
~~12_satisfied_KINSHIP_purebred_ESSAY_70~~
__24?expanse?PAYCHECK?naturist?STEADIER?08__
```

## Installation

Download the appropriate release for your operating system, or build from scratch.
With the archive in a directory next to the checksum, `sha256sum -c $CHECKSUM_FILE` can be used to easily validate the archive has not been corrupted.

### Build

1. https://www.rust-lang.org/tools/install
2. `cargo build --profile small`

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
- gui feature flag
- short/long help or manpage
- explore reducing binary size more in "small" profile
- symmetrical padding option, eg `*#$[PASSWORD]$#*`
- additional CSPRNG options under feature flags?
- dice RNG feature flag

### Housekeeping

- test coverage
- a lil more macro magic
- make own UnwrapErr for TryRngCore
- edit `run_bench.sh` to allow for saving version info

## Checklist Before Release

- [ ] README.md up-to-date?
- [ ] Cargo.toml package version bumped?
- [ ] `./run_bench.sh`?
## [Benchmarks](benchmarks)

Kind of silly to measure time given that this isn't an application where performance is critical, but still fun.

Binary size is more relevant.

### Tooling

[hyperfine](https://github.com/sharkdp/hyperfine)
