extern crate netstring;

use std::thread;
use std::net::{TcpStream, TcpListener};
use std::sync::mpsc::channel;

use netstring::{ReadNetstring, WriteNetstring};

fn main() {
    let (tx, rx) = channel();

    let t = thread::spawn(move || {
        // Bind to anything
        let l = TcpListener::bind("0.0.0.0:0").unwrap();

        // Send local address to the other half of the example
        tx.send(l.local_addr().unwrap()).unwrap();

        // Accept connection
        let (mut s, _) = l.accept().unwrap();

        // Receive netstring
        let x = s.read_netstring().unwrap();

        println!("Received: {}", x);
    });

    // Receive addr used by the example
    let addr = rx.recv().unwrap();

    // Connect to example
    let mut s = TcpStream::connect(addr).unwrap();

    // Send netstring
    s.write_netstring("Hello world!").unwrap();

    // Wait for listener to finish
    t.join().unwrap();
}
