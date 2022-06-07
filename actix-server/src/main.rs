use actix_web::{get, post, web, App, Responder, HttpResponse, HttpServer};
use serde::Deserialize;
use std::env;
use text_colorizer::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let args = parse_args();
    let server = HttpServer::new(|| {
        App::new()
            .service(get_index)
            .service(post_sum)
    });

    println!("serving on http://localhost:{}", args.port.green());
    let server_url = format!("127.0.0.1:{}", args.port);
    server
    .bind(server_url)?
    .run()
    .await
}

#[get("/")]
async fn get_index() -> impl Responder {
    return HttpResponse::Ok()
    .content_type("text/html")
    .body(r#"
    <h3>Calculator</h3>
    <form action="/sum" method="post">
    <input type='number' name='n' />
    <input type='number' name='m' />
    <button type='submit'>compute</button>
    </form>
    "#);
}

#[derive(Deserialize)]
struct FormParams {
    n: u64,
    m: u64,
}

#[post("/sum")]
async fn post_sum(form: web::Form<FormParams>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body("Computing the GCD with zero is boring!!!")
    }

    return HttpResponse::Ok().content_type("text/html").body("This post endpoint is working");
}

struct Arguments {
    port: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 {
        println!("please pass the {}", "port".red());
        std::process::exit(1)
    }

    Arguments {
        port: args[0].clone()
    }
}