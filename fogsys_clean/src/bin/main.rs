// use fogsys::{server,client,stdcli};
use std::error::Error;
use std::env;
use tokio::sync::mpsc;
use fogsys::datastore::{neighbour,app};
use fogsys::connection::{rx,dy_tx,gateway};
use fogsys::core::{coord};
use pnet::datalink;





use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "fogsys-server", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "fogsys")]
struct CARCOrLocal {



    #[structopt(name = "compute_env", long = "--env", default_value = "local")]
    host: String,



}



#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>  {


    let mut addr = "127.0.0.1".to_string();
    
    let compute_env = CARCOrLocal::from_args();
    // setting current cloudlet addr based on given env
    match compute_env.host.as_str() {
        "local" => {
            for iface in datalink::interfaces() {
                // println!("{:?}",iface);
                match iface.is_up(){
                    true   => {
                        match iface.ips.len() {
                            0 => {},
                        _ => match iface.ips[0].ip().to_string().contains("127.") || iface.ips[0].ip().to_string().contains("172.") {
                                false => 
                                { 
                                    addr = iface.ips[0].ip().to_string();
                                },
                                _ => {}
                                }
                            }
                        },

                    _ => {}
                }
            }

        },
        // anything else will assumed to be in carc, try to get node hostname
        _ => {
            let name = hostname::get()?;
            addr = name.to_string_lossy().to_string().clone();
            let mut parts = addr.splitn(2, '.'); // to convert xxx.hpc.usc.edu to xxx
            addr = parts.next().unwrap().to_string();
        }

    }






    println!("addr is: {}",addr);
    let ( p1, mut c1) = mpsc::channel(32); // information message passing producer consumer pair where producer: Rx,Gateway, consumer: Coordinator 
    let ( p2, mut c2) = mpsc::channel(32); // information message passing producer consumer pair where producer: Coordination, consumer: Dynamic Tx


    let nb = neighbour::Neighbour::new();  // Neighbouring cloudlet Map
    let apps = app::App::new();            // Grpc App Map


    // start Gateway
    let addr_clone = addr.clone();
    let p1_clone = p1.clone();
    let _gateway = tokio::spawn(async move { 
        gateway::run(addr_clone,p1_clone).await.expect("fail");
    });

    // start Rx
    let addr_clone = addr.clone();
    let p1_clone = p1.clone();
    let _rx = tokio::spawn(async move { 
        rx::run(addr_clone,p1_clone).await.expect("fail");
    });


    // start Coordinator
    let nb_clone = nb.clone();
    let apps_clone = apps.clone();
    let coord = tokio::spawn(async move { 
        coord::run(addr,&mut c1,nb_clone,p2,apps_clone).await.expect("fail");
    });


    // start Dynamic Tx
    let nb_clone = nb.clone();
    let _dy_tx = tokio::spawn(async move { 
        dy_tx::run(nb_clone,&mut c2).await.expect("fail");
    });

    let _done = coord.await?;

    Ok(())
}