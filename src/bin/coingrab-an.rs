#![allow(unused)]

#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate coingrab;

mod errors {
    error_chain! {
        foreign_links {
            AtomBlob(::coingrab::atomic_blobject::Error);
            Coingrab(::coingrab::errors::Error);
        }
    }
}

use errors::*;

use coingrab::model::Data;
use coingrab::atomic_blobject::AtomBlob;

quick_main!(run);

fn run() -> Result<()> {
    env_logger::init().expect("");

    let data: AtomBlob<Data> = AtomBlob::new("coingrab-data.json")?;

    Ok(())
}
