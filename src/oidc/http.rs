use std::fs::File;
use std::io::Read;

use openidconnect::{
    reqwest::Error, 
    HttpRequest,
    HttpResponse,
};
pub use reqwest;


///
/// Asynchronous HTTP client.
///
pub async fn async_http_client(
    request: HttpRequest,
) -> Result<HttpResponse, Error<reqwest::Error>> {

    let mut client_builder = reqwest::Client::builder();

    if let Some(ca_cert_path) = std::env::var("CA_CERT_PATH").ok() {
        let mut buf = Vec::new();
        let mut file = File::open(ca_cert_path).unwrap();
        file.read_to_end(&mut buf).unwrap();

        let cert = reqwest::Certificate::from_pem(&buf).unwrap();

        client_builder = client_builder.use_native_tls().add_root_certificate(cert);

    }

    let client = client_builder.build().map_err(Error::Reqwest)?;

    let mut request_builder = client
        .request(request.method, request.url.as_str())
        .body(request.body);
    for (name, value) in &request.headers {
        request_builder = request_builder.header(name.as_str(), value.as_bytes());
    }
    let request = request_builder.build().map_err(Error::Reqwest)?;

    let result = client.execute(request).await;

    let response = result.map_err(Error::Reqwest)?;

    let status_code = response.status();
    let headers = response.headers().to_owned();
    let chunks = response.bytes().await.map_err(Error::Reqwest)?;
    
    Ok(HttpResponse {
        status_code,
        headers,
        body: chunks.to_vec(),
    })
}
