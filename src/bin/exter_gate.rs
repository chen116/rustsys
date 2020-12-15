// cargo run --bin exter_gate -- --host 10.67.1.239
use tokio::net::{ TcpStream};

use futures::SinkExt;
use std::error::Error;
use std::io;

    use bytes::Bytes;
    use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

use tokio::sync::mpsc;

use structopt::StructOpt;
use rustsys::{EXTER_IN_PORT};
#[derive(StructOpt, Debug)]
#[structopt(name = "fogsys-server", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "A Redis server")]
struct RemoteHost {



    #[structopt(name = "host", long = "--host", default_value = "127.0.0.1")]
    host: String,

    #[structopt(name = "port", long = "--port", default_value = EXTER_IN_PORT)]
    port: String,


}



#[tokio::main]
pub async fn main() ->Result<(), Box<dyn Error>> {

    let remote_host = RemoteHost::from_args();



    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let (victx, mut vicrx) = mpsc::channel(32);

    let addr = format!("{}:{}", remote_host.host.as_str(),remote_host.port.as_str());
    println!("connecting to {}",addr);

    let stream = TcpStream::connect(addr).await?;



    let ( r,  w) = stream.into_split();
    let mut sink = FramedWrite::new(w, BytesCodec::new());
    // filter map Result<BytesMut, Error> stream into just a Bytes stream to match stdout Sink
    // on the event of an Error, log the error and end the stream
    let mut  _source = FramedRead::new(r, BytesCodec::new());


    let victxclone = victx.clone();



     tokio::spawn(async move { 

        while let Some(mesg) = vicrx.recv().await {
            println!("sending {:?}", mesg );
            sink.send(mesg).await.unwrap();
            // handle details
        }

      });

            loop {
                let mut input = String::new();
               
                io::stdin().read_line(&mut input).unwrap();
                input.pop();
              
                let total_bytes = input.as_bytes().to_vec();
                 

                victxclone.send(Bytes::copy_from_slice(&total_bytes)).await.expect("could not send");

            }
        



       





    Ok(())
}