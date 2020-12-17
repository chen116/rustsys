// rx receive data from other cloudlets

use tokio::net::{TcpListener};
use tokio::stream::{ StreamExt};
use tokio::sync::{mpsc};
use tokio_util::codec::{Framed,BytesCodec};

use std::error::Error;
use crate::{RX_PORT};

pub async fn run(addr_clone: String,p1: mpsc::Sender<String>) -> Result<(), Box<dyn Error>> {

    let addr =  addr_clone+":"+RX_PORT;

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;

    println!("rx server running on {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (stream, addr) = listener.accept().await?;
        println!("rx got cli: {}",addr.to_string() );
        let p1clone = p1.clone();
            tokio::spawn(async move {
                let mut lines = Framed::new(stream, BytesCodec::new());
                while let Some(msg) = lines.next().await {
                    match msg {
                        Ok(txt) => {
                            // send neighbouring cloudlet's command/data to coordinator
                            p1clone.send( String::from_utf8(txt.to_vec()).unwrap() ).await.unwrap();
                        },
                        _ => println!("rx get nothing"),
                    }
                }
        });
    }

    Ok(())
}

             

