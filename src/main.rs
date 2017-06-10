#![allow(unused)]

extern crate coingrab;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate rand;

mod errors {
    error_chain! {
        foreign_links {
            AtomBlob(::coingrab::atomic_blobject::Error);
            Coingrab(::coingrab::errors::Error);
        }
    }
}

use errors::*;

use std::time::Duration;
use std::thread;
use rand::{thread_rng, Rng};
use coingrab::model::Data;
use coingrab::atomic_blobject::AtomBlob;
use coingrab::social;

quick_main!(run);

fn run() -> Result<()> {
    env_logger::init().expect("");

    const PROCESSES: &[fn(AtomBlob<Data>)] = &[
        fourchan_process
    ];

    let data = AtomBlob::new("coingrab-data.json")?;

    for process in PROCESSES {
        let data = data.clone();
        thread::spawn(move || process(data));
    }

    loop { thread::sleep(Duration::from_secs(1)); }
}

fn fourchan_process(data: AtomBlob<Data>) {
    periodically("4chan", move || fourchan_job(data.clone()));
}

fn fourchan_job(mut data: AtomBlob<Data>) -> Result<()> {
    let new = social::fourchan::run()?;

    let mut data = data.get_mut()?;
    data.fourchan.push(new);

    Ok(())
}

const MIN_INIT_DELAY_S: u64 = 0;
const MAX_INIT_DELAY_S: u64 = 1;
const MIN_DELAY_S: u64 = 30;
const MAX_DELAY_M: u64 = 10;

fn periodically<F: Fn() -> Result<()>>(name: &str, f: F) {
    let ms_per_s = 1000;
    let s_per_m = 60;

    let min_init_delay_ms = MIN_INIT_DELAY_S * ms_per_s;
    let max_init_delay_ms = MAX_INIT_DELAY_S * ms_per_s;
    let min_delay_ms = MIN_DELAY_S * ms_per_s;
    let max_delay_ms = MAX_DELAY_M * s_per_m * ms_per_s;

    let init_delay = random_delay_ms(min_init_delay_ms, max_init_delay_ms);
    info!("init {} process in: {}", name, durfmt(init_delay));
    thread::sleep(init_delay);

    loop {
        info!("running {} process", name);

        if let Err(e) = f() {
            error!("process {} failed!", name);
            error!("{} error: {}", name, e);
            for e in e.iter().skip(1) {
                error!("{} cause: {}", name, e);
            }
        }

        let delay = random_delay_ms(min_delay_ms, max_delay_ms);
        info!("next {} process in: {}", name, durfmt(delay));
        thread::sleep(delay);
    }
}

fn random_delay_ms(min: u64, max: u64) -> Duration {
    let delay_ms = thread_rng().gen_range(min, max);
    Duration::from_millis(delay_ms)
}

fn durfmt(d: Duration) -> String {
    let nanos_per_milli = 1000000;

    if d < Duration::from_secs(1) {
        format!("{}ms", d.subsec_nanos() / nanos_per_milli)
    } else if d < Duration::from_secs(60) {
        format!("{}s", d.as_secs())
    } else {
        format!("{}m{}s", d.as_secs() / 60, d.as_secs() % 60)
    }
}
