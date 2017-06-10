#![allow(unused)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
extern crate fancy_flocks;
extern crate reqwest;
extern crate rand;
extern crate select;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tempdir;

pub mod social {
    pub mod fourchan;
}

pub mod model {
    pub use self::v1::*;

    pub mod v1;
}

pub mod coins;
pub mod sentiment;
pub mod util;
pub mod atomic_blobject;

pub mod errors {
    error_chain! {
        foreign_links {
            Reqwest(::reqwest::Error);
            AtomBlob(::atomic_blobject::Error);
        }
    }
}
