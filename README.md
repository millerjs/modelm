# modelm 
[![Build Status](https://travis-ci.org/millerjs/modelm.svg?branch=master)](https://travis-ci.org/millerjs/modelm)

A OSX **Mechanical keyboard audio simulator** for your Mac keyboard written in [Rust](https://www.rust-lang.org/). 

> *Get yourself clickity-clacking.*

Inspired by the [IBM Model M Keyboard](https://en.wikipedia.org/wiki/Model_M_keyboard) and a disproportionate love of clicky keyboards over non clicky keyboards, this is a simple program to simulate the Model M by providing audible keystroke feedback.

## Features
To provide audible feedback for your keystrokes, `modelm` hooks into OSX Quartz Events and must be run as `root`. 

* **Positional sounds** - Keys on the left sound like keys on the left. Keys on the right sound like Keys on the right.
* **Dynamic resource loading** - You can pick your favorite clickity-clacks.  Just point `modelm` to a directory with sound bites.


## Requirements

Install OpenAL audio dependency.
```
brew install openal-soft libsndfile
```

## Installation


### Running a pre-compiled binary

To run the compile binary, download the tar [here](https://github.com/millerjs/modelm/releases/download/0.1.0/modelm_v0.1.0_OSX.tar.gz), 
```
tar -zxf modelm_v0.1.0_OSX.tar.gz
cd modelm_v0.1.0
sudo ./modelm
```

### Installation from source

Install [Rust](https://github.com/rust-lang/rustup) and [Cargo](https://crates.io/).

```
git clone git@github.com:millerjs/modelm.git
cd modelm
cargo build
```

## Usage
And run:
```
sudo cargo run
```

### Options

To change the volume (e.g. cut in half):
```
sudo cargo run -- -v 0.5
```

To specify custom clickity clacks:
```
sudo cargo run -- -c path/to/clacks
```

### Credits

Currently ships with IBM sounds from https://webwit.nl/input/kbsim/
