use lazy_static::lazy_static;
#[allow(unused_imports)]
use mini_redis::S;
use volo_thrift::ResponseError;
use std::net::SocketAddr;
#[allow(unused_imports)]
use faststr::FastStr;
use clap::{ Parser, Subcommand };
use std::fmt::Debug;
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// ping the server
    Ping,
    /// set a key-value pair
    Set {
        /// key
        key: String,
        /// value
        value: String,
    },
    /// get a value by key, nil if key not exist
    Get {
        /// key
        key: String,
    },
    /// delete a key-value pair, error if key not exist
    Del {
        /// key
        key: String,
    },
    /// quit the client
    Quit,
    /// publish a message to a channel
    Publish {
        /// channel
        channel: String,
        /// message
        message: String,
    },
    /// subscribe a channel
    Subscribe {
        /// channel
        channel: String,
    },
}
lazy_static! {
    static ref CLIENT: volo_gen::mini_redis::MiniRedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::mini_redis::MiniRedisServiceClientBuilder::new("mini-redis").address(addr).build()
    };
}

#[volo::main]
async fn main() {
    loop {
        let user_input = {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input
        };
        let empty_element = std::iter::once("");
        let user_input = empty_element.chain(user_input.split_whitespace());
        let cli = Cli::try_parse_from(user_input);
        if let Err(e) = cli {
            let _ = clap::error::Error::print(&e);
            continue;
        }
        let cli = cli.unwrap();
        match cli.command {
            Commands::Ping => {
                let result = CLIENT.ping().await;
                if let Ok(status) = result {
                    output_status(status);
                } else {
                    println!("Err({:?})", result.err().unwrap());
                }
            }
            Commands::Set { key, value } => {
                let result = CLIENT.set(volo_gen::mini_redis::SetRequest {
                    key: key.into(),
                    value: value.into(),
                }).await;
                if let Ok(status) = result {
                    output_status(status);
                } else {
                    println!("Err({:?})", result.err().unwrap());
                }
            }
            Commands::Get { key } => {
                let result = CLIENT.get(volo_gen::mini_redis::GetRequest {
                    key: key.into(),
                }).await;
                let output = result.map(|v| v.value);
                output_value(output);
            }
            Commands::Del { key } => {
                let result = CLIENT.del(volo_gen::mini_redis::DelRequest {
                    key: key.into(),
                }).await;
                output_del(result);
            }
            Commands::Quit => {
                break;
            }
            Commands::Publish { channel, message } => {
                let result = CLIENT.publish(volo_gen::mini_redis::MessagePublish {
                    channel: channel.into(),
                    message: message.into(),
                }).await;
                if let Ok(status) = result {
                    output_status(status);
                } else {
                    println!("Err({:?})", result.err().unwrap());
                }
            }
            Commands::Subscribe { channel } => {
                let mut code = 0;
                loop {
                    let result = CLIENT.subscribe(volo_gen::mini_redis::MessageGet {
                        channel: channel.clone().into(),
                        code,
                    }).await;
                    if let Ok(response) = result {
                        if let Some(message) = response.message {
                            println!("message: {:?}", message);
                            code = response.code.unwrap();
                        }
                    } else {
                        println!("Err({:?})", result.err().unwrap());
                        break;
                    }
                    // sleep 1 second
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        }
    }
}

pub fn output_status(status: volo_gen::mini_redis::Status) {
    match status {
        volo_gen::mini_redis::Status::Ok => {
            println!("Ok");
        }
        volo_gen::mini_redis::Status::Error => {
            println!("Error");
        }
    }
}
pub fn output_value<T: Debug, E: Debug>(result: Result<Option<T>, E>) {
    match result {
        Ok(v) => {
            if let Some(v) = v {
                println!("{:?}", v);
            } else {
                println!("(nil)");
            }
        }
        Err(e) => {
            println!("Err({:?})", e);
        }
    }
}

pub fn output_del<T>(result: Result<volo_gen::mini_redis::Status, ResponseError<T>>) {
    match result {
        Ok(_) => {
            println!("(integer) 1");
        }
        Err(_) => {
            println!("(integer) 0");
        }
    }
}
