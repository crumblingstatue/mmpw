#![feature(min_const_generics)]

use mmpw::{binstring, validate, Password, LEN};
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::convert::TryInto;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod slice_permutations;

type Word = [u8; 6];
type Key = binstring::BinString;

include!("words.inc.rs");

fn fill_rand_words(buf: &mut Password, rng: &mut impl Rng, words: &[Word]) {
    for i in 0..3 {
        let word = words.choose(rng).unwrap();
        buf[i * 6..i * 6 + 6].copy_from_slice(&word[..]);
    }
}

#[derive(StructOpt)]
struct Opt {
    /// The name you want to use in the password. It will only work with this name.
    name: String,
    /// A list of custom words to use instead of the built-in ones.
    #[structopt(short, long)]
    custom_words: Option<Vec<String>>,
    /// A file containing custom words to read from. This overrides the custom-words option.
    #[structopt(short = "f", long = "word-file")]
    custom_word_file: Option<PathBuf>,
    /// Try passwords in a random fashion instead of in order. This will never finish.
    #[structopt(short, long)]
    random: bool,
}

fn permutate(key: &Key, words: &[Word]) {
    let mut s = [0; LEN as usize];
    let permutations = slice_permutations::SlicePermutations::<_, 3>::new(words);
    let mut count = 0;
    for [a, b, c] in permutations {
        s[0..6].copy_from_slice(&a[..]);
        s[6..12].copy_from_slice(&b[..]);
        s[12..18].copy_from_slice(&c[..]);
        if validate(&s, key) {
            show(&s);
            count += 1;
        }
    }
    eprintln!("Finished. Found {} valid passwords", count);
}

fn go_random(key: &Key, words: &[Word]) {
    let mut s = [0; LEN as usize];
    let mut rng = thread_rng();
    loop {
        fill_rand_words(&mut s, &mut rng, words);
        if validate(&s, key) {
            show(&s);
        }
    }
}

fn show(pw: &Password) {
    let utf = std::str::from_utf8(pw).unwrap();
    println!("{} {} {}", &utf[0..6], &utf[6..12], &utf[12..]);
}

fn prepare_words<'a>(words: impl Iterator<Item = &'a str>) -> Vec<[u8; 6]> {
    words
        .filter_map(|word| word.as_bytes().try_into().ok())
        .collect()
}

fn load_words(path: &Path) -> Vec<[u8; 6]> {
    let text = std::fs::read_to_string(path).unwrap();
    let words = text.split_whitespace();
    prepare_words(words)
}

fn main() {
    let opt = Opt::from_args();
    let key = binstring::hash_name(opt.name.as_bytes());
    let words: Vec<Word>;
    let words: &[Word] = match (opt.custom_words, opt.custom_word_file) {
        (None, None) => &SIX_LETTER_WORDS,
        (Some(cust_words), None) => {
            words = prepare_words(cust_words.iter().map(AsRef::as_ref));
            &words[..]
        }
        (_, Some(word_file)) => {
            words = load_words(&word_file);
            &words[..]
        }
    };

    if opt.random {
        go_random(&key, words);
    } else {
        permutate(&key, words);
    }
}
