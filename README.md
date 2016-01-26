# modelm

A OSX **Mechanical keyboard audio simulator** for your Mac keyboard written in [Rust](https://www.rust-lang.org/). 

> Get yourself clickity-clacking.

## Features
`modelm` hooks into OSX Quartz Events to provide audible feedback for your kestrokes.

* **Positonal sounds** - Keys on the left sound like keys on the left. Keys on the right sound like Keys on the right.
* **Dynamix resource loading** - You can pick your favorite clickity-clacks.  Just point `modelm` to a directory with sound bites.

## Usage

Install [Rust](https://github.com/rust-lang/rustup) and [Cargo](https://crates.io/).

```
git clone git@github.com:millerjs/modelm.git
cd modelm
sudo cargo run
```

To change the volume (e.g. cut in half)
```
sudo cargo run -- -v 0.5
```
