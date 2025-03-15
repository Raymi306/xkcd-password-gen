# Benchmarks - bb2edf4

## hyperfine

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/xkcd-password-gen -c 255 > /dev/null` | 56.9 ± 12.5 | 46.3 | 134.6 | 1.15 ± 0.30 |
| `target/small/xkcd-password-gen -c 255 > /dev/null` | 49.3 ± 7.2 | 41.1 | 69.9 | 1.00 |

## Binary Sizes

- 795K release
- 755K small

## Wordlist Sizes

- 61K eff_large_wordlist.txt
