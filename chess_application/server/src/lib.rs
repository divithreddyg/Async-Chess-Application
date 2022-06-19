use std::time::Duration;
use chess::chess::Chess;
use futures::channel::mpsc;
use crate::mpsc::{UnboundedSender};


use async_std::prelude::*;
use async_std::{
    future,
    future::TimeoutError,
    io::{BufReader, Lines},
    net::{SocketAddr, TcpListener, TcpStream},
    task,
};


type BoxErr = Box<dyn std::error::Error + Send + Sync>;
type ResultBoxErr<T> = Result<T, BoxErr>;

type Channel = mpsc::UnboundedSender<String>;


use std::io;
#[derive(Debug)]
struct PeerConnection {
    peer_addr: SocketAddr,
    reader_lines: Lines<BufReader<TcpStream>>,
    writer: TcpStream,
}
#[derive(Debug, thiserror::Error)]
#[error("disconnected while reading during {phase} phase")]
struct ReadDisconnectError {
    phase: &'static str,
}
#[derive(Debug, thiserror::Error)]
#[error("disconnected while writing during {phase} phase")]
struct WriteDisconnectError {
    phase: &'static str,
}
#[derive(Debug, thiserror::Error)]
#[error("{err}")]
struct PeerConnectionError {
    err: BoxErr,
}
impl PeerConnection {
    fn new(stream: TcpStream) -> ResultBoxErr<Self> {
        let peer_addr = stream.peer_addr()?;
        let reader_lines = BufReader::new(stream.clone()).lines();
        let writer = stream;
        Ok(PeerConnection {
            peer_addr,
            writer,
            reader_lines,
        })
    }

    fn clone(&self) -> ResultBoxErr<Self> {
        Self::new(self.writer.clone())
    }

    fn error(&self, err: BoxErr) -> BoxErr {
        PeerConnectionError { err }.into()
    }
    fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }

    async fn next_line(&mut self, phase: &'static str) -> ResultBoxErr<String> {
        println!("in next_line");
        Ok(match self.reader_lines.next().await {
            Some(line) => String::from(line?.trim()),
            None => return Err(self.error(ReadDisconnectError { phase }.into())),
        })
    }
    fn write_str<'a, 'b, 'r>(
        &'a mut self,
        msg: &'b str,
        phase: &'static str,
    ) -> impl Future<Output = ResultBoxErr<()>> + 'r
    where
        'a: 'r,
        'b: 'r,
    {
        async move {
            self.writer
                .write_all(msg.as_bytes())
                .await
                .map_err(|_| self.error(WriteDisconnectError { phase }.into()))
        }
    }
    async fn writeln_str(&mut self, msg: &str, phase: &'static str) -> ResultBoxErr<()> {
        self.write_str(&format!("{}\n", msg), phase).await
    }
    async fn newline(&mut self, phase: &'static str) -> ResultBoxErr<()> {
        self.write_str("\n", phase).await
    }

}

    pub async fn server(addr:&str) -> ResultBoxErr<()> {
        let listener = TcpListener::bind(addr).await?;
        let mut incoming = listener.incoming();
        let (req_channel_tx, request_channel_rx) = mpsc::unbounded();
        let _broker = task::spawn(broker_loop(request_channel_rx));
        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            let mut peer_connection = PeerConnection::new(stream)?;
            let peer_addr = peer_connection.peer_addr();
            // : (UnboundedSender<String>, UnboundedReceiver<String>)
            let clone_ = req_channel_tx.clone();
            println!("server:: Accepted {}", peer_addr);
            let welcome = async move {
                welcome(&mut peer_connection, clone_).await;
            };
            task::Builder::new()
                .name("new_game".to_string())
                .spawn(welcome);
        }
        Ok(())
    }

    async fn welcome(peer_connection: &mut PeerConnection, request_channel: UnboundedSender<PeerConnection>) {
            println!("Waiting for play");
            let matcher = String::from("START_NEW_GAME");
            match peer_connection.next_line("play").await {
                Ok(matcher) => {
                    println!("{}", matcher);
                    play_loop(peer_connection, request_channel).await;
                },
                _ => {println!("Did not receive play");
                }
            }
    }

    async fn play_loop(peer_connection: &mut PeerConnection, request_channel_tx: UnboundedSender<PeerConnection>) {
        request_channel_tx.unbounded_send(peer_connection.clone().unwrap());
    }

    async fn broker_loop(mut request_channel_rx: mpsc::UnboundedReceiver<PeerConnection>) {
        let mut potential_opponent = None;
        while let Some(request) = request_channel_rx.next().await {
        match request {
            peer_connection => {
                match potential_opponent {
                    Some(opponent_conn) => {
                        potential_opponent = None;
                        task::spawn(async move {
                            start_game(peer_connection, opponent_conn).await;
                        });
                    }
                    _ => {
                        potential_opponent = Some(peer_connection);
                    }
                }
            }
        }
    }
    }

async fn start_game(mut peerconn1: PeerConnection, mut peerconn2: PeerConnection) {
    peerconn1.writeln_str("White\n", "play").await;
    peerconn2.writeln_str("Black\n", "play").await;

    let mut chess = Chess::new("White".to_string(), "Black".to_string());
    loop {
        loop {
            let mut next_line = peerconn1.next_line("play").await.unwrap();
            let mut p1_move = next_line.split_whitespace();
            let source_r = p1_move.next().unwrap().parse::<isize>().unwrap();
            let source_c = p1_move.next().unwrap().parse::<isize>().unwrap();
            let destination_r = p1_move.next().unwrap().parse::<isize>().unwrap();
            let destination_c = p1_move.next().unwrap().parse::<isize>().unwrap();

            match chess.remote_move((source_r, source_c), (destination_r, destination_c)) {
                Ok(board) => {
                    peerconn1.writeln_str(&board, "play").await;
                    peerconn2.writeln_str(&board, "play").await;
                    break;
                }
                Err(_) => {
                    println!("Error: Play again");
                }
            };
        }

        loop {
            let mut next_line = peerconn2.next_line("play").await.unwrap();
            let mut p1_move = next_line.split_whitespace();
            let source_r = p1_move.next().unwrap().parse::<isize>().unwrap();
            let source_c = p1_move.next().unwrap().parse::<isize>().unwrap();
            let destination_r = p1_move.next().unwrap().parse::<isize>().unwrap();
            let destination_c = p1_move.next().unwrap().parse::<isize>().unwrap();

            match chess.remote_move((source_r, source_c), (destination_r, destination_c)) {
                Ok(board) => {
                    peerconn1.writeln_str(&board, "play").await;
                    peerconn2.writeln_str(&board, "play").await;
                    break;
                }
                Err(_) => {
                    println!("Error: Play again");
                }
            };
        }
    }
}



