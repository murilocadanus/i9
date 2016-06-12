#[macro_use]
extern crate log;

extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

pub fn init() {
	info!("Initialize");

    let mut verbose = false;
    let mut file = "app.ini".to_string();

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Parser generic data into JSON.");
        ap.refer(&mut verbose).add_option(&["-v", "--verbose"], StoreTrue, "Show processing data");
        ap.refer(&mut file).add_option(&["--config"], Store, "Config file name");
        ap.parse_args_or_exit();
    }

	if verbose {
        println!("file name is {}", file);
    }
    println!("Loading {}!", file);
}