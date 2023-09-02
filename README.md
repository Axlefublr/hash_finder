# hash_finder

Find hashes that end in your amount of trailing zeros!

## Usage
```
Usage: hash_finder -N <ZEROS> -F <RESULTS>

Options:
  -N <ZEROS>        The amount of consecutive zeros at the end of the hash
  -F <RESULTS>      The amount of results you want to print to stdout
  -h, --help        Print help
  -V, --version     Print version
```

## Installation
```
cargo install --git https://github.com/Axlefublr/hash_finder
```

## Fun trivia

In some cases hashes are compared not as an array of characters (a string), but as an array of bytes.

In an array of bytes, a 0 is a null byte, which, in C, will be interpreted as "stop comparing, this is the end of the string"

What this ends up meaning is that once you encounter a zero, all the other numbers to the left stop mattering, as they will match regardless because of the "null byte"
