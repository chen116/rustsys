

use std::error::Error;


use tokio::sync::mpsc;
use crate::datastore::{neighbour};
use crate::connection::{tx};

// use tokio::io::AsyncWriteExt;

pub async fn run(nb: neighbour::Neighbour,dy_tx_c: &mut mpsc::Receiver<String>,) -> Result<(), Box<dyn Error>> {


        while let Some(remote_host) = dy_tx_c.recv().await {
            println!("dy_tx going to connect with {:?}", remote_host );
            // let addr =  remote_host+":"+RX_PORT;

            let ( p, mut c) = mpsc::channel(32);
            // let clone_host = remote_host.clone();
            nb.set(remote_host.clone(), p);

            tokio::spawn(async move { 
                
                tx::run(remote_host.to_string(),&mut c).await.expect("could not run tx");
            });

        }



       



    Ok(())
}