use crate::config;
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fmt, str::FromStr};
use warp::{Reply, http::Uri, redirect, reject::Rejection, reply::Response};

pub(super) async fn handle_download(form: HashMap<String, String>) -> Result<Response, Rejection> {
    if let Some(link_or_hash) = form.get("link")
        && link_or_hash.len() >= 32
        && link_or_hash.is_ascii()
        && !link_or_hash.contains(' ')
    {
        if *config::DEBUG_LOGGING {
            eprintln!("[{link_or_hash}] ({})", link_or_hash.len());
        }

        let hash = link_or_hash[link_or_hash.len() - 32..].to_string();

        match get_fast_download_link(&hash).await {
            Ok(fast_download_link) => Ok(redirect::see_other(
                Uri::from_str(&fast_download_link).unwrap(),
            )
            .into_response()),
            Err(error) => Ok({
                let err_msg = format!(
                    "{error}{}",
                    if let Some(src) = error.source() {
                        format!(": {src}")
                    } else {
                        String::new()
                    }
                );

                eprintln!("{err_msg}");

                err_msg.into_response()
            }),
        }
    } else {
        Ok("invalid book URL or hash provided".into_response())
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Download {
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
    if *config::DEBUG_LOGGING {
        eprintln!("getting fast_download_link for [{hash}] ({})", hash.len());
    }

    let response: Download = reqwest::get(format!(
        "https://{}/dyn/api/fast_download.json?md5={}&key={}",
        *config::DOMAIN,
        hash,
        *config::SECRET
    ))
    .await?
    .json()
    .await?;

    if *config::DEBUG_LOGGING {
        eprintln!("{response:#?}");
    }

    if let Some(fast_download_link) = response.download_url {
        Ok(fast_download_link)
    } else {
        let error = response.error.unwrap_or("unknown error".to_string());

        eprintln!("{error} [{hash}]");

        Err(FetchError::new(&error))
    }
}
