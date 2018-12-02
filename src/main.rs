#![feature(core_intrinsics)]

mod methods;

extern crate clap;

use self::methods::{passphrase, password};
use clap::{App, Arg};

fn main() {
    let matches = App::new("Passphrase Generator")
        .version("1.0")
        .author("Caleb Hattingh")
        .about("==============")

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

    println!("{}", passphrase(3, sep));
    println!("{}", password(16, &true, &true, &true, None));
}
