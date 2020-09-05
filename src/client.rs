use crate::structs::{Connection, Response, Request, HTTPProtocol};
use crate::types::Result as CurioResult;
use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr, ToSocketAddrs};
use webpki::DNSNameRef;
use std::sync::Arc;
use rustls::{ClientSession, ClientConfig};
use webpki_roots::TLS_SERVER_ROOTS;
use std::str::FromStr;
use std::time::Duration;

pub fn get(req: &Request) -> CurioResult<Response> {
    let config = Arc::new(build_tls_config());
    let dns_ref = DNSNameRef::try_from_ascii_str(&req.domain.as_str()).unwrap();
    let mut session = build_tls_session(&config, &dns_ref);
    let mut tcp = build_tcp_stream(&req.domain, &req.port);
    let connection = match req.protocol {
        HTTPProtocol::HTTP => build_tcp_connection(&mut tcp),
        HTTPProtocol::HTTPS => {
            build_tls_connection(&req.protocol, &mut tcp, &mut session)
        }
    };

    let request = format!("GET {} HTTP/1.1\r\nUser-Agent: {}\r\nHost: {}\r\n\\r\n", req.path, req.user_agent, req.domain);
    return match req.protocol {
        HTTPProtocol::HTTPS => {
            connection.write(request);
            Err(crate::types::err_from_code(301))
        }
        HTTPProtocol::HTTP => {
            connection.write(request);
            Err(crate::types::err_from_code(0))
        }
    };
}


fn build_tcp_stream(domain: &String, port: &usize) -> TcpStream {
    let address = parse_socket(&domain, &port);
    TcpStream::connect_timeout(&address, Duration::from_secs(20)).unwrap()
}

fn build_tcp_connection(stream: &mut TcpStream) -> Connection {
    Connection {
        tcp: Some(stream),
        tls: None,
    }
}

fn build_tls_connection<'a>(protocol: &HTTPProtocol, stream: &'a mut TcpStream, session: &'a mut ClientSession) -> Connection<'a> {
    Connection {
        tcp: None,
        tls: Some(rustls::Stream::new(session, stream)),
    }
}

fn parse_socket(domain: &String, port: &usize) -> SocketAddr {
    let info = format!("{}:{}", domain, port);
    let addresses = info.to_socket_addrs().unwrap();
    println!("{:#?}", addresses.clone().last().unwrap());
    return addresses.last().unwrap();
}

fn build_tls_session(session: &Arc<ClientConfig>, dns_ref: &DNSNameRef) -> ClientSession {
    ClientSession::new(session, dns_ref.clone())
}

fn build_tls_config() -> rustls::ClientConfig {
    let mut cfg = rustls::ClientConfig::new();
    cfg.root_store.add_server_trust_anchors(&TLS_SERVER_ROOTS);
    cfg
}