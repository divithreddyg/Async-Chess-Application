use std::io;
use std::env;
use communicator::Communicator;
fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <type> <bind address>", args[0]);
        return;
    }

    let (prog_type, addr) = match &args[..] {
        [_, ref prog_type, ref addr] => (prog_type, addr),
        _ => unreachable!(),
    };
    
    if prog_type == "server" {
        let mut server = Communicator::new(addr.clone());
        server.create_server().unwrap();
        loop {
            let msg = server.recv().unwrap();
            println!("{}", msg);
            println!("Enter Message");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");
            server.send(input).unwrap();
        }
    } else if prog_type == "client" {
        println!("Creating client");
        let mut client = Communicator::new(addr.clone());
        client.connect().unwrap();
        loop {
            let msg = client.recv().unwrap();
            println!("{}", msg);
            let mut input = String::new();
            println!("Enter Message");
            io::stdin().read_line(&mut input).expect("Failed to read");
            client.send(input).unwrap();
            
        }
    } else {
        println!("Usage: {} <type> <bind address>", args[0]);
    }
}
