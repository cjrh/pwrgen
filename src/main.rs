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
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .help("password type. Either 'passphrase' or 'password'")
                .default_value("passphrase"),
        )
        .arg(
            Arg::with_name("word_count")
                .short("w")
                .long("word_count")
                .default_value("4"),
        )
        .arg(
            Arg::with_name("separator")
                .short("s")
                .long("separator")
                .help("Word separator for passphrase generation")
                .default_value(" "),
        )
        .arg(
            Arg::with_name("password_length")
                .short("pwl")
                .long("password_length")
                .help("Length")
                .default_value("16"),
        )
        .get_matches();

    match matches.value_of("separator") {
        Some(s) => {
            println!(
                "{}",
                passphrase(matches.value_of("word_count").unwrap().parse().unwrap(), s)
            );
            s
        }
        None => " ",
    };

    //    println!("{}", passphrase(3, sep));

    let pwl: u8 = match matches.value_of("password_length") {
        Some(s) => s.parse().unwrap(),
        None => 16,
    };

    println!("{}", password(pwl, &true, &true, &true, None));
}
