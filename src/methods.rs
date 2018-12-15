//! Generate a passphrase using a built-in dictionary
//!
//! # Examples
//!
//! NOTE: due to https://github.com/rust-lang/rust/issues/50784 it currently isn't possible
//! (actually: easy) to run doctests for projects that have a binary target.
//!
//! ```
//! panic!();
//! assert_eq!(methods::passphrase(1, " ").unwrap().matches(" ").count(), 0);
//! ```

extern crate rand;

use rand::thread_rng;
use rand::seq::IteratorRandom;

extern crate itertools; // 0.7.8

use itertools::free::join;

/// The second block
///
/// # Examples
///
/// ```
/// assert!(false);
/// use methods::passphrase;
///
/// assert_eq!(passphrase(1, " ").matches(" ").count(), 0);
/// assert_eq!(passphrase(2, " ").matches(" ").count(), 1);
/// assert_eq!(passphrase(3, " ").matches(" ").count(), 2);
/// assert_eq!(passphrase(4, " ").matches(" ").count(), 3);
/// assert_eq!(passphrase(5, " ").matches(" ").count(), 4);
/// ```
pub fn passphrase(word_count: u8, sep: &str) -> String {
    let text = include_str!("words_alpha.txt");
    let lines = text.split_whitespace();
    let mut rng = thread_rng();
    join(
        &lines.choose_multiple(&mut rng, word_count as usize),
        sep,
    )
}

pub fn password(length: u8, numbers: &bool, upper: &bool, lower: &bool,
                special: Option<&str>, ) -> String {
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
        // Zero disables special characters
        Some("0") => (),
        Some(v) => choicevalues.push_str(v),
        None => choicevalues.push_str("!@#$%^&*(){}[]-+=_:;<>?"),
    }

    (0..length)
        .map(|_| choicevalues.chars().choose(&mut rng).unwrap().clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{passphrase, password};

    #[test]
    fn test_passphrase() {
        assert_eq!(passphrase(1, " ").matches(" ").count(), 0);
        assert_eq!(passphrase(2, " ").matches(" ").count(), 1);
        assert_eq!(passphrase(3, " ").matches(" ").count(), 2);
        assert_eq!(passphrase(4, " ").matches(" ").count(), 3);
        assert_eq!(passphrase(5, " ").matches(" ").count(), 4);
    }

    #[test]
    fn test_password() {
        for i in 1..100 {
            let p = password(i, &true, &true, &true, None);
            println!("{}", p);
            assert_eq!(p.len(), i as usize);
        }
    }
}



