use tokio::io::{AsyncWriteExt,AsyncReadExt};
use tokio::net::{TcpStream};
use std::env;
use std::process;
use std::time;
use std::thread;
use chrono::{DateTime, Utc};

#[macro_use]
extern crate log;

// constants
//const ECHO_SERVER_ADDRESS: &str = "192.168.2.254:1300";
/*
 quart auditioning in terminals
.\exe tcp '#$eA00000' 192.168.2.254:1300	-> not recommend
.\exe tcp "#$eA00000" 192.168.2.254:1300	-> for Linux, powershell.exe, pwsh.exe,cmd.exe
.\exe tcp #$eA00000 192.168.2.254:1300		-> not recommend
*/

#[tokio::main]
async fn main() {
    //env::set_var("RUST_LOG", "debug");
    //env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp|udp] ['Commnad'] [addr:port].");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let cmd: &str = &args[2];
    let address = &args[3];
    match protocol {
        "tcp" => {
            // connecting
            let dt: DateTime<Utc> = Utc::now();
            println!("{} : connecting to {}",dt, address);
            // connected
            if let Ok(mut stream) = TcpStream::connect(address).await {
                // connected message
                println!(
                    "{} : connected to server: {}:{}",
                    Utc::now(),
                    stream.local_addr().unwrap().ip(),
                    stream.local_addr().unwrap().port()
                );
                // set our message as hello world
                let mut message :String = cmd.to_string() ;
                message = message.replace("\r\n","");
                message = message + "\r";
                println!("{} : sent: {}", Utc::now(), message);
                let _ = stream.write_all(message.as_bytes()).await;
                // wait
                let t = 1500;
                let wt = time::Duration::from_millis(t);
                thread::sleep(wt);
                println!("{} : wait: {}", Utc::now(), t.to_string());
                // read the result
                let mut buffer = [0; 4096];
                //
                //let len = stream.read(&mut buffer).await.unwrap();
                //println!("len: {}, {}", len.to_string(), Utc::now());
                match stream.try_read(&mut buffer) {
                    Ok(n) => {
                        let mut message = String::from_utf8_lossy(&buffer[..n]).to_string();
                        message = message.replace("\r","\r\n");
                        println!("{} : received: \n\n{}", Utc::now(), &message);
                        println!("{} : read {} bytes", Utc::now(),n);
                        // FIN
                        thread::sleep(std::time::Duration::from_millis(100));
                        return;
                    }
                    Err(e) => {
                        // FIN
                        println!("{} : {}", Utc::now(), e);
                        thread::sleep(std::time::Duration::from_millis(100));
                        return;
                    }
                }

            }
        },
        "udp" => {
            // UDP
        },
        _=> {
            error! ("Please specify tcp or udp on the 1st argument.");
            process::exit(1);
        }
    }
}
