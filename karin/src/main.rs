use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::prelude::*;
const SIROCCO_SERVER_ADD:&str ="localhost:8001";
const KARIN_SERVER_ADD:&str ="localhost:8001";
use std::env::args;

use std::{thread,time::Duration};


fn main() {

  
    //starting
    println!("karin starting {}",KARIN_SERVER_ADD);

    let listener=TcpListener::bind(KARIN_SERVER_ADD).unwrap();

    println!("karin listening on {}",KARIN_SERVER_ADD);

    for stream in listener.incoming(){


        
        match stream {
            Ok(stream) => {
                handle_connection(stream);
                println!("connection established");
            }
            Err(e) => { /* connection failed */ }
        }
    }
    

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();
    let message=String::from_utf8_lossy(&buffer);
    println!("received : {}",message);

    //delay

    let sirocco_message=call_siroko(message.to_owned().to_string());

    let output=format!("sirrocco says: {}", sirocco_message);
    // write the message
    let _= stream.write_all(output.as_bytes());
    println!("sent {}",output);
}


use  std::io::prelude::*;


const ECHO_SERVER:&str="localhost:1234";







fn call_siroko(message:String)->String {
   println!("connecting to {}",SIROCCO_SERVER_ADD);
  if let Ok(mut stream )= TcpStream::connect(SIROCCO_SERVER_ADD){

    println!("connected to SIrocco{}:{}",stream.local_addr().unwrap().ip(),stream.local_addr().unwrap().port());

    
    let _ =stream.write(message.as_bytes());
    let _ =stream.flush();
    println!("Message sent to sirocco-> {}",message);
     //read to result
     let  mut buffer=[0;1024];
     stream.read(&mut buffer);

     let sent_mag=String::from_utf8_lossy(&buffer);
     println!("recived form sirocco : {}",sent_mag);

     return  message.to_owned().to_string();

  }else {
    println!(" failed to connect to sirocco server {}",SIROCCO_SERVER_ADD);

    return String::from("failed to connect");
      
  }



}
