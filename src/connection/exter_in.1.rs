use tokio::net::{ TcpStream,TcpListener};
use tokio::stream::{StreamExt};
use tokio_util::codec::{Framed, LinesCodec};

use futures::SinkExt;
use std::error::Error;
use std::io;

    use bytes::Bytes;
    use futures::{future, Sink, Stream};
    use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

use tokio::sync::mpsc;


use crate::{EXTER_IN_PORT};

pub async fn run(addr_clone: String, p1: mpsc::Sender<String>) -> Result<(), Box<dyn Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.

    let addr =  addr_clone+":"+EXTER_IN_PORT;
    
    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;

    println!("exter_in server running on {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (stream, addr) = listener.accept().await?;
        println!("exter_in got cli: {}",addr.to_string() );
        let p1clone = p1.clone();
            tokio::spawn(async move {
                let mut lines = Framed::new(stream, LinesCodec::new_with_max_length(256));

                
                while let Some(msg) = lines.next().await {
                    match msg {
                        Ok(txt) => {
                           
                            p1clone.send(txt).await.unwrap();
                        },
                        _ => println!("exter in get nuffin"),
                    }
                }
        });
    }

    Ok(())
}