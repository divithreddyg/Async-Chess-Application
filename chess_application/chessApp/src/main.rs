use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use regex::{Match, Regex};
use chess::chess::Chess;
use communicator::Communicator;
use std::env;
use chessApp::chess_app;

fn main() {
    let SERVER_ADDRESS: String = String::from("127.0.0.1:7878");
    let addr = String::from("127.0.0.1:8080");
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage:{}  <bind address>", args[0]);
        return;
    }

    let addr = match &args[..] {
        [_, ref addr] => addr,
        _ => unreachable!(),
    };

    let mut server_communicator = Communicator::new(SERVER_ADDRESS.clone());
    match server_communicator.connect() {
        Ok(_) => {
            println!("Connected to Server on address {} successfully", SERVER_ADDRESS);
        }
        Err(e) => {
            println!("Failed to connect to Server {}", SERVER_ADDRESS);
            println!("{}", e);
            return;
        }
    }

    match TcpListener::bind(addr.clone()) {
        Ok(listener) => {
            println!("Started Web server on {}", addr);
            let mut chess_app = chess_app::new(server_communicator, listener);
            chess_app.run();
        }
        Err(e) => {
            println!("Failed to start server on {}", addr);
            println!("{}", e);
            return;
        }
    };

}
