# Benchmarks - [b08a313](https://github.com/Raymi306/xkcd-password-gen/tree/b08a313bfed1113cc140ebbfe9a050df7abfe8bb)

## hyperfine

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/xkcd-password-gen -c 255 > /dev/null` | 56.7 ± 8.0 | 45.4 | 81.2 | 1.11 ± 0.23 |
| `target/small/xkcd-password-gen -c 255 > /dev/null` | 50.9 ± 7.8 | 40.3 | 69.7 | 1.00 |

## Binary Sizes

- 795K release
- 755K small

## Wordlist Sizes

- 61K eff_large_wordlist.txt
