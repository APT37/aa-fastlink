use crate::config;
use log::{debug, error, warn};
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fmt, str::FromStr};
use warp::{Reply, http::Uri, redirect, reject::Rejection, reply::Response};

fn format_error(error: impl Error) -> String {
    match error.source() {
        Some(source) => format!("{error}: {source}"),
        None => error.to_string(),
    }
}

pub(super) async fn handle_download(form: HashMap<String, String>) -> Result<Response, Rejection> {
    fn is_valid_link_or_hash(link_or_hash: &str) -> bool {
        link_or_hash.len() >= 32 && link_or_hash.is_ascii() && !link_or_hash.contains(' ')
    }

    if let Some(link_or_hash) = form.get("link") {
        if is_valid_link_or_hash(link_or_hash) {
            debug!(
                "received query for [{link_or_hash}] ({})",
                link_or_hash.len()
            );

            let hash = link_or_hash[link_or_hash.len() - 32..].to_string(); // needed?

            match get_fast_download_link(&hash).await {
                Ok(fast_download_link) => Ok(redirect::see_other(
                    Uri::from_str(&fast_download_link).unwrap(),
                )
                .into_response()),

                Err(error) => Ok({
                    let err_msg = format_error(&*error);
                    error!("{err_msg}");
                    err_msg.into_response()
                }),
            }
        } else {
            let err_msg = "invalid book URL or hash provided";
            warn!("{err_msg} [{link_or_hash}]");
            Ok(err_msg.into_response())
        }
    } else {
        warn!("form submitted without input");
        Ok("no book URL or hash provided".into_response())
    }
}

#[derive(Debug, Clone, Deserialize)]
struct ApiResponse {
    download_url: Option<String>,
    error: Option<String>,
}

#[derive(Debug)]
struct FetchError {
    message: String,
}

impl FetchError {
    #[allow(clippy::unnecessary_box_returns)]
    fn new(error: &str) -> Box<Self> {
        Box::new(Self {
            message: error.to_string(),
        })
    }
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DownloadError: {}", self.message)
    }
}

impl Error for FetchError {}

async fn get_fast_download_link(hash: &str) -> Result<String, Box<dyn Error>> {
    debug!("getting fast_download_link for [{hash}] ({})", hash.len());

    let response_result = reqwest::get(format!(
        "https://{}/dyn/api/fast_download.json?md5={}&key={}",
        *config::DOMAIN,
        hash,
        *config::SECRET
    ))
    .await;
    debug!("{response_result:#?}");

    let json_result = response_result?.json().await;
    debug!("{json_result:#?}");

    let api_response: ApiResponse = json_result?;
    debug!("{api_response:#?}");

    if let Some(fast_download_link) = api_response.download_url {
        Ok(fast_download_link)
    } else {
        let err_msg = api_response
            .error
            .map_or("unknown error".to_string(), |err| format!("{err} [{hash}]"));
        Err(FetchError::new(&err_msg))
    }
}
