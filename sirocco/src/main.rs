use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::prelude::*;
const SIROCCO_SERVER_ADD:&str ="localhost:8000";
use std::env::args;

use std::{thread,time::Duration};


fn main() {

    // read the arguments
    let delay=args().nth(1).unwrap_or_default().parse::<u64>().unwrap();


    println!(" delay {:?}",delay);
    //starting
    println!("sirocco starting {}",SIROCCO_SERVER_ADD);

    let listener=TcpListener::bind(SIROCCO_SERVER_ADD).unwrap();

    println!("sirocco listening on {}",SIROCCO_SERVER_ADD);

    for stream in listener.incoming(){


        
        match stream {
            Ok(stream) => {
                handle_connection(stream,delay);
                println!("connection established");
            }
            Err(e) => { /* connection failed */ }
        }
    }
    

}

fn handle_connection(mut stream: TcpStream, delay:u64) {
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();
    let message=String::from_utf8_lossy(&buffer);
    println!("received : {}",message);

    //delay

    thread::sleep(Duration::from_millis(delay));
    // write the message
    let _= stream.write_all(message.as_bytes());
    println!("sent {}",message);
}



