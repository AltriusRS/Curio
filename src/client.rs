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

    let connection = match req.protocol {
        HTTPProtocol::HTTP => build_connection(&req.protocol, &req.domain, &req.port, None),
        HTTPProtocol::HTTPS => {
            let config = Arc::new(build_tls_config());
            let dns_ref = DNSNameRef::try_from_ascii_str(req.domain.clone().as_str()).unwrap();
            let session = build_tls_session(&config, &dns_ref);
            build_connection(&req.protocol, &req.domain, &req.port, Some(session))
        }
    };

    let request = format!("GET {} HTTP/1.1\r\nUser-Agent: {}\r\nHost: {}\r\n\\r\n", req.path, req.user_agent, req.domain);
    return match req.protocol {
        HTTPProtocol::HTTPS => {
            Err(crate::types::err_from_code(301))
        },
        HTTPProtocol::HTTP => {
            println!("TCP: {:#?}\nTLS: is none? {}", connection.tcp, connection.tls.is_none());

            Err(crate::types::err_from_code(0))
        }
    }
}


fn build_connection<'a>(protocol: &HTTPProtocol, domain: &String, port: &usize, session: Option<ClientSession>) -> Connection<'a> {
    let address = parse_socket(&domain, &port);
    match protocol {
        HTTPProtocol::HTTP => {
            Connection {
                tcp: Some(TcpStream::connect_timeout(&address, Duration::from_secs(20)).unwrap()),
                tls: None
            }
        },
        HTTPProtocol::HTTPS => {
            Connection {
                tcp: None,
                tls: Some(rustls::Stream::new(&mut session.unwrap(), &mut TcpStream::connect_timeout(&address, Duration::from_secs(20)).unwrap()))
            }
        }
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