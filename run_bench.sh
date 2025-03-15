#!/bin/sh

COMMIT="$(git rev-parse --verify --short HEAD)"

cargo build --profile release
cargo build --profile small

hyperfine --export-markdown benchmarks/hyperfine_$COMMIT.md --warmup 3 'target/release/xkcd-password-gen -c 255 > /dev/null' 'target/small/xkcd-password-gen -c 255 > /dev/null'

echo "# Benchmarks - $COMMIT\n" > benchmarks/README.md
echo "## hyperfine\n" >> benchmarks/README.md

cat benchmarks/hyperfine_$COMMIT.md >> benchmarks/README.md

echo "\n## Binary Sizes\n" >> benchmarks/README.md
echo "- $(stat -c %s target/release/xkcd-password-gen | numfmt --to=iec) release" >> benchmarks/README.md
echo "- $(stat -c %s target/small/xkcd-password-gen | numfmt --to=iec) small" >> benchmarks/README.md
echo "\n## Wordlist Sizes\n" >> benchmarks/README.md
echo "- $(stat -c %s wordlists/eff_large_wordlist.txt | numfmt --to=iec) eff_large_wordlist.txt" >> benchmarks/README.md
