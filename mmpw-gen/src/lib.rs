#![feature(min_const_generics)]

use mmpw_validate::{binstring::BinString, validate, Password, LEN};
use rand::{prelude::SliceRandom, thread_rng, Rng};
use std::convert::TryInto;

pub use data::{NAMES, SIX_LETTER_WORDS};

mod data;
mod slice_permutations;

pub type Word = [u8; 6];
pub type Key = BinString;

fn fill_rand_words(buf: &mut Password, rng: &mut impl Rng, words: &[Word]) {
    for i in 0..3 {
        let word = words.choose(rng).unwrap();
        buf[i * 6..i * 6 + 6].copy_from_slice(&word[..]);
    }
}

pub fn permutate(
    key: &Key,
    words: &[Word],
    name: &str,
    mut f: impl FnMut(&Password, &str),
) -> usize {
    let mut s = [0; LEN as usize];
    let permutations = slice_permutations::SlicePermutations::<_, 3>::new(words);
    let mut count = 0;
    for [a, b, c] in permutations {
        s[0..6].copy_from_slice(&a[..]);
        s[6..12].copy_from_slice(&b[..]);
        s[12..18].copy_from_slice(&c[..]);
        if validate(&s, key) {
            f(&s, name);
            count += 1;
        }
    }
    count
}

pub fn go_random(key: &Key, words: &[Word], name: &str) {
    let mut s = [0; LEN as usize];
    let mut rng = thread_rng();
    loop {
        fill_rand_words(&mut s, &mut rng, words);
        if validate(&s, key) {
            show(&s, name);
        }
    }
}

fn show(pw: &Password, name: &str) {
    let utf = std::str::from_utf8(pw).unwrap();
    println!(
        "name: {} password: {} {} {}",
        name,
        &utf[0..6],
        &utf[6..12],
        &utf[12..]
    );
}

fn word_filter_map(word: &str) -> Option<[u8; 6]> {
    match word.as_bytes().try_into() {
        Ok(bword) => {
            let mut bword: Word = bword;
            bword.make_ascii_uppercase();
            if bword.contains(&b'V') {
                eprintln!(
                    "Warning: invalid word: \'{}\'. Passwords cannot contain the letter V.",
                    word
                );
                return None;
            }
            Some(bword)
        }
        Err(_) => {
            eprintln!(
                "Warning: invalid word \'{}\'. It must be exactly 6 letters long.",
                word
            );
            None
        }
    }
}

pub fn prepare_words<'a>(words: impl Iterator<Item = &'a str>) -> Vec<[u8; 6]> {
    words.filter_map(word_filter_map).collect()
}
