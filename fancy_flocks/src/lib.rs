extern crate fs2;
#[macro_use]
extern crate log;
extern crate rand;

pub mod sd_flock;
pub mod dirty_flock;

pub mod scoped {
    pub mod dirty_flock;
}
