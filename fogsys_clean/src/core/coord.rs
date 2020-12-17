
use tokio::sync::{mpsc};
use crate::datastore::{neighbour,app};


use std::error::Error;
use std::process::Command;


use anyhow::Result;
use wasmtime::*;


use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

use tokio::fs::File;
use tokio::prelude::*; 
extern crate dirs;
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

// deplyong new prepcompiled grpc app
async fn create_new_app(app_name: String) -> String {
  let exec = format!("{}/grpc/examples/cpp/helloworld/cmake/build/server_{}",
  dirs::home_dir().unwrap().into_os_string().into_string().unwrap()
  ,app_name).to_string();
  let mut _child = Command::new(exec)
                          .arg("")
                          .spawn()
                          .expect("failed to execute child");
  let res = format!("{} deployed",app_name);
  res
}





pub async fn run(myaddr_ori: String,c1: &mut mpsc::Receiver<String>, 
nb: neighbour::Neighbour, dy_tx_p: mpsc::Sender<String> , 
apps: app::App  ) 
-> Result<(), Box<dyn Error>>  {



    let nbb=nb.clone();

    while let Some(mesg) = c1.recv().await {
      let myaddr = myaddr_ori.clone();
      let mut parts = mesg.splitn(2, ' ');

      match parts.next() {
          Some("NEWHOST") => { 
            // NEWHOST $NEW_TO BE_CONNECTED_CLOUDLET_HOST_NAME
            dy_tx_p.send(parts.next().unwrap().to_string()).await.expect("could not send");
          },
          Some("HOSTS") => { 

              println!("Connected HOSTs:");
              nb.list();
          },
          Some("APPS") => { 

              println!("Available Apps:");
              apps.list();
          },
          Some("NEWAPP") => { 
            // NEWAPP $GRPC_APP_NAME
            let app_name = parts.next().unwrap().to_string();
            let app_name_clone = app_name.clone();
            let join_handle = tokio::spawn(async move {
                create_new_app(app_name_clone).await
            });
            let res = join_handle.await.unwrap();
            println!("{}",res );

            apps.set(app_name.clone(),myaddr.clone());
            let info = format!("UPDATEAPPS {} {}",app_name.clone(),myaddr);
            let mut tx_ps = nb.all_neighbours();
            while let Some(tx_p) = tx_ps.pop() {
                tx_p.send(info.to_string()).await.expect("could not send");
            }

          },
          Some("UPDATEAPPS") => { 
            let mut part2s =  (parts.next().unwrap()).splitn(3, ' ');
            apps.set(part2s.next().unwrap().to_string(),part2s.next().unwrap().to_string()   );

          },
          Some("SEND2APP") => { 
            // SEND2APP $GRPC_APP_NAME $FUNCTION_AGRUMENT $REQUESTED_CLOUDLET_NAME
            let mut part2s =  (parts.next().unwrap()).splitn(4, ' ');

            let appname =  part2s.next().unwrap().to_string() ;
            let value =  part2s.next().unwrap().to_string() ;
            let mut remote_caller = "none".to_string();
                  let origin_host = part2s.next() ;
                  match origin_host {
                    Some(inner) =>
                    {
                      
                      remote_caller = inner.to_string().clone();
                    }
                      ,
                    None => {
                      println!("from here");
                      
                    },
                  }
            println!("from HOST {}",remote_caller);
            let host = apps.get(&(appname) ).unwrap() ;
            let nb_clone = nb.clone();
            let myaddr_clone = myaddr.clone();
            if host == myaddr_clone {
                println!("run here");
                tokio::spawn(async move {
                  let mut client = GreeterClient::connect("http://localhost:50051").await.unwrap(); // notice the port number is predetetmined, should be improved in the future, that means the grpc server port assigned should be improved as well
                  let request = tonic::Request::new(HelloRequest {
                      name: value.clone(),
                    });
                  let response = client.say_hello(request).await.unwrap();
                  let res_str = response.into_inner().message.to_string();
                  let info = format!("RESPONSE {}({})={}",appname,value,res_str);
                  if remote_caller != "none".to_string() && remote_caller != myaddr_clone
                  {
                    let tx_p = nb_clone.get(&( remote_caller   )).unwrap() ;
                    tx_p.send(   info.to_string()).await.expect("could not send");
                  }
                  else{
                    println!("{}",info );
                  }
            
                });
            } else {
              let tx_p = nb.get(&(host)).unwrap() ;
              let info = format!("SEND2APP {} {} {}",appname,value,myaddr.clone());
              tx_p.send( info.to_string() ).await.expect("could not send");
            }


          },
          Some("GETWASM") => {
            let mut part2s =  (parts.next().unwrap()).splitn(3, ' ');
            let remote_caller = part2s.next().unwrap().to_string(); 
            let param = part2s.next().unwrap().to_string().parse::<i32>().unwrap();
            let wasm_string = part2s.next().unwrap().to_string(); 
            let nbbb =  nbb.clone();
            tokio::spawn(async move {
              let swasm_bytes =  wasm_string.as_bytes();
              println!("wasm byte from: {}, func param: {}",remote_caller,param);
              let store = Store::default();
              let module = Module::from_binary(store.engine(), swasm_bytes).unwrap();
              let instance = Instance::new(&store, &module, &[]).unwrap();
              // Invoke export
              let func = instance
                  .get_func("func")
                  .ok_or(anyhow::format_err!("failed to find function export")).unwrap()
                  .get1::<i32, i32>().unwrap();
              match func(param )
              {
                Ok(res ) => {
                      println!("Result func({}) = {}", param,res );
                      tokio::spawn(async move {
                        let info = format!("RESPONSE func({}) = {}", param,res);
                        let tx_p = nbbb.get(&( remote_caller   )).unwrap() ;
                        tx_p.send(   info.to_string()).await.expect("could not send");
                      });
                },
                _=>{
                  println!("not good wasm");
                }
              }
            });
          },
          Some("SENDWASM") =>{
            // SENDWASM $NAME_OF_CLOUDLET_TO_SEND_TO $WASM_APP_NAME $FUNCTION_AGRUMENT 
            let mut part2s =  (parts.next().unwrap()).splitn(3, ' ');
            let host =  part2s.next().unwrap().to_string() ;
            let wasm_file_name =  part2s.next().unwrap().to_string() ;
            let param =  part2s.next().unwrap().to_string() ;
            let wasm_path = format!("{}/rust/fogsys/src/wasm/{}.wasm",
            dirs::home_dir().unwrap().into_os_string().into_string().unwrap()
            ,wasm_file_name).to_string();
            let nbb_clone=nbb.clone();
            tokio::spawn(async move {
              let file = File::open(&wasm_path).await;
              println!("{} {} {}",host,wasm_path,param );
              match file {                                                  
                Ok(mut readfile) => { 
                  let mut total_bytes = vec![];
                  readfile.read_to_end(&mut total_bytes).await.expect("could not read");
                  println!("{:?} {}",total_bytes,total_bytes.len() );

                    if &host!="local" {
                      let str_wasm_full = format!("GETWASM {} {} {}",myaddr,param,String::from_utf8(total_bytes).unwrap()).to_string();
                      let tx_p = nbb_clone.get(&(  host   )).unwrap() ;
                      tx_p.send(str_wasm_full).await.expect("could not send");
                    }
                    else
                    {
                      tokio::spawn(async move {
                        let param = param.parse::<i32>().unwrap();
                          println!("wasm byte from: local, func param: {}",param);
                          let store = Store::default();
                          let module = Module::from_binary(store.engine(), &total_bytes).unwrap();
                          let instance = Instance::new(&store, &module, &[]).unwrap();
                          let func = instance
                              .get_func("func")
                              .ok_or(anyhow::format_err!("failed to find function export")).unwrap()
                              .get1::<i32, i32>().unwrap();
                          let res = func(param ).unwrap();
                          println!("RESPONSE func({}) = {}", param,res );
                        });
                    }
                },                                                  
                Err(error) => {                                                    
                    panic!("Problem opening the file: {:?}", error)                
                }                                                                
                };     
              });                    
          },
          Some("RESPONSE") => { 
            println!("REMOTE RESPONSE {}",parts.next().unwrap());
          }
          _ => {      
            println!("unknown command, try again");         
          }
      }



            
        
    }


  Ok(())
}
