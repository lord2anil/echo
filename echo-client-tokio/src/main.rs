

use tokio::{io::AsyncWriteExt,  net::TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
const ECHO_SERVER:&str="localhost:8000";






#[tokio::main]

async fn main() {
   println!("connecting to {}",ECHO_SERVER);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER).await {
        println!("connected to echo server {}:{}", stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());

        let message = "hello this is anil";
        let _ = stream.write_all(message.as_bytes()).await;
        
        println!("Message sent-> {}", message);

        // read to result
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await;

        let sent_mag = String::from_utf8_lossy(&buffer);
        println!("recived : {}", sent_mag);
    



   
     
    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER);
    }
  



}
