use bks_rsbk_ch20_1to3::ThreadPool;
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    println!("server: main");

    // listen
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("server: shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    // request read
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let req_get_root = b"GET / HTTP/1.1\r\n";
    let req_get_sleep = b"GET /sleep HTTP/1.1\r\n";

    // response build
    let html_template_hello = "bks/rsbk/ch20_web_server/ch20_1to3_webserver/web_content/hello.html";
    let html_template_404 = "bks/rsbk/ch20_web_server/ch20_1to3_webserver/web_content/404.html";
    let (status_line, html_template_path) = if buffer.starts_with(req_get_root) {
        ("HTTP/1.1 200 OK", html_template_hello)
    } else if buffer.starts_with(req_get_sleep) {
        // emulate long request
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", html_template_hello)
    } else {
        ("HTTP/1.1 404 NOT FOUND", html_template_404)
    };

    let contents = fs::read_to_string(html_template_path).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // write response
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
