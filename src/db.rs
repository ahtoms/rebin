use std::time::{SystemTime, UNIX_EPOCH};
use rusqlite::{Connection, NO_PARAMS, OpenFlags, Result, params};
use crate::dump::Dump;
use crate::comment::Comment;
use rand::prelude::*;

macro_rules! DB_DEFAULTS {
    (DB_PATH) => ("./rebin.db")
}

pub fn establish() -> Connection {
    match Connection::open_with_flags(DB_DEFAULTS!(DB_PATH),
        OpenFlags::SQLITE_OPEN_READ_WRITE) {
            Ok(c) => { c },
            _ => { create() }
    }
}

pub fn create() -> Connection {
    if let Ok(c) = Connection::open(DB_DEFAULTS!(DB_PATH)) {
        match c.execute(include_str!("../db.sql"), NO_PARAMS) {
            Ok(_) => { println!("Table was created"); },
            Err(_) => {println!("Table was not created"); }
        }
        return c;
    } else {
        //At this point, *Force* panic if database cannot open
        return Connection::open_in_memory().unwrap();
    }
}

///
/// This will retrieve the source with comments
///
pub fn get_source(con: &Connection, id: u32) -> Result<Dump> {
    con.query_row("SELECT * FROM source WHERE source_id = (?1)", &[id], |row| {
        Ok(Dump::new(row.get(1)?, row.get(0)?, row.get(2)?, row.get(3)?, Vec::new()))
    })
}

pub fn commit_source(con: &Connection, source: &Dump) -> Result<usize> {
    let mut stmt = con.prepare("INSERT INTO source(source_id, username,
        text, lang, ts) VALUES((?1, ?2, ?3, ?4, ?5);")?;
    stmt.execute(params!(source.dump_id, source.username, source.text, source.lang,
     source.timestamp as u32))
}

pub fn commit_comment(con: &Connection, source: &Comment) -> Result<usize> {
    let mut rng = rand::thread_rng();
    let id : u32 = rng.gen();

    let mut stmt = con.prepare("INSERT INTO comment(comment_id, sid,
        response, line_start, line_end, ts) VALUES((?1, ?2, ?3, ?4, ?5);")?;
    stmt.execute(params!(id, source.comment_id, source.dump_id, source.comment,
     source.line_start, source.line_end, source.timestamp as u32))
}

///
/// TODO: Update to u64 in the future
/// u32 is used as a temporary measure
///
pub fn clean_up(con: &Connection, exp: u32) {
    let current_time : u32 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
    let mut stmt = con.prepare("SELECT source_id, ts FROM source;").unwrap();
    let _ = stmt.query_map(NO_PARAMS, |row| {
        let (id, ts) : (u32, u32) = (row.get(0)?, row.get(1)?);
        if (ts + exp) < current_time {
            let mut delstmt = con.prepare("DELETE FROM source WHERE source_id = (?1);")?;
            let _ = delstmt.execute(params!(id));
        }
        Ok(())
    });
}
