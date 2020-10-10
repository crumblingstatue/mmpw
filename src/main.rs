#![feature(min_const_generics)]

mod array_byte_vec;
pub mod binstring;

use binstring::BinString;
use rand::{seq::SliceRandom, thread_rng, Rng};

const LEN: u8 = 18;
const PW_ITEM_COUNT: u8 = 30;
const CKSUM_BITS: u8 = 9;

include!("words.inc.rs");

type Password = [u8; LEN as usize];

pub fn validate(pw: &Password, key: &BinString) -> bool {
    let mut bs = BinString::from_alphanumeric(pw);
    bs.hash(key);
    let mut reader = bs.reader();
    if reader.next_int(1) == 1 {
        reader.advance(12 + 8);
        if reader.next_int(10) != 326 {
            return false;
        }
    } else {
        reader.advance(PW_ITEM_COUNT as usize);
    }
    reader.advance(2 + 3 + 2 + 2 + 3 + 2 + 2 + 2 + 2 + 2 + 6 + 3 + 3);
    let rank = reader.next_int(7);
    if rank > 65 {
        return false;
    }
    if reader.len() > CKSUM_BITS as usize {
        reader.advance(reader.len() - CKSUM_BITS as usize);
    }
    let parsed_cksum = reader.next_int(CKSUM_BITS as usize);
    parsed_cksum as u32 == bs.calc_checksum()
}

fn fill_rand_words(buf: &mut Password, rng: &mut impl Rng) {
    for i in 0..3 {
        let word = SIX_LETTER_WORDS.choose(rng).unwrap();
        buf[i * 6..i * 6 + 6].copy_from_slice(&word[..]);
    }
}

#[test]
fn test_validate() {
    let key = binstring::hash_name(b"DEW");
    assert_eq!(validate(b"88H4B75X8FR9C54577", &key), false);
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
