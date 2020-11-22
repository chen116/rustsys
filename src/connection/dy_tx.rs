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
use crate::datastore::{ets,neighbour};
use crate::connection::{tx};
use tokio::sync::watch;

// use tokio::io::AsyncWriteExt;

pub async fn run(nb: neighbour::Neighbour,dy_tx_c: &mut mpsc::Receiver<String>,) -> Result<(), Box<dyn Error>> {


        while let Some(remote_host) = dy_tx_c.recv().await {
            println!("dy_tx going to connect with {:?}", remote_host );
            // let addr =  remote_host+":"+RX_PORT;

            let (mut p, mut c) = mpsc::channel(32);
            // let clone_host = remote_host.clone();
            nb.set(remote_host.clone(), p);


            let tx = tokio::spawn(async move { 
                
                tx::run(remote_host.to_string(),&mut c).await;
            });

            // handle details
        }

    //   });


    // //     let totalk = tokio::spawn(async move { 

    // //         let mut input = String::new();
    // //         io::stdin().read_line(&mut input).unwrap();
    // //         input.pop();
    // //         victxclone.send(input).await;

        
    // //     });
    // loop{
    //         let mut input = String::new();
    //         io::stdin().read_line(&mut input).unwrap();
    //         input.pop();
    //         // let result = stream.write(b"hello world\n").await;
    //         // let result = stream.write(input).await;

    //         sink.send(input).await.unwrap();
    //     }
    // // let done = listen.await?;

       



    Ok(())
}