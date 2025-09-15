use reqwest::Client;

pub fn build_http(timeout_ms: u64) -> Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build()
        .expect("reqwest client")
}
