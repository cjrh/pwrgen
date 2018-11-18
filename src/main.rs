#![feature(core_intrinsics)]

mod methods;

extern crate clap;

use self::methods::passphrase;
use clap::{App, Arg};

fn main() {
    let matches = App::new("My Program")
        .version("1.0")
        .author("Caleb")
        .about("Blah Blah")

        .arg(Arg::with_name("separator")
            .short("s")
            .long("separator")
            .help("Word separator for passphrase generation")
            .default_value(" ")
        )

        .get_matches();

    let sep = match matches.value_of("separator") {
        Some(s) => s,
        None => " ",
    };

    match passphrase(3, sep) {
        Ok(new_passphrase) => println!("{}", new_passphrase),
        Err(err) => println!("Error occurred: {:?}", err),
    }
}
