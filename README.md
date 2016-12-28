# modelm
[![Build Status](https://travis-ci.org/millerjs/modelm.svg?branch=master)](https://travis-ci.org/millerjs/modelm)

A **Mechanical keyboard audio simulator** for your keyboard written in [Rust](https://www.rust-lang.org/).

> *Get yourself clickity-clacking.*

Inspired by the [IBM Model M Keyboard](https://en.wikipedia.org/wiki/Model_M_keyboard) and a disproportionate love of clicky keyboards over non clicky keyboards, this is a simple program to simulate the Model M by providing audible keystroke feedback.

## Features
To provide audible feedback for your keystrokes, `modelm` hooks into OSX Quartz Events (on OSX) or your `/dev/input` keyboard device and must be run as `root`(or from a terminal with accessibility features enabled on OSX).

* **Stereo sounds** - Keys on the left sound like keys on the left. Keys on the right sound like Keys on the right.
* **Custom resource loading** - You can pick your favorite clickity-clacks.  Just point `modelm` to a directory with sound bites and a config file.

## Requirements

Install OpenAL audio dependency.

### OSX

```bash
brew update
brew install openal-soft libsndfile
```
### Debian/Ubuntu

```bash
apt-get update
apt-get install libopenal-dev libsndfile1-dev
```

### Arch

You may also need pulseaudio as shown

```bash
pacman -S openal libsndfile pulseaudio-alsa
```

## Installation


### Running a pre-compiled binary

To run the compiled
binary,
[download the latest release](https://github.com/millerjs/modelm/releases/latest) and
simply run the following from within the extracted tarball

```bash
./modelm -d resources/modelm
```

### Installation from source

First, install [Rust](https://github.com/rust-lang/rustup) and [Cargo](https://crates.io/).

```
git clone https://github.com/millerjs/modelm.git
cd modelm
cargo run --release
```

### Usage

```bash
# To specify custom clickity clacks:
sudo ./modelm -d path/to/clacks

# To make the key gradient from left to right more dramatic
sudo ./modelm -x 5.0

# Or less dramatic
sudo ./modelm -x 0.5

# Or reverse because you have your headphones on backward, silly
sudo ./modelm -x'-1'
```

#### Note: Linux usage

Currently, `modelm` defaults to reading from `/dev/input/event0`, but
you can specify which event device to read at runtimefrom by setting the
`MODELM_INPUT_DEVICE` environment variable to a new path.


#### Help output

```
modelm 0.3.0
Joshua Miller <jsmiller@uchicago.edu>
Turns your computer into a mechanical keyboard emulator!

USAGE:
	modelm [FLAGS] [OPTIONS]

FLAGS:
    -v, --debug      Debug output
    -h, --help       Prints help information
        --version    Prints version information

OPTIONS:
    -c, --config <CONFIG>     Specify the config to parse click options from
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

### Options

You can pass options through cargo with a `--`, e.g. to change the volume:
```
sudo cargo run --release -- -V 0.5
```

### Credits

Currently ships with [IBM sounds](https://webwit.nl/input/kbsim/) and [HHKB Pro 2 Topre]( https://www.youtube.com/watch?v=9hXeG_YEkBs) sounds.
