
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
pub async fn run(p1: mpsc::Sender<String>) -> Result<(), Box<dyn Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr =  "127.0.0.1:8082".to_string();

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;

    println!("exter_in server running on {}", addr);


    


    loop {
        // Asynchronously wait for an inbound socket.
        let (stream, addr) = listener.accept().await?;
        let p1clone = p1.clone();
            tokio::spawn(async move {
                let mut lines = Framed::new(stream, LinesCodec::new());
                while let Some(msg) = lines.next().await {
                    match msg {
                        Ok(txt) => {
                            println!("exter in got:{}",txt);
                            p1clone.send(txt).await.unwrap();
                        },
                        _ => println!("exter in get nuffin"),
                    }
                }
        });
    }

    Ok(())
}

async fn process(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {


    let mut lines = Framed::new(stream, LinesCodec::new());
    while let Some(msg) = lines.next().await {


    }


    // Send a prompt to the client to enter their username.
    lines.send("Please enter your username:").await?;



Ok(())
}