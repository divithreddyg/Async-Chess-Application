use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use regex::{Match, Regex};
use chess::chess::Chess;
use communicator::Communicator;
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
use std::io;


enum Error {
    PlayerNotFound,
    InvalidMove,
}
pub struct chess_app {
    server_communicator: Communicator,
    local_web_server: TcpListener,
}

#[derive(Serialize, Deserialize, Debug)]
struct gameColor {
    color: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct gameState {
    encoded_game_state: String,
}

impl chess_app {
    pub fn new(server_communicator: Communicator, local_web_server: TcpListener) -> chess_app {
        chess_app {
            server_communicator,
            local_web_server
        }
    }

    pub fn run(&mut self){
        let clone_listener = self.local_web_server.try_clone().unwrap();
        for stream in clone_listener.incoming() {
            match stream {
                Ok(stream) => {
                   self.handle_connection(stream);
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..]);
        println!("Request: {}", request);
        if request.contains("GET / HTTP/1.1") {
            let contents = fs::read_to_string("static/index.html").unwrap();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", 
                contents.len(),
                contents
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request.contains("GET /startNewGame HTTP/1.1") {
            let response = match self.startNewGame() {
                Ok(color) => {
                    let abc = gameColor {
                        color: color.trim().to_string(),
                    };
                    let json = serde_json::to_string(&abc).unwrap();
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", 
                        json.len(),
                        json
                    )
                },
                Err(e) => {
                    format!(
                        "HTTP/1.1 404 Not Found\r\n\r\n\r\n", 
                    )
                }
                
            };
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request.contains("GET /makeMove"){
            match self.make_move(request.to_string()){
                Err(e) => {
                    // println!("Error: {}", e);
                    stream.write(format!("HTTP/1.1 404 Invalid Move\r\n\r\n").as_bytes()).unwrap();
                    stream.flush().unwrap();
                },
                Ok(board) => {
                    let board = gameState {
                        encoded_game_state: board,
                    };
                    let json = serde_json::to_string(&board).unwrap();
                    let board = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        json.len(),
                        json
                    );
                    stream.write(board.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            };
        } else if request.contains("GET /getMove") {
            match self.get_move() {
                Err(e) => {
                    println!("Error: {}", e);
                    stream.write(format!("HTTP/1.1 404 Invalid Move\r\n\r\n").as_bytes()).unwrap();
                    stream.flush().unwrap();
                },
                Ok(board) => {
                    let board = gameState {
                        encoded_game_state: board,
                    };
                    let json = serde_json::to_string(&board).unwrap();
                    let board = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        json.len(),
                        json
                    );
                    stream.write(board.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            }
        }
    }

    fn get_move(&mut self) -> io::Result<String>{
        self.server_communicator.recv()
    }

    fn make_move(&mut self, request: String) -> io::Result<String> {
        let re = Regex::new(r"^.*source=(?P<source_r>[1-7])-(?P<source_c>[1-7])&destination=(?P<destination_r>[1-7])-(?P<destination_c>[1-7])").unwrap();
        let caps = re.captures(&request).unwrap();
        let source_r = caps.name("source_r").unwrap().as_str().parse::<isize>().unwrap();
        let source_c = caps.name("source_c").unwrap().as_str().parse::<isize>().unwrap();
        let destination_r = caps.name("destination_r").unwrap().as_str().parse::<isize>().unwrap();
        let destination_c = caps.name("destination_c").unwrap().as_str().parse::<isize>().unwrap();
        let move_str = format!("{} {} {} {}\n", source_r, source_c, destination_r, destination_c);
        self.server_communicator.send(move_str.to_string());
        self.server_communicator.recv()
    }

    fn startNewGame(&mut self) -> io::Result<String> {
        self.server_communicator.send(String::from("START_NEW_GAME\n"))?;
        let x = self.server_communicator.recv();
        return x;
    }
}