#!/bin/sh

COMMIT="$(git rev-parse --verify HEAD)"
VERIFY_EXIT=$?

if [ $VERIFY_EXIT -ne 0 ]; then
    echo "git rev-parse --verify HEAD failed"
    exit $VERIFY_EXIT
fi

SHORTCOMMIT="$(git rev-parse --short HEAD)"

cargo build --profile release
cargo build --profile small

TMPFILE="$(basename $0)".md.tmp
hyperfine --export-markdown $TMPFILE --warmup 3 'target/release/xkcd-password-gen -c 255 > /dev/null' 'target/small/xkcd-password-gen -c 255 > /dev/null'

echo "# Benchmarks - [$SHORTCOMMIT](https://github.com/Raymi306/xkcd-password-gen/tree/$COMMIT)\n" > benchmarks/README.md
echo "## hyperfine\n" >> benchmarks/README.md

cat $TMPFILE >> benchmarks/README.md
rm $TMPFILE

echo "\n## Binary Sizes\n" >> benchmarks/README.md
echo "- $(stat -c %s target/release/xkcd-password-gen | numfmt --to=iec) release" >> benchmarks/README.md
echo "- $(stat -c %s target/small/xkcd-password-gen | numfmt --to=iec) small" >> benchmarks/README.md
echo "\n## Wordlist Sizes\n" >> benchmarks/README.md
echo "- $(stat -c %s wordlists/eff_large_wordlist.txt | numfmt --to=iec) eff_large_wordlist.txt" >> benchmarks/README.md

cp benchmarks/README.md benchmarks/$SHORTCOMMIT.md
