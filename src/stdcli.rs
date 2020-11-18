use tokio::net::{ TcpStream};
use tokio::stream::{StreamExt};
use tokio_util::codec::{Framed, LinesCodec};

use futures::SinkExt;
use std::error::Error;
use std::io;
// use tokio::io;

    use bytes::Bytes;
    use futures::{future, Sink, Stream};
    use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

use tokio::sync::mpsc;

pub async fn run( maintx: mpsc::Sender<String>) -> Result<(), Box<dyn Error>>  {






    loop {
        println!("next command:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        println!("next command:{}",input);
        maintx.send(input).await;

    }

    Ok(())
 
}


use tokio::time::Duration;
use tokio::time::interval;
pub async fn looping() {
    let mut interval_timer = tokio::time::interval(Duration::from_millis(1000));
    loop {
        // Wait for the next interval tick
        interval_timer.tick().await;
        tokio::spawn(async { println!("tick"); }); // For async task
    }

}
