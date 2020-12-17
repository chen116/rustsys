use tokio::net::{ TcpListener};
use tokio::stream::{StreamExt};
use tokio_util::codec::{Framed};

use std::error::Error;

use tokio_util::codec::{BytesCodec};

use tokio::sync::mpsc;


use crate::{GATEWAY_IN_PORT};

pub async fn run(addr_clone: String, p1: mpsc::Sender<String>) -> Result<(), Box<dyn Error>> {

    let addr =  addr_clone+":"+GATEWAY_IN_PORT;
    
    let listener = TcpListener::bind(&addr).await?;

    println!("gateway server running on {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (stream, addr) = listener.accept().await?;
        println!("gateway got cli: {}",addr.to_string() );
        let p1clone = p1.clone();
            tokio::spawn(async move {
                let mut lines = Framed::new(stream, BytesCodec::new());
                while let Some(msg) = lines.next().await {
                    match msg {
                        Ok(txt) => {
                            p1clone.send( String::from_utf8(txt.to_vec()).unwrap() ).await.unwrap();
                        },
                        _ => println!("gateway got nothing"),
                    }
                }
        });
    }

    Ok(())
}
