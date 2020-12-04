use tokio::net::{ TcpStream,TcpListener};
use tokio::stream::{StreamExt};
use tokio_util::codec::{Framed, LinesCodec};

use futures::SinkExt;
use std::error::Error;
use std::io;

    use bytes::Bytes;
    use futures::{future, Sink, Stream};
    use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

use tokio::sync::mpsc;


use crate::{EXTER_IN_PORT};

pub async fn run(addr_clone: String, p1: mpsc::Sender<String>) -> Result<(), Box<dyn Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.

    let addr =  addr_clone+":"+EXTER_IN_PORT;
    
    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;

    println!("exter_in server running on {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (stream, addr) = listener.accept().await?;
        println!("exter_in got cli: {}",addr.to_string() );
        let p1clone = p1.clone();
            tokio::spawn(async move {
                let mut lines = Framed::new(stream, LinesCodec::new());
                let mut lenbuff = String::new();
                let mut content = String::new();
                let mut newdata_len: u32 = 0;
                
                while let Some(msg) = lines.next().await {
                    match msg {
                        Ok(txt) => {
                            let mut cloned_txt = txt.clone();
                            if newdata_len == 0 {
                                println!("data init whole len:{}", cloned_txt.len());

                                let data = cloned_txt.split_off(4);
                                // let data_size = data.clone();
                                newdata_len =cloned_txt.parse::<u32>().unwrap()  ;

                                content.push_str(&data);
                                println!("{} len : {}",
                                data,newdata_len);
                                println!("data init real len:{},{}",content.len(), data.len());

                            } else {
                                println!("data filling len : {},{}",content.len(),txt.len());
                                content.push_str(&txt);

                            }
                                println!("content len {}",content.len());

                            // println!("exter in got:{}",content);
                            if (content.len() as u32) >= newdata_len {
                                let param = content.split_off( (newdata_len as usize)-1);
                                

                                println!("newdata_len {}, param {}",newdata_len,param);

                                p1clone.send(content.clone()).await.unwrap();
                                content.clear();
                                newdata_len=0;
                            }
                            // p1clone.send(content.clone()).await.unwrap();
                        },
                        _ => println!("exter in get nuffin"),
                    }
                }
        });
    }

    Ok(())
}
