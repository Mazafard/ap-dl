use reqwest::header::{HeaderMap, ACCEPT, RANGE, REFERER, USER_AGENT};
use reqwest::Client;
use std::time::Duration;

pub fn build_http_client() -> Result<Client, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".parse().unwrap(),
    );
    headers.insert(REFERER, "https://www.aparat.com/".parse().unwrap());
    headers.insert("Origin", "https://www.aparat.com".parse().unwrap());
    headers.insert(ACCEPT, "*/*".parse().unwrap());

    Client::builder()
        .default_headers(headers)
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(45))
        .tcp_nodelay(true)
        .pool_max_idle_per_host(4)
        .build()
}

pub async fn probe_range_support(client: &Client, candidates: &[String]) -> Option<(u64, bool)> {
    for mirror_url in candidates {
        match client
            .get(mirror_url)
            .header(RANGE, "bytes=0-0")
            .send()
            .await
        {
            Ok(resp) => {
                let status = resp.status();
                if status == reqwest::StatusCode::PARTIAL_CONTENT || status == reqwest::StatusCode::OK {
                    if let Some(cr) = resp.headers().get("content-range") {
                        if let Ok(cr_str) = cr.to_str() {
                            if let Some(total_str) = cr_str.split('/').last() {
                                if let Ok(total) = total_str.trim().parse::<u64>() {
                                    return Some((total, true));
                                }
                            }
                        }
                    }
                    if let Some(cl) = resp.headers().get("content-length") {
                        if let Ok(cl_str) = cl.to_str() {
                            if let Ok(total) = cl_str.trim().parse::<u64>() {
                                return Some((total, status == reqwest::StatusCode::PARTIAL_CONTENT));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                log::warn!("Range probe to mirror {} failed: {}, trying next...", mirror_url, e);
            }
        }
    }
    None
}
