
mod dump;
mod comment;
mod cleaner;
mod db;
mod routes;

use std::thread;
use actix_web::{App, HttpServer, web::ServiceConfig};
use routes::tcp::listen;

const HEADER: &'static str = "
           _    _
 ___  ___ | |_ |_| ___
|  _|| -_|| . || ||   |
|_|  |___||___||_||_|_|

";

fn config(cfg: &mut ServiceConfig) {
    routes::bin::register_routes(cfg);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    /*
        Rebin,
        A hastebin like software with inline commenting
        Aiming to support logins after initial dump features are working.
    */
    println!("{}", HEADER);
    cleaner::cleaner_start();
    thread::spawn(move || { listen("0.0.0.0:9000"); });
    HttpServer::new(|| {
        App::new().configure(config)
    })
    .bind("0.0.0.0:8050")
    .expect("Rebin did not run")
    .run()
    .await
}
