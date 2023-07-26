use tera::Tera;
use tera::Context;
use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;

async fn greet() -> Html<String>  {
    let tera = match Tera::new("templates/*.html") {
    Ok(t) => t,
    Err(e) => {
        println!("Parsing error(s): {}", e);
        ::std::process::exit(1);
        }
    };

    let mut context = Context::new();
    context.insert("name", &"World");
    
    Html(tera.render("example.html", &context).unwrap())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(greet));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
