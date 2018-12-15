#![feature(core_intrinsics)]

mod methods;

extern crate clap;

use self::methods::{passphrase, password};
use clap::{App, Arg};
use clap::SubCommand;
use clap::ArgMatches;

fn main() {
    let matches = App::new("Passphrase Generator")
        .version("1.0")
        .author("Caleb Hattingh")
        .about("==============")
        .subcommand(SubCommand::with_name("phrase")
            .about("Passphrase made up of dictionary words")
            .version("1.0")
            .author("Caleb Hattingh")
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
        )
        .subcommand(SubCommand::with_name("password")
            .about("Password made up of symbols")
            .version("1.0")
            .author("Caleb Hattingh")
            .arg(
                Arg::with_name("length")
                    .short("l")
                    .long("length")
                    .help("Password length")
                    .default_value("16"),
            )
            .arg(
                Arg::with_name("numbers")
                    .short("n")
                    .long("numbers")
                    .takes_value(true)
                    .default_value("1")
                    .help("Include numbers")
            )
            .arg(
                Arg::with_name("upper")
                    .short("u")
                    .long("upper")
                    .takes_value(true)
                    .default_value("1")
                    .help("Include uppercase ASCII letters")
            )
            .arg(
                Arg::with_name("lower")
                    .short("w")
                    .long("lower")
                    .takes_value(true)
                    .default_value("1")
                    .help("Include lowercase ASCII letters")
            )
            .arg(
                Arg::with_name("special")
                    .short("s")
                    .long("special")
                    .takes_value(true)
                    .default_value("!@#$%^&*(){}[]-+=_:;<>?")
                    .help("Include special characters")
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("phrase") {
        let sep = match matches.value_of("separator") {
            Some(s) => s,
            None => " ",
        };
        println!(
            "{}",
            passphrase(
                matches.value_of("word_count").unwrap().parse().unwrap(),
                sep)
        );
    } else if let Some(matches) = matches.subcommand_matches("password") {
        let pwl: u8 = match matches.value_of("length") {
            Some(s) => {
                s.parse().unwrap()
            }
            None => 16,
        };
        let numbers = process_bool_option(matches, "numbers");
        let upper = process_bool_option(matches, "upper");
        let lower = process_bool_option(matches, "lower");
        let special = matches.value_of("special");
        println!("{}", password(
            pwl,
            &numbers,
            &upper,
            &lower,
            special)
        );
    }
}

fn process_bool_option(matches: &ArgMatches, name: &str) -> bool {
    match matches.value_of(name).unwrap() {
        "1" | "yes" | "y" => true,
        _ => false,
    }
}
