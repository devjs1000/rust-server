use::std::net::TcpListener;
use::std::net::TcpStream;
use::std::io::prelude::*;
use::std::fs;



fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
     for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    
    }

}   // end of main

fn handle_connection(mut stream: TcpStream){
    let mut buffer= [0;512];
    stream.read(&mut buffer).unwrap();
    let contents = fs::read_to_string("src/welcome.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",contents.len(), contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
