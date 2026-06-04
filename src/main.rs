// declare modules
mod config;
mod download;
mod form;

#[tokio::main]
async fn main() {
    // import configuration constants
    use config::{BIND_IP, BIND_PORT, DOMAIN, MAX_BODY_SIZE, SECRET};

    // import warp items
    use warp::{Filter, body, get, path, post, serve};

    // ensure environment variables are set
    let _ = *SECRET;
    let _ = *DOMAIN;

    // define download form route and download redirection routes
    let routes = get().map(form::render_form).or(post()
        .and(body::content_length_limit(MAX_BODY_SIZE))
        .and(body::form())
        .and(path("dl"))
        .and_then(download::handle_download));

    // initialize logger
    init_logger();

    // print version and bind address
    log::info!("Running aa-fastlink {}", env!("CARGO_PKG_VERSION"));
    log::info!("Binding to http://{}:{}", *BIND_IP, *BIND_PORT);

    // start server
    serve(routes).run((*BIND_IP, *BIND_PORT)).await;
}

// initializes logger with custom settings
pub fn init_logger() {
    use std::io::Write;

    let mut builder = env_logger::Builder::new();

    builder.format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()));

    builder.filter_level(if *config::DEBUG_LOGGING {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    });

    builder.init();
}
