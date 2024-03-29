
use  std::io::prelude::*;
use std:: net::TcpStream;

const ECHO_SERVER:&str="localhost:1234";







fn main() {
   println!("connecting to {}",ECHO_SERVER);
  if let Ok(mut stream )= TcpStream::connect(ECHO_SERVER){

    println!("connected to echo server {}:{}",stream.local_addr().unwrap().ip(),stream.local_addr().unwrap().port());

    let message="hello this is anil";
    let _ =stream.write(message.as_bytes());
    let _ =stream.flush();
    println!("Message sent-> {}",message);
     //read to result
     let  mut buffer=[0;1024];
     stream.read(&mut buffer);

     let sent_mag=String::from_utf8_lossy(&buffer);
     println!("recived : {}",sent_mag);

     

  }else {
    println!(" failed to connect to echo server {}",ECHO_SERVER);
      
  }



}
