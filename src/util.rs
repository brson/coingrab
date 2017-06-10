use reqwest;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::result::Result as StdResult;
use errors::*;

pub fn http_get(url: &str) -> Result<String> {
    let mut resp = reqwest::get(url)?;

    if !resp.status().is_success() {
        return Err(format!("unsuccesful http get: {}", resp.status()).into());
    }

    let mut content = String::new();
    resp.read_to_string(&mut content)
        .chain_err(|| "unable to read html body")?;

    Ok(content)
}

pub fn words(s: &str) -> Vec<String> {
    let words = s.split(' ');
    let clean_word = |word: &str| -> String {
        word.chars().filter(|c| c.is_alphanumeric()).collect()
    };
    let words = words.map(clean_word);
    let words = words.filter(|w| !w.chars().all(char::is_whitespace));
    let words = words.map(|s| s.to_lowercase());
    words.collect()
}

pub fn atomic_file_rename<P, Q>(src: P, dst: Q) -> StdResult<(), io::Error>
    where P: AsRef<Path>, Q: AsRef<Path>
{
    if cfg!(windows) {
        // TODO use ReplaceFile
        // This doesn't matter much because the rename is done under
        // an exclusive flock
        //panic!("unimplemented atomic file move");
    }

    fs::rename(src, dst)
}

