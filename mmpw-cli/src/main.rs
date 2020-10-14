use mmpw_gen::{go_random, load_words, permutate, prepare_words, Word, NAMES, SIX_LETTER_WORDS};
use mmpw_validate::binstring;
use std::path::PathBuf;
use structopt::StructOpt;

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
    /// Try the same passwords with different names in hopes of it becoming valid with at least one.
    #[structopt(short, long)]
    brute_force_with_names: bool,
}

fn main() {
    let opt = Opt::from_args();
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

    let mut key_name_pairs = Vec::new();
    key_name_pairs.push((binstring::hash_name(opt.name.as_bytes()), &opt.name[..]));

    if opt.brute_force_with_names {
        for name in NAMES.iter() {
            key_name_pairs.push((binstring::hash_name(name.as_bytes()), name));
        }
    }

    if opt.random {
        go_random(&key_name_pairs[0].0, words, &key_name_pairs[0].1);
    } else {
        let mut count = 0;
        for (key, name) in key_name_pairs {
            count += permutate(&key, words, name);
        }
        eprintln!("Finished. Found {} valid passwords", count);
    }
}
