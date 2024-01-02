use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..]);

    println!("Received request:\n{}", request);
    let response: &str = "HTTP/3 418 OK\r\nContent-Type: text/html\r\nIP: 223.206.246.70\r\nis_pedo: yes\r\n\r\n\
                    <!DOCTYPE html>\
                    <html>\
                    <head>\
                        <title>We Love Pun</title>\
                    </head>\
                    <body>\
                        <h1>We love pun</h1>\
                    </body>\
                    </html>";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:6969").expect("Failed to bind to address");

    println!("Server listening on 0.0.0.0:6969");

    for stream  in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
