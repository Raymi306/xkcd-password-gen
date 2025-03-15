# xkcd-password-gen

## Inspiration

- https://xkcd.com/936/
- https://www.xkpasswd.net/
- https://metacpan.org/pod/Crypt::HSXKPasswd

I have used xkpasswd several times in the past and enjoyed it.
I wanted to make my own version.
I am sure the Perl and JavaScript flavors do the job fine, but I wanted a native option.
I haven't been looking at the original source, but I have been reading the documentation and playing with the website.

## Resources

[The wordlist is courtesy of the Electronic Frontier Foundation](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt)

## TODO

### Features

- config files
- presets
- statistics (entropy)
- custom wordlists
- feature flag to not include default wordlist
- gui
- long help or manpage

### Housekeeping

- documentation
- more automated tests
- CI
- releases w/ signed binaries
- license
- logging?
- macros
- examples in help (correct horse battery staple)

## Examples

```
Usage: target/debug/xkcd-password-gen [options]

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
    -l, --padding-length NUM, default=2
                        how much to pad
    -p, --padding-character CHOICES, default="!@$%^&*-_+=:|~?/.;"
                        list of characters to choose from
    -s, --separator CHOICES, default="!@$%^&*-_+=:|~?/.;"
                        list of characters to choose from

types are case insensitive

WORD TRANSFORMATIONS:
    lower                   (correct horse battery staple)
    upper                   (CORRECT HORSE BATTERY STAPLE)
    capitalize-first        (Correct Horse Battery Staple)
    capitalize-last         (correcT horsE batterY staplE)
    capitalize-not-first    (cORRECT hORSE bATTERY sTAPLE)
    alternating-lower-upper (correct HORSE battery STAPLE)
    alternating-upper-lower (CORRECT horse BATTERY staple)
    random-upper-lower      (correct HORSE battery staple)

PADDING TYPES:
    fixed    (add padding-length padding-characters to front and back)
    adaptive (if unpadded password is less than padding-length, pad to length)
```

```
$ ./target/debug/xkcd-password-gen -c 3
||47@amicably@JUDGE@enlarged@DECK@11||
~~12_satisfied_KINSHIP_purebred_ESSAY_70~~
__24?expanse?PAYCHECK?naturist?STEADIER?08__
```
