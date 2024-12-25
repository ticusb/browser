use native_tls::{TlsConnector, TlsStream};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};

use crate::url::Url;

use crate::requestable::Requestable;

enum Connection {
    Http(TcpStream),
    Https(TlsStream<TcpStream>), // After TLS negotiation
}

struct SocketPool {
    connections: Mutex<HashMap<String, Connection>>,
    tls_connector: TlsConnector,
}

impl SocketPool {
    // Initialize a new SocketPool
    fn new(&self) -> Self {
        Self {
            connections: Mutex::new(HashMap::new()),
            tls_connector: TlsConnector::new().unwrap(),
        }
    }

    fn get_connection(&self, url: &Url) -> Result<Connection, io::Error> {
        let mut pool = self.connections.lock().unwrap();
        let address = url.address();

        if let Some(conn) = pool.get(address.as_str()) {
            return conn.try_clone();
        }
        
        let tcp_stream = TcpStream::connect(url.address())?;
        
        if url.scheme() == "https" {
            let url_host = address.split(":").next().unwrap();
            let tls_stream = self.tls_connector.connect(url_host, tcp_stream).unwrap();
            pool.insert(address.to_string(), Connection::Https(tls_stream));
            Ok(Connection::Https(&tls_stream))
        } else {
            pool.insert(address.to_string(), Connection::Http(tcp_stream));
            Ok(Connection::Http(&tcp_stream))
        }
    }

    // Release or close a specific connection
    fn release_connection(&self, addr: &str) {
        let mut pool = self.connections.lock().unwrap();
        pool.remove(addr);
    }
}

impl Connection {
    // Helper function to clone the connection, ensuring each enum variant is cloneable
    fn try_clone(&self) -> std::io::Result<Connection> {
        match self {
            Connection::Http(stream) => Ok(Connection::Http(stream.try_clone()?)),
            Connection::Https(stream) => Ok(Connection::Https(stream.try_clone()?)),
        }
    }
}
