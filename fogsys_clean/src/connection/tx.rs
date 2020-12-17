use tokio::net::{ TcpStream};

use futures::SinkExt;
use std::error::Error;

use bytes::Bytes;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

use tokio::sync::mpsc;
use crate::{RX_PORT};

pub async fn run(addr_clone: String, c2: &mut mpsc::Receiver<String>) -> Result<(), Box<dyn Error>> {
    let addr =  addr_clone+":"+RX_PORT;



    let  stream = TcpStream::connect(&addr).await?;
    let ( r,  w) = stream.into_split();
    let mut sink = FramedWrite::new(w, BytesCodec::new());


    let  _source = FramedRead::new(r, BytesCodec::new());



    println!("tx estlibshed {}",addr);




        while let Some(mesg) = c2.recv().await {
         
                let  input_bytes = mesg.as_bytes().to_vec();
                // send to neighbouring cloudlet
                sink.send(Bytes::copy_from_slice(&input_bytes)).await.expect("could not send");

        }

   


   

       



    Ok(())
}