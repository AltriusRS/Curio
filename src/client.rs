use crate::structs::{Connection, Response, Request, HTTPProtocol};
use crate::types::{Result as CurioResult, err_from_code};
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use webpki::DNSNameRef;
use std::sync::Arc;
use rustls::{ClientSession, ClientConfig};
use webpki_roots::TLS_SERVER_ROOTS;
use std::time::Duration;
use std::process::exit;

pub fn get(req: &mut Request, upgraded: bool) -> CurioResult<Response> {
    let config = Arc::new(build_tls_config());
    let dns_ref = DNSNameRef::try_from_ascii_str(&req.domain.as_str()).unwrap();
    let mut session = build_tls_session(&config, &dns_ref);
    let mut tcp = build_tcp_stream(&req.domain, &req.port);
    let mut connection = match req.protocol {
        HTTPProtocol::HTTP => build_tcp_connection(&mut tcp),
        HTTPProtocol::HTTPS => {
            build_tls_connection(&mut tcp, &mut session)
        }
    };

    if upgraded {
        println!("Upgraded request, building new connection");
        connection = build_tls_connection(&mut tcp, &mut session);
    }


    let request = format!("GET {} HTTP/1.1\r\nUser-Agent: {}\r\nHost: {}\r\nConnection: close\r\n\r\n", req.path, req.user_agent, req.domain);
    if upgraded {
        println!("UPGRADE:\n{}", request.clone());
    }
    connection = connection.write(request);
    if upgraded {
        exit(101)
    }
    let (mut head, connection) = connection.read_head();

    return if head.status.unwrap() == 301 && head.headers.get("Location").unwrap().contains("https://") {
        if upgraded {
            Err(err_from_code(9)) // too many upgrades error (not implemented)
        } else {
            //get(req, true) // attempt to re-run the request using TLS to upgrade the connection
            Err(err_from_code(301))
        }
    } else {
        if head.status.unwrap() >= 400 {
            Err(err_from_code(head.status.unwrap() as u16))
        } else if head.headers.get("content-type").unwrap().contains("charset=utf-8") {
            Ok(connection.read_body_string(&mut head)) // attempt to read the content body as plaintext into a string
        } else {
            Err(err_from_code(10)) // unsupported content type
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

fn build_tls_connection<'a>(stream: &'a mut TcpStream, session: &'a mut ClientSession) -> Connection<'a> {
    Connection {
        tcp: None,
        tls: Some(rustls::Stream::new(session, stream)),
    }
}

fn parse_socket(domain: &String, port: &usize) -> SocketAddr {
    let info = format!("{}:{}", domain, port);
    let addresses = info.to_socket_addrs().unwrap();
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