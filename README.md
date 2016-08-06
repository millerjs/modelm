# modelm
[![Build Status](https://travis-ci.org/millerjs/modelm.svg?branch=master)](https://travis-ci.org/millerjs/modelm)

A OSX **Mechanical keyboard audio simulator** for your Mac keyboard written in [Rust](https://www.rust-lang.org/).

> *Get yourself clickity-clacking.*

Inspired by the [IBM Model M Keyboard](https://en.wikipedia.org/wiki/Model_M_keyboard) and a disproportionate love of clicky keyboards over non clicky keyboards, this is a simple program to simulate the Model M by providing audible keystroke feedback.

## Features
To provide audible feedback for your keystrokes, `modelm` hooks into OSX Quartz Events and must be run as `root` OR from a terminal with accessibility features enabled.

* **Stereo sounds** - Keys on the left sound like keys on the left. Keys on the right sound like Keys on the right.
* **Customizable resource loading** - You can pick your favorite clickity-clacks.  Just point `modelm` to a directory with sound bites and a config file.

## Requirements

Install OpenAL audio dependency.
```bash
brew install openal-soft libsndfile
```

## Installation


### Running a pre-compiled binary

To run the compile binary, download the tar [here](https://github.com/millerjs/modelm/releases/download/0.1.0/modelm_v0.1.0_OSX.tar.gz),
```bash
tar -zxf modelm_v0.1.0_OSX.tar.gz
cd modelm_v0.1.0
sudo ./modelm
```

### Usage

```bash
# To specify custom clickity clacks:
sudo ./modelm -d path/to/clacks

# To make the key gradient from left to right more dramatic
sudo ./modelm -x 5.0

# Or less dramatic
sudo ./modelm -x 0.5

# Or reverse because you have your headphones on backward
sudo ./modelm -x'-1'
```

#### Help output

```
modelm 0.2.0
Joshua Miller <jsmiller@uchicago.edu>
Turns your computer into a mechanical keyboard emulator!

USAGE:
	modelm [FLAGS] [OPTIONS]

FLAGS:
    -v, --debug      Debug output
    -h, --help       Prints help information
        --version    Prints version information

OPTIONS:
        --c <CONFIG>          Specify the config to parse click options from
    -d, --directory <DIR>     Specify the directory to load click sounds from
    -V, --volume <VOLUME>     Adjust the keyboard volume in range [0.0, 1.0]
    -x, --x-scale <XSCALE>    Specify the pan amount for the positional sound of clicks. A decimal (default: 1.0).  The larger the value, the further apart the clicks will sound. A value of 0 turns off positional sound. A value < 0 reverses the directionality.

```

#### Example config file
```yaml
switches:

  ## enter
  -  keycode_regex: '36'
     keydown_paths:
       - enter_down_1.wav
       - enter_down_2.wav

     keyup_paths:
       - enter_up_1.wav
       - enter_up_2.wav

  ## all other keys
  -  keycode_regex: '\d+'
     keydown_paths:
       - down_1.wav
       - down_2.wav

     keyup_paths:
       - up_1.wav
       - up_2.wav
```

### Installation from source

First, install [Rust](https://github.com/rust-lang/rustup) and [Cargo](https://crates.io/).

```
git clone git@github.com:millerjs/modelm.git
cd modelm
cargo run
```

### Options

You can pass options through cargo with a `--`, e.g. to change the volume:
```
sudo cargo run -- -v 0.5
```

### Credits

Currently ships with [IBM sounds](https://webwit.nl/input/kbsim/) and [HHKB Pro 2 Topre sounds]( https://www.youtube.com/watch?v=9hXeG_YEkBs).
