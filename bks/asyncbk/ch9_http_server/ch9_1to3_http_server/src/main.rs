use std::{
    fs,
    time::Duration,
};
use async_std::{
    task,
    task::spawn,
    prelude::*,
    net::{TcpListener, TcpStream},
};
use futures::stream::StreamExt;

#[async_std::main]
async fn main() {
    println!("server: main");

    // listen
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    listener.incoming().for_each_concurrent(None, |tcpstream| async move {
        let tcpstream = tcpstream.unwrap();
        spawn(handle_connection(tcpstream));
    }).await;

    println!("server: shutting down");
}

async fn handle_connection(mut stream: TcpStream) {
    // request read
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let req_get_root = b"GET / HTTP/1.1\r\n";
    let req_get_sleep = b"GET /sleep HTTP/1.1\r\n";
    let resp_header_ok = "HTTP/1.1 200 OK\r\n\r\n";
    let resp_header_err = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

    let html_template_hello = "bks/rsbk/ch20_web_server/ch20_1to3_webserver/web_content/hello.html";
    let html_template_404 = "bks/rsbk/ch20_web_server/ch20_1to3_webserver/web_content/404.html";
    
    // build response
    let (status_line, html_template_path) = if buffer.starts_with(req_get_root) {
        (resp_header_ok, html_template_hello)
    } else if buffer.starts_with(req_get_sleep) {
        // emulate long request
        task::sleep(Duration::from_secs(5)).await;
        (resp_header_ok, html_template_hello)
    } else {
        (resp_header_err, html_template_404)
    };

    let contents = fs::read_to_string(html_template_path).unwrap();
    let response = format!("{status_line}{contents}");

    // write response
    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
