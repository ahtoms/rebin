use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read, BufRead, BufReader};
use std::thread;
use std::str;

use crate::db::{get_source, commit_source, establish};
use crate::dump::Dump;
use rand::prelude::*;

/*
 * Because we may as well have a TCP variant right?
 * Why should we just make it only web? Allow those 486ers to connect
 * when all modern electronics are dead
 */
pub fn listen(address: &str) {
    if let Ok(listener) = TcpListener::bind(address) {
        for stream in listener.incoming() {
            if let Ok(client) = stream {
                handle(client);
            }
        }
    }
}


fn handle(mut stream: TcpStream) {
    thread::spawn(move || {
        let mut bufreader = BufReader::new(stream.try_clone().unwrap());
        let mut sbuf = String::new();
        let _ = bufreader.read_line(&mut sbuf);

        if sbuf.starts_with("GET ") {
            let spl : Vec<&str> = sbuf.split(' ').collect();
            if spl.len() >= 2 {
                if let Ok(n) = spl[1].parse::<u32>() {
                    if let Ok(d) = get_source(&establish(), n) {
                        sbuf.clear();
                        sbuf.push_str(format!("Source; #{}\n", d.dump_id).as_ref());
                        sbuf.push_str(d.text.as_ref());
                        let _ = stream.write(sbuf.as_bytes());
                    } else {
                        let _ = stream.write("Invalid Id".as_bytes());
                    }
                }
            }
        } else if sbuf.starts_with("PUT ") {
            let spl : Vec<&str> = sbuf.split(' ').collect();
            if spl.len() >= 2 {
                let mut nsbuf = String::new();
                let mut rng = rand::thread_rng();
                let lang = String::from(spl[1]);
                let id : u32 = rng.gen();
                let _ = bufreader.read_to_string(&mut nsbuf);
                let d = Dump::new("Anonymous".to_owned(), id, nsbuf, lang, Vec::new());
                let _ = commit_source(&establish(), &d);
            }
        }
    });
}
