#![feature(core_intrinsics)]
mod methods;

extern crate clap;

use self::methods::passphrase;
use clap::{App, Arg, SubCommand};

fn main() {
        println!("Hello, world!");

        let matches = App::new("My Program")
                .version("1.0")
                .author("Caleb")
                .about("Blah Blah")
                .arg(Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Enable verbosity"))
                .get_matches();
        let verb = matches.is_present("verbose");
        println!("verb = {:?}", verb);

        passphrase(3, "abc");
        println!("end main");
}
