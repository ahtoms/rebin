use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub username: String, //0 will be anonymous
    pub comment_id: u32,
    pub dump_id: u32,
    pub line_start: u32,
    pub line_end: u32,
    pub comment: String,
    pub timestamp: u64
}

/*

The source code will look like

*/

#[allow(dead_code)]
impl Comment {
    pub fn new_anon(dump_id: u32, comment: String, line_start: u32, line_end: u32,
        timestamp: u64)
        -> Comment {

        Comment {
            comment_id: 0,
            username: "Anonymous".to_owned(),
            dump_id: dump_id,
            comment: comment,
            line_start: line_start,
            line_end: line_end,
            timestamp: timestamp
        }
    }
}
