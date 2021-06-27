use std::{
    io::{self, BufReader, BufWriter},
    net::TcpStream,
    env::consts::*,
};

use crate::VERSION;
use crate::definitions::connection_state::ConnectionState;
use crate::definitions::uniform_resource_identifier::URI;


pub struct Connection {
    state: ConnectionState,
    i: BufReader<TcpStream>,
    o: BufWriter<TcpStream>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let input = BufReader::new(stream.try_clone()?);
        let output = BufWriter::new(stream);

        Ok(Self { state: ConnectionState::CONNECTED, i: input, o: output })
    }

    pub fn get(&mut self, uri: URI) {
        let query = format!("GET {} HTTP/1.1\
Host: {}\
User-Agent: Curio / {} {}", uri.path, uri.domain.host, VERSION, format!("{} {} {}", FAMILY, OS, ARCH));
        println!("{}", query);
    }
}