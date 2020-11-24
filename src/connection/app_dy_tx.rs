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
use crate::connection::{app_tx};
use tokio::sync::watch;

// use tokio::io::AsyncWriteExt;

pub async fn run(apps: neighbour::Neighbour,dy_app_tx: &mut mpsc::Receiver<String>,) -> Result<(), Box<dyn Error>> {


        while let Some(remote_port) = dy_app_tx.recv().await {
            println!("dy_tx going to connect with {:?}", remote_port );
            // let addr =  remote_port+":"+RX_PORT;

            let (mut p, mut c) = mpsc::channel(32);
            // let clone_host = remote_port.clone();
            apps.set(remote_port.clone(), p);


            let tx = tokio::spawn(async move { 
                
                app_tx::run(remote_port.to_string(),&mut c).await;
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