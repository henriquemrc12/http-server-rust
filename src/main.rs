use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    if let Some(first_line) = http_request.get(0) {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() > 0 {
            let http_method = parts[0];
            make_response(http_method, stream);
        }
    }
}

fn make_response(http_method: &str, mut stream: TcpStream) {
    let mut response = String::new;

    if http_method == "GET" {
        let template_name = "list.html";
        let contents = get_template_string(template_name);
        let content_length = contents.len();
        let status_line = "HTTP/1.1 200 OK";
        response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}");
    }

    stream.write_all(response().as_bytes()).unwrap();
}


fn get_template_string(template_name: &str) -> &str {
    let template_string = fs::read_to_string(format!("templates/{template_name}"))
    let template: &str = template_string.expect("REASON").as_str();
    template;
}
