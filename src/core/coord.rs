
use tokio::sync::{mpsc, Mutex};
use crate::datastore::{ets,neighbour,app};


use std::error::Error;
use tokio::sync::watch;
use std::process::Command;


use anyhow::Result;
use wasmtime::*;


use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;



pub mod hello_world {
    tonic::include_proto!("helloworld");
}


async fn create_new_app(app_name: String) -> String {
let exec = format!("/home/vic/cpp_grpc/grpc/examples/cpp/helloworld/cmake/build/server_{}",app_name);

let mut _child = Command::new(exec)
                        .arg("")
                        .spawn()
                        .expect("failed to execute child");
let res = format!("{} deployed",app_name);
res
}





pub async fn run(myaddr: String,c1: &mut mpsc::Receiver<String>, db: ets::SimpleEts, 
nb: neighbour::Neighbour, dy_tx_p: mpsc::Sender<String> , 
apps: app::App  ) 
-> Result<(), Box<dyn Error>>  {



    let nbb=nb.clone();

    // println!("sender watch: {:?}",tx_dy_sender);
        while let Some(mesg) = c1.recv().await {
            println!("coord c1 got {:?}", mesg );
             let mut parts = mesg.splitn(2, ' ');

             match parts.next() {
               Some("SENDWASM") => {
                 //host param
                  let mut part2s =  (parts.next().unwrap()).splitn(3, ' ');
                  let host = part2s.next().unwrap().to_string();
                  let param = part2s.next().unwrap().to_string();
                  let wasm_bytes = include_bytes!("../wasm/fib.wasm");
                  let wasm_string = match String::from_utf8(wasm_bytes.to_vec()) {
                      Ok(v) => v,
                      Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                  };

                   let tx_p = nb.get(&(host   )).unwrap() ;

                          let info = format!("GETWASM {} {}",param,wasm_string);
                            

                              tx_p.send(   info.to_string()).await;
                            


               },
               Some("GETWASM") =>{
                 //param wasm_string
                let mut part2s =  (parts.next().unwrap()).splitn(2, ' ');
                let param = part2s.next().unwrap().to_string();
                let wasm_string = part2s.next().unwrap().to_string();
 let swasm_bytes =  wasm_string.as_bytes();



let wasm_bytess = include_bytes!("../wasm/fib.wasm");
    let s = match String::from_utf8(wasm_bytess.to_vec()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let swasm_bytess = s.as_bytes();


 println!("{}",wasm_string.len());
 println!("kkkk");
 println!("{}",s.len());

    let store = Store::default();
    let module = Module::from_binary(store.engine(), swasm_bytes)?;
    let instance = Instance::new(&store, &module, &[])?;

    // Invoke `gcd` export
    let func = instance
        .get_func("fib")
        .ok_or(anyhow::format_err!("failed to find `gcd` function export"))?
        .get1::<i32, i32>()?;

    println!("fib({}) = {}", param, func( param.parse::<i32>().unwrap())?);

               },


                 Some("NEWHOST") => { 
                    dy_tx_p.send(parts.next().unwrap().to_string()).await;
                  },
                  Some("CLOUDLETS") => { 

                      println!("Connected Cloudlets:");
                      nb.list();
                    // println!("{:?}",nb.get(&("hi".to_string())).unwrap());
                    // println!("LIST {:?}", nb.get(&(  parts.next().unwrap().to_string()   )).unwrap()   );
                  },
                  Some("APPS") => { 

                      println!("Available Apps:");
                      apps.list();
                    // println!("{:?}",nb.get(&("hi".to_string())).unwrap());
                    // println!("LIST {:?}", nb.get(&(  parts.next().unwrap().to_string()   )).unwrap()   );
                  },
                 Some("SEND2HOST") => { 
                    let mut part2s =  (parts.next().unwrap()).splitn(2, ' ');
                    let tx_p = nb.get(&(  part2s.next().unwrap().to_string()   )).unwrap() ;
                    tx_p.send(part2s.next().unwrap().to_string()).await;
                  },
                 Some("NEWAPP") => { 
                    let app_name = parts.next().unwrap().to_string();
                    let app_name_clone = app_name.clone();
                    let join_handle = tokio::spawn(async move {
                        create_new_app(app_name_clone).await
                    });
                    let res = join_handle.await.unwrap();
                    println!("{}",res );
                    apps.set(app_name.clone(),myaddr.clone());
                    let info = format!("UPDATEAPPS {} {}",app_name.clone(),myaddr.clone());
                    let mut tx_ps = nb.all_neighbours();
                    while let Some(tx_p) = tx_ps.pop() {
                        tx_p.send(info.to_string()).await;
                    }
  
                  },
                 Some("UPDATEAPPS") => { 
                   let mut part2s =  (parts.next().unwrap()).splitn(3, ' ');
                  //  println!("{} {}",parts.next().unwrap().to_string(),parts.next().unwrap().to_string());
                   apps.set(part2s.next().unwrap().to_string(),part2s.next().unwrap().to_string()   );

                  },
                  Some("SEND2APP") => { 
                    let mut part2s =  (parts.next().unwrap()).splitn(3, ' ');

                    let appname =  part2s.next().unwrap().to_string() ;
                    let value =  part2s.next().unwrap().to_string() ;
                    let mut remoteCaller = "none".to_string();
                          let originHost = part2s.next() ;
                          match originHost {
                            Some(inner) =>
                            {
                             
                              remoteCaller = inner.to_string().clone();
                            }
                             ,
                            None => {
                              println!("from here");
                              
                            },
                          }
                   println!("from HOST {}",remoteCaller);



                    //  println!("{} {}",parts.next().unwrap().to_string(),parts.next().unwrap().to_string());
                    let host = apps.get(&(appname) ).unwrap() ;
                    let nb_clone = nb.clone();
                    let myaddr_clone = myaddr.clone();

                    if host == myaddr {
                      println!("run here");
                      if appname == "pi".to_string() {
                            tokio::spawn(async move {
                            // Process each socket concurrently.
                            let mut client = GreeterClient::connect("http://localhost:50050").await.unwrap();
                            let request = tonic::Request::new(HelloRequest {
                            name: value.clone(),
                            });
                                   let response = client.say_hello(request).await.unwrap();
                            // println!("RESPONSE {}({})={:?}", appname,value,response.into_inner().message);
                            let resStr = response.into_inner().message.to_string();
                            // println!("RESPONSE {}({})={}", appname,value,resStr);

                            let info = format!("RESPONSE {}({})={}",appname,value,resStr);
                            
                            if remoteCaller != "none".to_string()
                            {
                               
                              let tx_p = nb_clone.get(&( remoteCaller   )).unwrap() ;
                              tx_p.send(   info.to_string()).await;
                            }
                            else{
                              println!("{}",info );
                            }
                       
                        });
                      }
                      else
                      {
                            tokio::spawn(async move {
                            // Process each socket concurrently.
                            let mut client = GreeterClient::connect("http://localhost:50051").await.unwrap();
                            let request = tonic::Request::new(HelloRequest {
                            name: value.clone(),
                            });
                            let response = client.say_hello(request).await.unwrap();
                            // println!("RESPONSE {}({})={:?}", appname,value,response.into_inner().message);
                            let resStr = response.into_inner().message.to_string();
                            // println!("RESPONSE {}({})={}", appname,value,resStr);

                            let info = format!("RESPONSE {}({})={}",appname,value,resStr);
                            
                            if remoteCaller != "none".to_string()
                            {
                               
                              let tx_p = nb_clone.get(&( remoteCaller   )).unwrap() ;
                              tx_p.send(   info.to_string()).await;
                            }
                            else{
                              println!("{}",info );
                            }
                       
                        });


                      }




                    }else{
                    let tx_p = nb.get(&(host)).unwrap() ;
                    let info = format!("SEND2APP {} {} {}",appname,value,myaddr.clone());

                    tx_p.send( info.to_string() ).await;
                    }


                  },
                   Some("RESPONSE") => { 

                     println!("RESPONSE {}",parts.next().unwrap());
                   
                   
                   }

                 _ => {               
                let wasm_string = parts.next().unwrap().to_string();
 let swasm_bytes =  wasm_string.as_bytes();

 println!("wasm byte len:{}",swasm_bytes.len());
 let store = Store::default();
    let module = Module::from_binary(store.engine(), swasm_bytes)?;
    let instance = Instance::new(&store, &module, &[])?;

    // Invoke `gcd` export
    let func = instance
        .get_func("fib")
        .ok_or(anyhow::format_err!("failed to find `gcd` function export"))?
        .get1::<i32, i32>()?;

    println!("fib({}) = {}", 40, func(40 )?);






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
                        println!("response others: {}",response);

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