use tokio::net::{ TcpStream};
use tokio::stream::{StreamExt};
use tokio_util::codec::{Framed, LinesCodec};

use futures::SinkExt;
use std::error::Error;
use std::io;

use bytes::Bytes;
use futures::{future, Sink, Stream};
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

use tokio::sync::mpsc;
use crate::{TX_PORT,RX_PORT};
// use tokio::io::AsyncWriteExt;

pub async fn run(addr_clone: String, c2: &mut mpsc::Receiver<String>) -> Result<(), Box<dyn Error>> {
    let addr =  addr_clone+":"+RX_PORT;

    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    // let (mut victx, mut vicrx) = mpsc::channel(32);


    let mut stream = TcpStream::connect(&addr).await?;
    let (mut r, mut w) = stream.into_split();
    let mut sink = FramedWrite::new(w, LinesCodec::new());
    // filter map Result<BytesMut, Error> stream into just a Bytes stream to match stdout Sink
    // on the event of an Error, log the error and end the stream
    let mut source = FramedRead::new(r, LinesCodec::new());
    println!("tx estlibshed {}",addr);
    //     let inis="Please enter your username:".to_string();

    // let victxclone = victx.clone();
    // let listen = tokio::spawn(async move { 
    //         while let Some(Ok(event)) = source.next().await {
    //                     println!("Event {}", event);
    //                     match event.as_str()  {
    //                     "Please enter your username:" => {
    //                             let mut input = String::new();
    //                             // io::stdin().read_line(&mut input).unwrap();
    //                             // victxclone.send("sss".to_string() ).await;

    //                             // sink.send(input).await.unwrap();
    //                         },
    //                         _ => println!("getting {}", event)
    //                     }  
    //                 }
    //   });




        while let Some(mesg) = c2.recv().await {
            println!("sending {:?}", mesg );
            sink.send(mesg).await.unwrap();
            // handle details
        }

   


    //     let totalk = tokio::spawn(async move { 

    //         let mut input = String::new();
    //         io::stdin().read_line(&mut input).unwrap();
    //         input.pop();
    //         victxclone.send(input).await;

        
    //     });
    // loop{
    //         let mut input = String::new();
    //         io::stdin().read_line(&mut input).unwrap();
    //         input.pop();
    //         // let result = stream.write(b"hello world\n").await;
    //         // let result = stream.write(input).await;

    //         sink.send(input).await.unwrap();
    //     }
    // let done = listen.await?;

       



    Ok(())
}