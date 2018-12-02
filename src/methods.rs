use std::io::Error;

extern crate rand;

use rand::thread_rng;
use rand::seq::IteratorRandom;

extern crate itertools; // 0.7.8

use itertools::free::join;

fn _print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

pub fn passphrase(word_count: u8, sep: &str) -> Result<String, Error> {
    let text = include_str!("words_alpha.txt");
    let lines = text.split_whitespace();
    let mut rng = thread_rng();
    let phrase = join(
        &lines.choose_multiple(&mut rng, word_count as usize),
        sep,
    );
    Ok(phrase)
}

pub fn password(length: u8, numbers: &bool, upper: &bool, lower: &bool,
                special: Option<&str>, ) -> Result<String, Error> {
    let mut rng = thread_rng();
    let mut choicevalues= "".to_string();

    if *numbers == true {
        choicevalues.push_str("0123456789");
    }
    if *upper == true {
        choicevalues.push_str("ABCDEFGIHJKLMNOPQRSTUVWXYZ")
    }
    if *lower == true {
        choicevalues.push_str("abcdefgihjklmnopqrstuvwxyz")

    }
    match special {
        Some(v) => choicevalues.push_str(v),
        None => choicevalues.push_str("!@#$%^&*(){}[]-+=_:;<>?"),
    }

    let phrase = join(
        choicevalues.chars().choose_multiple(&mut rng, length as usize),
        "",
    );
    Ok(phrase)
}
