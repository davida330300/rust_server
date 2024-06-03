use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buffer = [0; 2048];
            stream.read(&mut read_buffer).unwrap();
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into(); // !!!

            Router::route(req, &mut stream)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_server_run() {
        let server_addr = "127.0.0.1:7878";
        let server = Server::new(server_addr);

        // Run the server in a separate thread
        let server_thread = thread::spawn(move || {
            server.run();
        });

        // Allow some time for the server to start
        thread::sleep(Duration::from_millis(100));

        // Simulate a client request
        let mut client = TcpStream::connect(server_addr).unwrap();
        let request = "GET / HTTP/1.1\r\n\r\n";
        client.write(request.as_bytes()).unwrap();
        client.flush().unwrap();

        // Read the response from the server
        let mut response = String::new();
        client.read_to_string(&mut response).unwrap();
        println!("{}", response);
        assert!(response.contains("HTTP/1.1 200 OK"));
        assert!(response.contains("Handled request"));

        // Close the client connection
        drop(client);

        // Allow the server to process the disconnection
        thread::sleep(Duration::from_millis(100));

        // Stop the server thread
        drop(server_thread);
    }
}