use warp::{Filter, body, get, path, post, serve};

mod config;
mod download;
mod form;

#[tokio::main]
async fn main() {
    // import configuration into main scope
    use config::{BIND_IP, BIND_PORT, DOMAIN, MAX_BODY_SIZE, SECRET};

    // ensure environment variables are set
    let _ = *SECRET;
    let _ = *DOMAIN;

    // define download form route and download redirection routes
    let routes = get().map(form::render_form).or(post()
        .and(body::content_length_limit(MAX_BODY_SIZE))
        .and(body::form())
        .and(path("dl"))
        .and_then(download::handle_download));

    // print version and bind address
    println!("Running aa-fastlink {}", env!("CARGO_PKG_RUST_VERSION"));
    println!("Binding to http://{}:{}", *BIND_IP, *BIND_PORT);

    // start server
    serve(routes).run((*BIND_IP, *BIND_PORT)).await;
}
