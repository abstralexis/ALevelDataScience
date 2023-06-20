//! This is the main file for this project. The execution
//! should start here, i.e. starting the server and the
//! request handling functions.

#![allow(non_snake_case)] // make rust-analyzer stop yelling at me

mod schema;
mod types;

use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct SearchForm {
    location: String,
    year: u32,
}

#[post("search/submit")]
async fn search_submit(
    hb: web::Data<Handlebars<'_>>,
    form: web::Form<SearchForm>,
) -> impl Responder {
    web::Redirect::to(format!("/view/{0}/{1}", form.location, form.year)).see_other()
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({});
    let body = hb.render("index", &data).unwrap(); // TODO not unwrap
    HttpResponse::Ok().body(body)
}

#[get("sayhello/{name}")]
async fn sayhello(hb: web::Data<Handlebars<'_>>, path: web::Path<(String)>) -> impl Responder {
    let name = path.into_inner();
    let data = json!({ "name": name });

    let body = hb.render("sayhello", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("search")]
async fn search(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({});

    let body = hb.render("search", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("view/{location}/{year}")]
async fn view(hb: web::Data<Handlebars<'_>>, path: web::Path<(String, String)>) -> impl Responder {
    let (location, year) = path.into_inner();
    let data = json!(
        {
            "location": location,
            "year": year
        }
    );

    let body = hb.render("view", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print!("Starting server...");

    let mut hb = Handlebars::new();
    hb.register_templates_directory(".html", "./static/templates/")
        .unwrap();
    let hb_ref = web::Data::new(hb);

    HttpServer::new(move || {
        App::new()
            .app_data(hb_ref.clone())
            .service(index)
            .service(sayhello)
            .service(view)
            .service(search)
            .service(search_submit)
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
