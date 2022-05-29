use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/sum", web::post().to(post_index))
    });

    println!("serving on http://localhost:3003");
    server
    .bind("127.0.0.1:3003")
    .expect("error binding server to address")
    .run()
    .expect("error running server");
}

fn get_index() -> HttpResponse {
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

fn post_index(form: web::Form<FormParams>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body("Computing the GCD with zero is boring!!!")
    }

    return HttpResponse::Ok().content_type("text/html").body("This post endpoint is working");
}