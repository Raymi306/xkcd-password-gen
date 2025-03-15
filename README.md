# xkcd-password-gen

## Inspiration

https://xkcd.com/936/
https://www.xkpasswd.net/
https://metacpan.org/pod/Crypt::HSXKPasswd

## TODO

- statistics (entropy)
- wordlists
- gui
- more automated tests
- CI?
- releases w/ signed binaries
- license
- long help or manpage
- logging?

## Examples

```
Usage: target/debug/xkcd-password-gen [options]

Options:
    -h, --help
    -c, --count NUM, default=4
                        how many passwords to make
    -w, --word-count NUM, default=4
                        number of words
        --word-min-length NUM, default=4
                        minimum length of a chosen word
        --word-max-length NUM, default=11
                        maximum length of a chosen word
    -W, --word-transformation TYPE, default=alternating-lower-upper
                        transformation to apply to the selected words
        --digits-before NUM, default=2
                        number of digits to prepend
        --digits-after NUM, default=2
                        number of digits to append
        --padding-type TYPE, default=fixed
                        how to pad
        --padding-length NUM, default=2
                        how much to pad
        --padding-character CHOICES, default="!@$%^&*-_+=:|~?/.;"
                        list of characters to choose from
        --separator CHOICES, default="!@$%^&*-_+=:|~?/.;"
                        list of characters to choose from

types are case insensitive

WORD TRANSFORMATIONS:
    lower
    upper
    capitalize-first
    capitalize-last
    capitalize-not-first
    alternating-lower-upper
    alternating-upper-lower
    random-upper-lower

PADDING TYPES:
    fixed
    adaptive
```

```
$ ./target/debug/xkcd-password-gen -c 3
$$34?labor?MODERN?deep?WATER?15$$
^^83~hello~WATER~world~LABOR~58^^
!!70%deep%LABOR%hello%WATER%46!!
```
