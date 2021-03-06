binaryornot
===========

[![Build Status](https://travis-ci.org/keirlawson/binaryornot.svg?branch=master)](https://travis-ci.org/keirlawson/binaryornot)
[![Latest version](https://img.shields.io/crates/v/binaryornot.svg)](https://crates.io/crates/binaryornot)
[![Documentation](https://docs.rs/binaryornot/badge.svg)](https://docs.rs/binaryornot)

A Rust port of [binaryornot](https://github.com/audreyr/binaryornot), 
letting you detect whether a file is binary or text.

## Usage

Add the following to your `cargo.toml`:

```toml
[dependencies]
binaryornot = "1.0"
```

```rust
use binaryornot;

if binaryornot::is_binary("/path/to/some/file").expect("unable to read file") {
    println!("a binary!");
}
```

## Limitations

Due to relying on the encoding crate for detecting different possible 
text encodings, UTF-32 will not be correctly detected as text, unlike 
in the original Python binaryornot implementation.

## Licence
As indicated in the cargo.toml, this library can be consumed either under 
the MIT or Apache 2.0 licences.  However, it should be noted that one of 
its dependencies, chardet, is licenced under the LGPL, meaning that any 
program statically linking it must also be distributed under the LGPL.