use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("server: main");

    // listen
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // request read
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // response
    let (status_line, html_template_path) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "bks/rsbk/ch20_web_server/ch20_1to3_webserver/web_content/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "bks/rsbk/ch20_web_server/ch20_1to3_webserver/web_content/404.html")
    };
    let contents = fs::read_to_string(html_template_path).unwrap();
    let length = contents.len();

    // write response
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

}
