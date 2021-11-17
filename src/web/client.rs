//! web/client.rs --- Web HTTP Client
/*!
Generic HTTP/S Client bound to a socket. The 'WebClientConfig'
controls various transport parameters like timeouts, socket range,
auth, etc.
*/
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;

/// spawn an unmanaged HTTP/S client handle. The socket is provided by
/// syscall and requests need to be built, sent, and handled by
/// programmer.
pub async fn void_client() -> Client<HttpsConnector<HttpConnector>> {
  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, hyper::Body>(https);
  client
}
