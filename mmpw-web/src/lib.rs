use mmpw_validate::binstring;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn doit(names_arg: JsValue, words_arg: JsValue) -> Result<JsValue, JsValue> {
    let words = words_arg.as_string().unwrap();
    let names = names_arg.as_string().unwrap();
    let prepared_words = mmpw_gen::prepare_words(words.split_whitespace());
    let mut buf = String::new();
    let mut limiter = 0;
    for name in mmpw_gen::NAMES.iter() {
        let key = binstring::hash_name(name.as_bytes());
        limiter += mmpw_gen::permutate(&key, &prepared_words, name, |pw, name| {
            buf += name;
            buf += ": ";
            let s = std::str::from_utf8(pw).unwrap();
            buf += &s[0..6];
            buf += " ";
            buf += &s[6..12];
            buf += " ";
            buf += &s[12..18];
            buf += "\n";
        });
        if limiter > 1000 {
            buf += "Limiting results to avoid hanging your browser. Sorry.\n";
            break;
        }
    }
    for name in names.split_whitespace() {
        let key = binstring::hash_name(name.as_bytes());
        limiter += mmpw_gen::permutate(&key, &prepared_words, name, |pw, name| {
            buf += name;
            buf += ": ";
            let s = std::str::from_utf8(pw).unwrap();
            buf += &s[0..6];
            buf += " ";
            buf += &s[6..12];
            buf += " ";
            buf += &s[12..18];
            buf += "\n";
        });
        if limiter > 1000 {
            buf += "Limiting results to avoid hanging your browser. Sorry.\n";
            break;
        }
    }

    Ok(JsValue::from_str(&buf))
}
