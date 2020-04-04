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
        ", res.dumpid, res.lang, res.text).as_ref());
        html.push_str(FOOTER);
        HttpResponse::Ok().body(html)
    } else {
        html.push_str("<pre><code>Invalid Id</code></pre>");
        html.push_str(FOOTER);
        HttpResponse::Ok().body(html)
    }
}

async fn make_bin(mut obj: web::Json<Dump>) -> Result<HttpResponse> {
    println!("{:?}", obj);
    let mut rng = rand::thread_rng();
    let id : u32 = rng.gen();
    obj.dumpid = id;
    if let Ok(_) = commit_source(&establish(), &obj.into_inner()) {
        Ok(HttpResponse::Ok().json(format!("{{'success' : 1, 'id' : {} }}", id)))
    } else {
        Ok(HttpResponse::Ok().json("{ 'success' : 0 }"))
    }
}

async fn make_comm(obj: web::Json<Comment>) -> Result<HttpResponse> {
    if let Ok(_) = commit_comment(&establish(), &obj.into_inner()) {
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
    html.push_str("
<div class='vid'>rebin - help</div>
<pre><code class='language-plaintext'>
    This is place to dump some code and share it with others!

    How to upload?
        You can upload a file by using the following json object (set id as 0)
        (comments are currently not supported yet)
        ```
        {
            'dumpid' : 0,
            'username' : 'username you want',
            'text' : 'source code',
            'lang' : 'programming language',
            'comments' : [],
            'timestamp' : 0
        }
        ```

    How do I retrieve a source file?
        Once you have uploaded a file you can retrieve it by using its URL:

        http://<hostname>/v/<id>

    When you upload a file, the server will reply with an ID of the source
        ```
        {
            'success': 1,
            'id' : 0
        }
        ```
</code></pre>");
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

