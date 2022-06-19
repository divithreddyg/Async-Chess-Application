use communicator::Communicator;
use chess::chess::Chess;
use std::io;
use std::process;
use std::env;

fn main() {
    let (prog_type, addr, args) = user_interface::parse_arguments();
    let mut game = Chess::new(String::from("Player-1"), String::from("Player-2"));
    user_interface::connect(&prog_type, &addr, args, &mut game)
}



