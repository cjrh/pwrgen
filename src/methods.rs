use std::fs::File;
use std::io::{BufReader, BufRead, Error};

extern crate rand;

use rand::{Rng, thread_rng};
use rand::seq::IteratorRandom;

extern crate itertools; // 0.7.8

use itertools::free::join;

fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

pub fn passphrase(word_count: u8, sep: &str) -> Result<String, Error> {
    println!("hi!");

    let text = include_str!("words_alpha.txt");
    let lines = text.split_whitespace();

//    let file = File::open("words_alpha.txt")?;
//
//    let lines: Vec<_> = BufReader::new(&file)
//        .lines()
//        .filter_map(Result::ok)
//        .collect();

    let mut rng = thread_rng();
    let phrase = join(
        &lines.choose_multiple(&mut rng, word_count as usize),
        " ",
    );


    println!("passphrase: {:?}", phrase);
    Ok(phrase)
}


pub fn passphrase_mmap(word_count: u8, sep: &str) -> Result<String, Error> {
    /// Read the memory map example here:
    /// https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html
    let file = File::open("words_alpha.txt")?;
//    let lines: Vec<&str> = file.lines().collect();
    Ok("abc".to_string())
}
