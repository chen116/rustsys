// use rustsys::{server,client,stdcli};
use std::error::Error;
use std::env;
use tokio::sync::mpsc;
use rustsys::datastore::{neighbour,app};
use rustsys::connection::{rx,dy_tx,exter_in};
use rustsys::core::{coord};
use pnet::datalink;





use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "fogsys-server", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "rustsys")]
struct DiscoveryOrLocal {



    #[structopt(name = "compute_env", long = "--env", default_value = "local")]
    host: String,



}



#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>  {


    let mut addr = "127.0.0.1".to_string();
    
    let compute_env = DiscoveryOrLocal::from_args();

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
        _ => {
            let name = hostname::get()?;
            addr = name.to_string_lossy().to_string().clone();
            let mut parts = addr.splitn(2, '.'); // to convert xxx.hpc.usc.edu to xxx
            addr = parts.next().unwrap().to_string();
        }

    }






    // let addr = addr + ":6142";
    println!("addr is: {}",addr);
    let (mut p1, mut c1) = mpsc::channel(32);
    let (mut p2, mut c2) = mpsc::channel(32);
    // let (mut p3, mut c3) = mpsc::channel(32);


    let nb = neighbour::Neighbour::new();
    let apps = app::App::new();
    // nb.set("hi".to_string(), p2);
    // let p3 = nb.get(&("hi".to_string())).unwrap();
    // let p4=p3.clone();
    // nb.set("hi".to_string(), p4);
    // println!("{:?}",nb.get(&("hi".to_string())).unwrap());


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

    let addr_clone = addr.clone();
    let p1_clone = p1.clone();
    let rx = tokio::spawn(async move { 
        rx::run(addr_clone,p1_clone).await;
    });

    // let addr_clone = addr.clone();
    // let p1_clone = p1.clone();
    // let app_rx = tokio::spawn(async move { 
    //     app_rx::run(addr_clone,p1_clone).await;
    // });

    let nb_clone = nb.clone();
    let apps_clone = apps.clone();
    let coord = tokio::spawn(async move { 
        coord::run(addr,&mut c1,nb_clone,p2,apps_clone).await;
    });



    let nb_clone = nb.clone();
    let dy_tx = tokio::spawn(async move { 
        dy_tx::run(nb_clone,&mut c2).await;
    });



    let _done = coord.await?;

    Ok(())
}