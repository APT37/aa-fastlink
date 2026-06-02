use std::{env::var, net::IpAddr, sync::LazyLock};
use warp::{Filter, body, get, path, post, serve};

static SECRET: LazyLock<String> =
    LazyLock::new(|| var("AA_SECRET").expect("environment variable AA_SECRET"));

static DOMAIN: LazyLock<String> =
    LazyLock::new(|| var("AA_DOMAIN").expect("environment variable AA_DOMAIN"));

static BIND_IP: LazyLock<IpAddr> = LazyLock::new(|| {
    var("AA_BIND_IP")
        .unwrap_or_else(|_| "127.0.0.1".into())
        .parse()
        .expect("environment variable AA_BIND_IP must be a valid IP address")
});

static BIND_PORT: LazyLock<u16> = LazyLock::new(|| {
    var("AA_BIND_PORT")
        .unwrap_or_else(|_| "3030".into())
        .parse()
        .expect("environment variable AA_BIND_PORT must be a valid u16")
});

const MAX_BODY_SIZE: u64 = 128; // 96 works too

mod download;
mod form;

#[tokio::main]
async fn main() {
    // initialize env vars
    let _ = *SECRET;
    let _ = *DOMAIN;

    let routes = get().map(form::render_form).or(post()
        .and(body::content_length_limit(MAX_BODY_SIZE))
        .and(body::form())
        .and(path("dl"))
        .and_then(download::handle_download));

    println!("Server running on http://{}:{}", *BIND_IP, *BIND_PORT);
    serve(routes).run((*BIND_IP, *BIND_PORT)).await;
}
