mod app;
mod handlers;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io::{self, Read};
use std::path::PathBuf;

use app::{AppState, CliApp};
use handlers::{AppInput, AppOutput};

async fn index(body: web::Json<AppInput>, data: web::Data<AppState>) -> impl Responder {
    let mut app = CliApp::new(&data.cli_path);
    let path = body
        .arguments
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let curdir = &body.working_directory;
    let env_vars = &body.env_vars;
    let result = app.execute(&path, curdir, env_vars).unwrap();
    let out = AppOutput::new(result.stdout, result.stderr, result.status.success());
    HttpResponse::Ok().json(out)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://127.0.0.1:8080");
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Error reading from STDIN");
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                cli_path: PathBuf::from(&buffer.trim()),
            })
            .route("/", web::post().to(index))
    })
    .workers(2)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
