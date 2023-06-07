//! This is the main file for this project. The execution
//! should start here, i.e. starting the server and the
//! request handling functions.

#![allow(non_snake_case)] // make rust-analyzer stop yelling at me

mod schema;
mod types;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({});
    let body = hb.render("index", &data).unwrap(); // TODO not unwrap
    HttpResponse::Ok().body(body)
}

#[get("sayhello/{name}")]
async fn sayhello(hb: web::Data<Handlebars<'_>>, path: web::Path<(String)>) -> impl Responder {
    let name = path.into_inner();
    let data = json!(
        {
            "name": name
        }
    );

    let body = hb.render("sayhello", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print!("Starting server...");

    let mut hb = Handlebars::new();
    hb.register_templates_directory(".html", "./static/templates/").unwrap();
    let hb_ref = web::Data::new(hb);

    HttpServer::new(move || {
        App::new()
            .app_data(hb_ref.clone())
            .service(index)
            .service(sayhello)
            
    })
        .bind(("127.0.0.1", 8002))?
        .run()
        .await
}
