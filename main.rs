extern crate rand;
use std::env;
use std::str::from_utf8;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread;
use std::io;
mod lib;
use lib::*;

fn make_server() {
    let host = "localhost:8888";
    let listener = TcpListener::bind(host.to_string()).unwrap();
    println!("Listening...");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Add new client: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    let (mut hash, mut key, mut message) = ([0 as u8; 5], [0 as u8; 10], [0 as u8; 50]); 
                    while match stream.read(&mut hash) {
                        Ok(_) => {
                            stream.read(&mut key).unwrap();
                            stream.read(&mut message).unwrap();
                            let (received_hash, received_key) = (from_utf8(&hash).unwrap(), from_utf8(&key).unwrap());
                            let new_key = next_session_key(&received_hash, &received_key);
                            let result = new_key.clone().into_bytes();
                            stream.write(&result).unwrap();
                            stream.write(&message).unwrap();
                            true
                        },
                        Err(_) => {
                            println!("Connection error with {}", stream.peer_addr().unwrap());
                            stream.shutdown(Shutdown::Both).unwrap();
                            false
                        }
                    } {}
                });
            }   Err(err) => {println!("Error: {}", err);}
        }
    }   drop(listener);
}

fn make_client() {
    let host = "localhost:8888";
    let err_message = "Something went wrong";
    match TcpStream::connect(host) {
        Ok(mut stream) => {
            println!("Connection with server");
            let (mut data, mut rep) = ([0 as u8; 50], [0 as u8; 50]);
           loop {
               let (hash_str, session_key) = (get_hash_str(), get_session_key());
               let next_key = next_session_key(&hash_str, &session_key);
               let mut message = String::new();
               println!("Enter your message to server: ");
               io::stdin().read_line(&mut message).unwrap();
               stream.write(&hash_str.into_bytes()).unwrap();
               stream.write(&session_key.into_bytes()).unwrap();
               stream.write(&message.into_bytes()).unwrap();
               match stream.read(&mut data) {
                Ok(size) => {
                    stream.read(&mut rep).unwrap();
                       let (received_key, answer) = (from_utf8(&data[0..size]).unwrap(), from_utf8(&rep).unwrap());
                       if received_key == next_key {
                           println!("Your key: {}. Server key: {}", next_key, received_key);
                       }   else {break;}
                       println!("Answer: {}", answer);
                   }, Err(err) => {println!("{}: {}", err_message, err);}
               }
           }
        },  Err(err) => {println!("{}: {}", err_message, err);}
    }
}

fn main() {
    let parameters: Vec<String> = env::args().collect();
    let port = "8888";
    if parameters[1] == port {make_server();}
    else if parameters[1] == "127.0.0.1:8888" {
        for _i in 0..parameters[2].parse().unwrap() {make_client();}   
    }
}