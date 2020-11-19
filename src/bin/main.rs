// use rustsys::{server,client,stdcli};
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use std::env;
use tokio::sync::mpsc;

use tracing_subscriber;
use tracing::info;


use rustsys::datastore::{ets};
use rustsys::connection::{rx,tx,exter_in};
use rustsys::core::{coord};
// use dns_lookup::{lookup_host, lookup_addr};
// use get_local_ip::{local, network};
use pnet::datalink;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>  {

    // println!("{:?}", network().unwrap().ip); // 192.168.43.134
    // println!("{:?}", local().unwrap()); // 127.0.0.1
    // let hostname = "localhost";
    // let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
    // for ip in ips.iter(){
    //     println!("ip:{}",ip.to_string());
    // } 

    let mut addr = "127.0.0.1".to_string();
    for iface in datalink::interfaces() {
        println!("{:?}",iface);
        match iface.is_up(){
            true   => {
                match iface.ips[0].ip().to_string().contains("127.") || iface.ips[0].ip().to_string().contains("172.") {
                    false => 
                    { 
                        addr = iface.ips[0].ip().to_string();
                    },
                    _ => {}
                    }
                },
            _ => {}
        }
    }
    // let addr = addr + ":6142";
    println!("addr is: {}",addr);
    let (mut p1, mut c1) = mpsc::channel(32);
    // let (mut p2, mut c2) = mpsc::channel(32);

    let ds = ets::Ets::new();
    let db = ets::SimpleEts::new();


    // let addr = env::args()
    //     .nth(1)
    //     .unwrap_or_else(|| "127.0.0.1:6142".to_string());

    // Bind a TCP listener to the socket address.
    // Note that this is the Tokio TcpListener, which is fully async.
    // let listener = TcpListener::bind(&addr).await?;
    // let ser = tokio::spawn(async move { 
    //     server::run(listener,addr).await;
    // });

    let addr_clone = addr.clone();
    let p1_clone = p1.clone();
    let exter_in = tokio::spawn(async move { 
        exter_in::run(addr_clone,p1_clone).await;
    });


    let db = ets::SimpleEts::new();
    let coord = tokio::spawn(async move { 
        coord::run(&mut c1,db.clone()).await;
    });

    let addr_clone = addr.clone();
    let p1_clone = p1.clone();
    let rx = tokio::spawn(async move { 
        rx::run(addr_clone,p1_clone).await;
    });

    // let addr_clone = addr.clone();
    // let tx = tokio::spawn(async move { 
    //     tx::run(addr_clone,&mut c2).await;
    // });



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