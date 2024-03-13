use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use bks_rsbk_ch20_webserver::ThreadPool;

fn main() {
    println!("server: main");

    // listen
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // request read
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // response
    let html_template_hello = "bks/rsbk/ch20_web_server/ch20_1to3_webserver/web_content/hello.html";
    let html_template_404 = "bks/rsbk/ch20_web_server/ch20_1to3_webserver/web_content/404.html";
    let (status_line, html_template_path) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", html_template_hello),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", html_template_hello)
        }
        _ => ("HTTP/1.1 404 NOT FOUND", html_template_404),
    };
    let contents = fs::read_to_string(html_template_path).unwrap();
    let length = contents.len();

    // write response
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

}
