use communicator::Communicator;
use chess::chess::Chess;
use std::io;
use std::process;
use std::env;

/// Helper function for parsing command line arguments
pub fn parse_arguments() -> (String, String, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let (prog_type, addr) = match &args[..] {
        [_, ref prog_type, ref addr] => (prog_type, addr),
        _ => {
            println!("Usage: {} <type> <bind address>", args[0]);
            process::exit(1);
        }
    };
    (prog_type.to_string(), addr.to_string(), args)
}

/// Creates a connection based on the type provided in command line arguments
pub fn connect(prog_type: &String, addr: &String, args: Vec<String>, game: &mut Chess) {
    match &prog_type[..] {
        "server" => {
            println!("Creating server");
            let mut server = Communicator::new(addr.clone());
            server.create_server().unwrap();
            game.start_server(&mut server);
        },
        "client" => {
            println!("Creating client");
            let mut client = Communicator::new(addr.clone());
            client.connect().unwrap();
            game.start_client(&mut client);
        },
        _ => println!("Usage: {} <type> <bind address>", args[0]),
    }
}