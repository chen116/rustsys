
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::{Stream, StreamExt};
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};

use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use crate::{RX_PORT};


pub async fn hi() -> Result<(), Box<dyn Error>>   {
    println!("hi");
    Ok(())
}
struct Shared {
    peers: HashMap<SocketAddr, String>,
}

impl Shared {
    /// Create a new, empty, instance of `Shared`.
    fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }

    /// Send a `LineCodec` encoded message to every peer, except
    /// for the sender.
    async fn broadcast(&mut self, sender: SocketAddr, message: &str) {
        // for peer in self.peers.iter_mut() {
        //     if *peer.0 != sender {
        //         let _ = peer.1.send(message.into());
        //     }
        // }
    }
}
pub async fn run(addr_clone: String,p1: mpsc::Sender<String>) -> Result<(), Box<dyn Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr =  addr_clone+":"+RX_PORT;

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;

    println!("rx server running on {}", addr);


    


    loop {
        // Asynchronously wait for an inbound socket.
        println!("rx got cli: {}",addr.to_string() );
        let (stream, addr) = listener.accept().await?;
        let p1clone = p1.clone();
            tokio::spawn(async move {
                let mut lines = Framed::new(stream, LinesCodec::new());
                while let Some(msg) = lines.next().await {
                    match msg {
                        Ok(txt) => {
                            println!("rx got:{}",txt);
                            p1clone.send(txt).await.unwrap();
                        },
                        _ => println!("rx get nuffin"),
                    }
                }
        });
    }

    Ok(())
}
