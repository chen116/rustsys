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

pub async fn run(mainvicrx: &mut mpsc::Receiver<String>) -> Result<(), Box<dyn Error>>  {
 // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let (mut victx, mut vicrx) = mpsc::channel(32);

// println!("port num please:");
//  let mut input = String::new();
// io::stdin().read_line(&mut input).unwrap();

    let mut interval_timer = tokio::time::interval(Duration::from_millis(1000));
    interval_timer.tick().await;


    // let mut stream = TcpStream::connect(input+":6142").await?;
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;



        let (mut r, mut w) = stream.into_split();
        let mut sink = FramedWrite::new(w, LinesCodec::new());
        // filter map Result<BytesMut, Error> stream into just a Bytes stream to match stdout Sink
        // on the event of an Error, log the error and end the stream
        let mut source = FramedRead::new(r, LinesCodec::new());

        let inis="Please enter your username:".to_string();

    let listen = tokio::spawn(async move { 
            while let Some(Ok(event)) = source.next().await {
                        println!("Event {}", event);
                        match event.as_str()  {
                        "Please enter your username:" => {
                                let mut input = String::new();
                                // io::stdin().read_line(&mut input).unwrap();
                                // victxclone.send("sss".to_string() ).await;

                                // sink.send(input).await.unwrap();
                            },
                            _ => println!("getting {}", event)
                        }  
                    }
      });



    let tosend = tokio::spawn(async move { 
        while let Some(mesg) = vicrx.recv().await {
            println!("sending {:?}", mesg );
            sink.send(mesg).await.unwrap();
            // handle details
        }

      });

    // let victxclone = victx.clone();

    //     let totalk = tokio::spawn(async move { 
    //         let mut input = String::new();
    //         io::stdin().read_line(&mut input).unwrap();
    //         input.pop();
    //         victxclone.send(input).await;
    //          for n in 1..4 {

    //             victxclone.send(n.to_string()).await;

    //             }
        
    //     });



        while let Some(mesg) = mainvicrx.recv().await {
            println!("stdcli sending {:?}", mesg );
            victx.send(mesg).await;
            // handle details
        }



// let victxclone2 = victx.clone();
// let looping = tokio::spawn(async move {



//     loop {
//         // Wait for the next interval tick
//     // 
//             let mut input = String::new();
//             io::stdin().read_line(&mut input).unwrap();
//             input.pop();


//         // interval_timer.tick().await;
//          victxclone2.send(input).await;
//     }

//  });


    // 
    //         let mut input = String::new();
    //         io::stdin().read_line(&mut input).unwrap();
    //         input.pop();
    //             println!("goingin: {}",input);

    //         victx.send(input).await;
    //     }
    let done = listen.await?;

       



    // let mut lines =  Framed::new(stream, LinesCodec::new());


    // let first_line = match lines.next().await {
    //     Some(Ok(line)) => line,
    //     _ => "meoww".to_string()
    // };

    // println!("{}",first_line);

    // let mut input = String::new();


    // match io::stdin().read_line(&mut input) {
    //     Ok(_txt) => {
    //         println!("{:?}",input);
    //         let result = lines.send(input).await?;
    //     },
    //     Err(_no_updates_is_fine) => {},
    // }




    // let mut input = String::new();
    // loop {
    // let username = match lines.next().await {
    //     Some(Ok(line)) => line,
    //     // We didn't get a line so we return early here.
    //     _ => "meoww".to_string()
    // };

    // println!("{}",username);

    // lines.send("nice").await?;

    // }

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
