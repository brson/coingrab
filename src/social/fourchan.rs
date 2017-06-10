use util;
use std::cell::RefCell;
use select::document::Document;
use select::predicate::{Any, Class, Predicate};
use errors::*;
use std::collections::HashMap;
use coins::COINS;
use chrono::UTC;
use model::{FourChanRun, FourChanThread};

pub fn run() -> Result<FourChanRun> {
    let url = "https://4chan.org/biz/";
    let content = util::http_get(url)?;
    let doc = Document::from(&*content);

    let mut results = vec![];

    for thread in doc.find(Class("thread")) {
        let id = thread.attr("id").and_then(|id| {
            if id.starts_with("t") {
                Some(id.chars().skip(1).collect::<String>())
            } else {
                None
            }
        });
        let link = id.as_ref().map(|id| format!("https://boards.4chan.org/biz/thread/{}", id));

        if let Some(link) = link.as_ref() {
            debug!("4chan thread: {:?}", link);
        } else {
            debug!("unknown 4chan thread id: {:?}", thread.attr("id"));
            continue;
        }

        let id = id.expect("");
        let link = link.expect("");

        let mut strings = RefCell::new(vec![]);
        let mut push = |s| {
            // Something about what I'm doing is producing dupe lines, so
            // de-dupe them
            let mut strings = strings.borrow_mut();
            if strings.last() != Some(&s) {
                strings.push(s);
            }
        };
        for post in thread.find(Class("post")) {
            for subject in post.find(Class("subject").descendant(Any)) {
                let text = subject.text();
                push(text);
            }
            for post in post.find(Class("postMessage")) {
                for post_body in post.descendants() {
                    let text = post_body.text();
                    push(text);
                }
            }
        }

        let sym = analyze_thread(&strings.borrow());
        if let Some(sym) = sym {
            results.push(FourChanThread {
                sym: sym,
                id: id,
                link: link,
            })
        }
    }

    Ok(FourChanRun {
        time: UTC::now(),
        threads: results,
    })
}

fn analyze_thread(strings: &[String]) -> Option<String> {

    let mut symbols = HashMap::new();

    for s in strings {
        let mut found = true;
        for word in util::words(s) {
            for coin in COINS.iter() {
                let is_coin = word == coin.sym.to_lowercase()
                    || word == coin.name.to_lowercase();
                if is_coin {
                    *symbols.entry(coin.sym).or_insert(1) += 1;
                }
            }
        }
        for coin in COINS.iter() {
            if coin.name.contains(' ') {
                if s.contains(coin.name) {
                    *symbols.entry(coin.sym).or_insert(1) += 1;
                }
            }
        }
    }

    let mut sym_weights: Vec<_> = symbols.into_iter().map(|(k, v)| (v, k)).collect();
    sym_weights.sort();

    debug!("4chan thread weights:");
    for &(weight, sym) in &sym_weights {
        debug!("{}: {}", sym, weight);
    }

    sym_weights.pop().map(|(_, s)| s.to_owned())
}
