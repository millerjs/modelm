//! Small OSX commandline app that Keyboard uses your keyboard to
//! emulate a mechanical keyboard.

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
        .get_matches();

    // Parse the click directory
    let dir = matches.value_of("CLICKS").unwrap_or(DEFAULT_PATH);

    // Parse the volume
    let volume = match matches.value_of("VOLUME").unwrap_or("1.0").parse::<f32>() {
        Ok(v) => v,
        Err(_) => panic!("Volume must be a decimal between 0 and 1."),
    };

    Keyboard::new(dir).set_volume(volume).listen();
}
