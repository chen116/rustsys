// use rustsys::{server,client,stdcli};
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use std::env;
use tokio::sync::mpsc;

use tracing_subscriber;
use tracing::info;


use rustsys::datastore::{ets};
use rustsys::connection::{rx,exter_in};
use rustsys::core::{coord};


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>  {


    let (mut p1, mut c1) = mpsc::channel(32);

    let ds = ets::Ets::new();
    let db = ets::SimpleEts::new();


    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());

    // Bind a TCP listener to the socket address.
    //
    // Note that this is the Tokio TcpListener, which is fully async.
    let listener = TcpListener::bind(&addr).await?;

    // let ser = tokio::spawn(async move { 
    //     server::run(listener,addr).await;
    // });

    let exter_in = tokio::spawn(async move { 
        exter_in::run(p1.clone()).await;
    });



    let db = ets::SimpleEts::new();
    let coord = tokio::spawn(async move { 
        coord::run(&mut c1,db.clone()).await;
    });


    // let rx = tokio::spawn(async move { 
    //     rx::run().await;
    // });
    

    // let stcli = tokio::spawn(async move { 
    //     stdcli::run(victx).await;
    // });

    // let cli = tokio::spawn(async move { 
    //     client::run( &mut  vicrx).await;
    // });

    println!("sss");
    let done = coord.await?;

    Ok(())
}