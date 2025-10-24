#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod files_listing;
mod parser;
mod resource;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, post};
use clap::Parser;
use env_logger::Env;
use parser::Arguments;
use std::io;
use tokio::process::Command;

#[post("/")]
async fn shutdown_server() -> impl Responder {
    let _ = Command::new("shutdown.exe")
        .args(&["/s", "/t", "0"])
        .spawn();
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let arguments = Arguments::parse();

    HttpServer::new(move || {
        let list_files = Files::new("/", &arguments.directory)
            .show_files_listing()
            .files_listing_renderer(files_listing::render);

        App::new()
            .wrap(Cors::permissive())
            .service(shutdown_server)
            .service(list_files)
    })
    .bind((arguments.ip_address, arguments.port))?
    .run()
    .await
}
