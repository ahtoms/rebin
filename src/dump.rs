use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Serialize, Deserialize};
use crate::comment::Comment;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dump {
    pub dumpid: u32,
    pub username: String,
    pub text: String,
    pub lang: String,
    pub comments: Vec<Comment>,
    pub timestamp: u64
}

impl Dump {

    pub fn new(username: String, dumpid: u32, text: String, lang: String, comments: Vec<Comment>)
        -> Dump {

        Dump { username: username, dumpid: dumpid, text: text, lang: lang, comments: comments,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() }
    }


}
