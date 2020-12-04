// cargo run --bin exter_gate -- --host 10.67.1.239
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

use structopt::StructOpt;
use rustsys::{EXTER_IN_PORT};

#[derive(StructOpt, Debug)]
#[structopt(name = "fogsys-server", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "A Redis server")]
struct Remote_host {



    #[structopt(name = "host", long = "--host", default_value = "127.0.0.1")]
    host: String,

    #[structopt(name = "port", long = "--port", default_value = EXTER_IN_PORT)]
    port: String,


}



#[tokio::main]
pub async fn main() ->Result<(), Box<dyn Error>> {

    let remote_host = Remote_host::from_args();



    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let (mut victx, mut vicrx) = mpsc::channel(32);

    let addr = format!("{}:{}", remote_host.host.as_str(),remote_host.port.as_str());
    println!("connecting to {}",addr);

    let mut stream = TcpStream::connect(addr).await?;


    let (mut r, mut w) = stream.into_split();
    let mut sink = FramedWrite::new(w, LinesCodec::new());
    // filter map Result<BytesMut, Error> stream into just a Bytes stream to match stdout Sink
    // on the event of an Error, log the error and end the stream
    let mut source = FramedRead::new(r, LinesCodec::new());

    let inis="enter your command:".to_string();

    let victxclone = victx.clone();



    let tosend = tokio::spawn(async move { 

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
                
                
                 let wasm_bytes = include_bytes!("../wasm/fib.wasm");
                  let wasm_string = match String::from_utf8(wasm_bytes.to_vec()) {
                      Ok(v) => v,
                      Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                  };

                  input.insert_str(0,&wasm_string);
                
                
                
                
                
                
                
                
                let string_byte_len = wasm_string.as_bytes().len().to_string();
                let string_byte_len_len = wasm_string.as_bytes().len().to_string().as_bytes().len();
                let prefix_len: u32 = 4;
                let mut complement_placeholder = 4-string_byte_len_len;
                // let s="0021".to_string();
                // let intt=s.parse::<i32>().unwrap();

                input.insert_str(0,&(string_byte_len));

                let final_string = loop {
                    complement_placeholder -=1;
                    input.insert(0,'0');
                    if complement_placeholder==0{
                        break input;
                    }
                };





                            println!("gate got:{} {} {}",final_string,
                             string_byte_len, string_byte_len_len);
                victxclone.send(final_string).await;
            //  for n in 1..4 {

            //     victxclone.send(n.to_string()).await;

            //     }
            }
        

    let done = tosend.await?;

       





    Ok(())
}