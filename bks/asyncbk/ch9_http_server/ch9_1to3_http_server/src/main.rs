use std::{
    env, fs, time::Duration
};
use async_std::{
    task,
    task::spawn,
    prelude::*,
    net::TcpListener,
};
use futures::stream::StreamExt;
use async_std::io::{Read, Write};

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

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    // request read
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let req_get_root = b"GET / HTTP/1.1\r\n";
    let req_get_sleep = b"GET /sleep HTTP/1.1\r\n";
    let resp_header_ok = "HTTP/1.1 200 OK\r\n\r\n";
    let resp_header_err = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

    
    let html_template_hello = fix_path("web_content/hello.html");
    let html_template_404 = fix_path("web_content/404.html");   

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

#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::Error;
    use futures::task::{Context, Poll};
    use std::cmp::min;
    use std::pin::Pin;
    use std::fs;

    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }
    
    impl Read for MockTcpStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _: &mut Context,
            buf: &mut [u8],
        ) -> Poll<Result<usize, Error>> {
            let size: usize = min(self.read_data.len(), buf.len());
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size))
        }
    }
    
    impl Write for MockTcpStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context,
            buf: &[u8],
        ) -> Poll<Result<usize, Error>> {
            self.write_data = Vec::from(buf);
    
            Poll::Ready(Ok(buf.len()))
        }
    
        fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
    
        fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
    }
    
    impl Unpin for MockTcpStream {}

    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n\r\n";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        handle_connection(&mut stream).await;

        let expected_contents = fs::read_to_string("web_content/hello.html").unwrap();
        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
        assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }
}

// temporary solution for fix issue with cargo run and cargo test
fn fix_path(p: &str) -> String {
    let curr_dir = env::current_dir().unwrap();
    if !curr_dir.ends_with("ch9_1to3_http_server") {
        let pkg_path = String::from("bks/rsbk/ch20_web_server/ch20_1to3_webserver");
        let out = format!("{}/{}", pkg_path, p);
        return out
    }
    let out = String::from(p);
    out
}
