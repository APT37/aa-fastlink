use std::{env::var, net::IpAddr, sync::LazyLock};

// secret account key, used for API authentication
pub(crate) static SECRET: LazyLock<String> =
    LazyLock::new(|| var("AA_SECRET").expect("environment variable AA_SECRET"));

// mirror domain, e.g. 'annas-archive.org'
pub(crate) static DOMAIN: LazyLock<String> =
    LazyLock::new(|| var("AA_DOMAIN").expect("environment variable AA_DOMAIN"));

// sets the bind IP address. defaults to '127.0.0.1'
pub(crate) static BIND_IP: LazyLock<IpAddr> = LazyLock::new(|| {
    var("AA_BIND_IP")
        .unwrap_or_else(|_| "127.0.0.1".into())
        .parse()
        .expect("environment variable AA_BIND_IP must be a valid IP address")
});

// sets the bind port. defaults to '3030'
pub(crate) static BIND_PORT: LazyLock<u16> = LazyLock::new(|| {
    var("AA_BIND_PORT")
        .unwrap_or_else(|_| "3030".into())
        .parse()
        .expect("environment variable AA_BIND_PORT must be a valid u16")
});

// enables debug logging. defaults to 'true' when using 'debug' profile.
pub(crate) static DEBUG_LOGGING: LazyLock<bool> = LazyLock::new(|| {
    cfg!(debug_assertions)
        || var("AA_DEBUG_LOGGING")
            .unwrap_or_else(|_| "false".into())
            .parse()
            .expect("environment variable AA_DEBUG_LOGGING must be a valid boolean")
});

// limits max bytes when receiving book URL or hash.
// increase when using a longer mirror URL than 'annas-archive.org'
pub(crate) const MAX_BODY_SIZE: u64 = 96;
