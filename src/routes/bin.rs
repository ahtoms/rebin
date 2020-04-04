use std::fs::File;
use std::io::prelude::*;
use actix_web::{web, web::ServiceConfig, HttpRequest, HttpResponse, Result};
use crate::db::{get_source, commit_source, establish, commit_comment};
use crate::dump::Dump;
use crate::comment::Comment;
use rand::prelude::*;

const HEADER : &'static str = include_str!("../../static/html/head.html");
const FOOTER : &'static str = include_str!("../../static/html/footer.html");

fn get_bin(req: HttpRequest) -> HttpResponse {
    let id: &str = req.match_info().query("id");
    let dump = get_source(&establish(), id.parse::<u32>().unwrap());
    let mut html = String::new();

    html.push_str(HEADER);
    if let Ok(mut res) = dump {
        //Clean up
        res.text = res.text.replace("&", "&amp;");
        res.text = res.text.replace("<", "&lt;");
        html.push_str(format!("
            <div class='vid'>Source #{}</div>
            <pre><code class='language-{}'>
                {}
            </code></pre>
        ", res.dump_id, res.lang, res.text).as_ref());
        html.push_str(FOOTER);
        HttpResponse::Ok().body(html)
    } else {
        html.push_str("<pre><code>Invalid Id</code></pre>");
        html.push_str(FOOTER);
        HttpResponse::Ok().body(html)
    }
}

async fn make_bin(mut obj: web::Path<Dump>) -> Result<HttpResponse> {
    let mut rng = rand::thread_rng();
    let id : u32 = rng.gen();
    obj.dump_id = id;
    if let Ok(_) = commit_source(&establish(), obj.as_ref()) {
        Ok(HttpResponse::Ok().json("{ 'success' : 1 }"))
    } else {
        Ok(HttpResponse::Ok().json("{ 'success' : 0 }"))
    }
}

async fn make_comm(obj: web::Path<Comment>) -> Result<HttpResponse> {
    if let Ok(_) = commit_comment(&establish(), obj.as_ref()) {
        Ok(HttpResponse::Ok().json("{ 'success' : 1 }"))
    } else {
        Ok(HttpResponse::Ok().json("{ 'success' : 0 }"))
    }
}

fn static_files(req: HttpRequest) -> HttpResponse{
    let path: String = format!("static/{}", req.match_info().query("name"));

    if let Ok(mut file) = File::open(path) {
        let mut data = Vec::new();
        let _ = file.read_to_end(&mut data);
        HttpResponse::Ok().body(data)
    } else {
        HttpResponse::NoContent()
            .body("File did not generate correctly")
    }
}

pub fn index() -> HttpResponse {
    let mut html = String::new();
    html.push_str(HEADER);
    //WHAT WE NEED HERE!
    html.push_str(FOOTER);
    HttpResponse::Ok().body(html)
}

pub fn register_routes(app: &mut ServiceConfig) {
    app.route("/", web::get().to(index));
    app.route("/u", web::post().to(make_bin));
    app.route("/c/{id:.*}", web::post().to(make_comm));
    app.route("/v/{id:.*}", web::get().to(get_bin));
    app.route("/static/{name:.*}", web::get().to(static_files));
}

