use mmpw::{binstring, validate, Password, LEN};
use rand::{seq::SliceRandom, thread_rng, Rng};

include!("../words.inc.rs");

fn fill_rand_words(buf: &mut Password, rng: &mut impl Rng) {
    for i in 0..3 {
        let word = SIX_LETTER_WORDS.choose(rng).unwrap();
        buf[i * 6..i * 6 + 6].copy_from_slice(&word[..]);
    }
}

fn main() {
    let name = std::env::args().nth(1).expect("Need a name as argument");
    let mut rng = thread_rng();
    let mut s = [0; LEN as usize];
    let key = binstring::hash_name(name.as_bytes());
    loop {
        fill_rand_words(&mut s, &mut rng);
        if validate(&s, &key) {
            let utf = std::str::from_utf8(&s).unwrap();
            println!("{} {} {}", &utf[0..6], &utf[6..12], &utf[12..]);
        }
    }
}
