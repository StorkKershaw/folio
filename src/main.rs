mod anchor;
mod page;
mod parser;

use clap::Parser;
use parser::Arguments;

use std::io;

use actix_files::Files;
use actix_web::{App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let arguments = Arguments::parse();

    HttpServer::new(move || {
        App::new().service(
            Files::new("/", &arguments.directory)
                .show_files_listing()
                .files_listing_renderer(page::render),
        )
    })
    .bind((arguments.ip_address, arguments.port))?
    .run()
    .await
}
