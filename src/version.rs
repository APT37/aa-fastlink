use warp::reply::Reply;

// returns package version as HTML
pub(super) fn plain() -> impl Reply {
    env!("CARGO_PKG_VERSION")
}
