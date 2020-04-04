use std::thread;
use std::time::Duration;
use crate::db::{establish, clean_up};

///
/// TODO: Have a graceful terminate function
///
pub fn cleaner_start() {
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(604800));
            let db = establish();
            clean_up(&db, 604800);
        }
    });
}
