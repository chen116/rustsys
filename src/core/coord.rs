
use tokio::sync::{mpsc, Mutex};
use crate::datastore::{ets};


use std::error::Error;



pub async fn run(c1: &mut mpsc::Receiver<String>, db: ets::SimpleEts) -> Result<(), Box<dyn Error>>  {

        while let Some(mesg) = c1.recv().await {
            println!("coord c1 got {:?}", mesg );

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
                });

            // victx.send(mesg).await;
            // handle details
        }


Ok(())
}