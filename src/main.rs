// use async_std;

// async fn say_hello() {
//     println!("Hello, world!");
// }

// #[async_std::main]
// async fn main() {
//     say_hello().await;
// }

use async_std::net::TcpStream;
use async_tls::TlsConnector;
use http_types::{Method, Request, Url};

#[async_std::main]
async fn main() -> http_types::Result<()> {
    let tcp_stream = TcpStream::connect("cn.bing.com:443").await?;
    let connector = TlsConnector::default();
    let tls_stream = connector.connect("cn.bing.com", tcp_stream.clone()).await?;

    let peer_addr = tcp_stream.peer_addr()?;
    println!("connecting to {}", peer_addr);
    let url = Url::parse(&format!("http://{}/", peer_addr))?;

    let mut req = Request::new(Method::Get, url);
    req.insert_header("version", "11");
    req.insert_header("target", "/");
    
    let res = async_h1::connect(tls_stream, req).await?;
    println!("{:?}", res);

    Ok(())
}
