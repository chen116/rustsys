
use tokio::sync::{mpsc, Mutex};
use crate::datastore::{ets,neighbour};


use std::error::Error;
use tokio::sync::watch;
use std::process::Command;
async fn create_new_app(port_num: String) -> String {
   let output =  Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process");
    "the result of the computation".to_string()
}


pub async fn run(c1: &mut mpsc::Receiver<String>, db: ets::SimpleEts, 
nb: neighbour::Neighbour, dy_tx_p: mpsc::Sender<String> , 
apps: neighbour::Neighbour ,app_dy_tx_p:  mpsc::Sender<String> ) 
-> Result<(), Box<dyn Error>>  {



    let nbb=nb.clone();

    // println!("sender watch: {:?}",tx_dy_sender);
        while let Some(mesg) = c1.recv().await {
            println!("coord c1 got {:?}", mesg );
             let mut parts = mesg.splitn(2, ' ');

             match parts.next() {
                 Some("NEWHOST") => { 
                    dy_tx_p.send(parts.next().unwrap().to_string()).await;
                  },
                  Some("LIST") => { 

                      println!("Connected Cloudlets:");
                      nb.list();
                    // println!("{:?}",nb.get(&("hi".to_string())).unwrap());
                    // println!("LIST {:?}", nb.get(&(  parts.next().unwrap().to_string()   )).unwrap()   );
                  },
                 Some("SEND2HOST") => { 
                    let mut part2s =  (parts.next().unwrap()).splitn(2, ' ');
                    let tx_p = nb.get(&(  part2s.next().unwrap().to_string()   )).unwrap() ;
                    tx_p.send(part2s.next().unwrap().to_string()).await;
                  },
                 Some("NEWAPP") => { 
                    let port_num = parts.next().unwrap().to_string();
                    let port_num_clone = port_num.clone();
                    let join_handle = tokio::spawn(async move {
                        create_new_app(port_num_clone).await
                    });
                    let res = join_handle.await.unwrap();
  
                    app_dy_tx_p.send(port_num).await;
                  },
                 Some("SEND2APP") => { 
                    let mut part2s =  (parts.next().unwrap()).splitn(2, ' ');
                    let tx_p = apps.get(&(  part2s.next().unwrap().to_string()   )).unwrap() ;
                    tx_p.send(part2s.next().unwrap().to_string()).await;
                  },
                 _ => {               
                        let db = db.clone();
                        // Like with other small servers, we'll `spawn` this client to ensure it
                        // runs concurrently with all other clients. The `move` keyword is used
                        // here to move ownership of our db handle into the async closure.
                        tokio::spawn(async move {
                        // Since our protocol is line-based we use `tokio_codecs`'s `LineCodec`
                        // to convert our stream of bytes, `socket`, into a `Stream` of lines
                        // as well as convert our line based responses into a stream of bytes.
                        // Here for every line we get back from the `Framed` decoder,
                        // we parse the request, and if it's valid we generate a response
                        // based on the values in the database.
                        let response = db.handle_request(&mesg.as_str());
                        let response = response.serialize();
                        println!("response: {}",response);

                    // The connection will be closed at this point as `lines.next()` has returned `None`.
                    });}
             }

    

                
                // let db = db.clone();
                // tokio::spawn(async move {
                //     let response = db.handle_request(&mesg.as_str());
                //     let response = response.serialize();
                //     println!("response: {}",response);
                // });

            // victx.send(mesg).await;
            // handle details
        }


Ok(())
}

// fn parse(input: &str) -> Result<Request, String> {
//         let mut parts = input.splitn(3, ' ');
//         match parts.next() {
//             Some("GET") => {
//                 let key = parts.next().ok_or("GET must be followed by a key")?;
//                 if parts.next().is_some() {
//                     return Err("GET's key must not be followed by anything".into());
//                 }
//                 Ok(Request::Get {
//                     key: key.to_string(),
//                 })
//             }
//             Some("SET") => {
//                 let key = match parts.next() {
//                     Some(key) => key,
//                     None => return Err("SET must be followed by a key".into()),
//                 };
//                 let value = match parts.next() {
//                     Some(value) => value,
//                     None => return Err("SET needs a value".into()),
//                 };
//                 Ok(Request::Set {
//                     key: key.to_string(),
//                     value: value.to_string(),
//                 })
//             }
//             Some(cmd) => Err(format!("unknown command: {}", cmd)),
//             None => Err("empty input".into()),
//         }
//     }