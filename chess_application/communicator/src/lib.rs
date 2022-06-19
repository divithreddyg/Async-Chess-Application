use std::io;
use std ::net::{TcpListener,TcpStream};
use std::io::{Read,Write};

pub struct Communicator {
    addr : String,
    stream : Option<TcpStream>,
}

impl Communicator {
    pub fn new(addr : String) -> Communicator {
        Communicator {
            addr,
            stream : None,
        }
    }
    
    pub fn create_server(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind(self.addr.as_str())?;
        println!("Started server on {}", self.addr);
        let stream = listener.accept()?;
        println!("Accepted connection from {}", stream.0.peer_addr()?);
        self.stream = Some(stream.0);
        Ok(())
    }

    pub fn connect(&mut self) -> io::Result<()> {
        println!("Connecting to {}", self.addr);
        let stream = TcpStream::connect(self.addr.as_str())?;
        println!("Connected to {}", stream.peer_addr()?);
        self.stream = Some(stream);
        Ok(())
    }

    pub fn send(&mut self, msg : String) -> io::Result<()> {
        let stream = self.stream.as_mut().unwrap();
        stream.write(msg.as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    pub fn recv(&mut self) -> io::Result<String> {
        let stream = self.stream.as_mut().unwrap();
        let mut buf = [0;2048];
        let bytes_read = stream.read(&mut buf)?;
        // if bytes_read == 0 {
        //     return Err(io::Error::new(io::ErrorKind::Other, "Connection closed"));
        // }
        let str = match String::from_utf8_lossy(&buf) {
            s if s.len() > 0 => s.to_string(),
            _ => "".to_string(),
        };
        Ok(str)
    }
}