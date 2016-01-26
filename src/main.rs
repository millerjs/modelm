//! Small OSX commandline app that uses your keyboard to
//! emulate mechanical keyboard audio.

extern crate modelm;
extern crate ears;
extern crate clap;

use modelm::keyboard::Keyboard;

use clap::Arg;
use clap::App;

static DEFAULT_PATH: &'static str = "resources/modelm";

fn main() -> () {
    ears::init();

    let matches = App::new("modelm")
        .version("0.1.0")
        .author("Joshua Miller <jsmiller@uchicago.edu>")
        .about("Turns your computer into a mechanical keyboard emulator!")
        .arg(Arg::with_name("VOLUME")
             .short("v")
             .long("volume")
             .help("Adjust the keyboard volume in range [0.0, 1.0]")
             .takes_value(true))
        .arg(Arg::with_name("CLICKS")
             .short("c")
             .long("click-directory")
             .help("Specify the directory to load click sounds from")
             .takes_value(true))
        .arg(Arg::with_name("XSCALE")
             .short("x")
             .long("x-scale")
             .help("Specify the pan amount for the positional sound of clicks. \
                    A decimal (default: 1.0).  The larger the value, the further \
                    apart the clicks will sound. A value of 0 turns off positional \
                    sound. A value < 0 reverses the directionality.")
             .takes_value(true))
        .get_matches();

    // Parse the click directory
    let dir = matches.value_of("CLICKS").unwrap_or(DEFAULT_PATH);

    // Parse the volume
    let volume = match matches.value_of("VOLUME").unwrap_or("1.0").parse::<f32>() {
        Ok(v) => v,
        Err(_) => panic!("Volume must be a decimal between 0 and 1."),
    };

    // Parse the volume
    let x_scale = match matches.value_of("XSCALE").unwrap_or("1.0").parse::<f32>() {
        Ok(v) => v,
        Err(_) => panic!("x-scale must be a decimal. (default: 1.0)"),
    };

    Keyboard::new(dir)
        .set_volume(volume)
        .set_x_scale(x_scale)
        .listen();
}
