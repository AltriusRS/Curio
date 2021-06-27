use std::{
    io::{self, BufReader, BufWriter},
    net::TcpStream,
};
use crate::definitions::connection_state::ConnectionState;


pub struct Connection {
    state: ConnectionState,
    i: BufReader<TcpStream>,
    o: BufWriter<TcpStream>,
}

impl Connection {
    fn new(stream: TcpStream) -> io::Result<Self> {
        let input = BufReader::new(stream.try_clone()?);
        let output = BufWriter::new(stream);

        Ok(Self { state: ConnectionState::CONNECTED, i: input, o: output })
    }

    // fn get<A: Into<String>>(path: )
}