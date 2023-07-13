extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::net::TcpStream;
use rand::Rng;
use std::{io, thread};
use std::time::Duration;
use tungstenite::{connect, Error, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use bots::ThreadPool;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len()!=3 {
        println!("\nNumero incorreto de argumentos\nDeveria receber 2 e recebeu {}\ncargo run [numero_de_bots_desejados] [porta_desejada]\n", args.len()-1);
        process::exit(0x0100);
    }

    let arg_1 = &args[1];
    let arg_2 = &args[2];

    let number_of_bots = arg_1.trim().parse::<i32>().unwrap();
    let port_number = arg_2;

    let pool = ThreadPool::new(number_of_bots as usize);

    let mut count = 0;

    for bot_number in 0..number_of_bots {
        let (mut socket, _) = loop {
            match connect(format!("ws://127.0.0.1:{}", port_number.trim())) {
                Ok((socket, response)) => {
                    break (socket, response);
                },
                Err(_) => {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        };

        pool.execute(move || {
            //random hexadecimal
            socket.write_message(Message::Text(format!("conecta;Bot{};{};0;0", bot_number, random_hex()))).unwrap();
            let msg = socket.read_message().unwrap();
            // let info:Vec<&str> = msg.to_string().split(";").collect();
            let json =  Json::from_str(&msg.to_string()).unwrap();
            let bot_id = json.as_object().unwrap().get("id").unwrap().as_i64().unwrap();
            //let bot_id = msg.to_string().trim().parse::<i32>().unwrap();
            loop {
                next_movement(&mut socket, bot_id);
                thread::sleep(Duration::from_millis(100));
                count = count+1;
                //println!("Requisicoes realizadas pelo bot{}: {}", bot_id, count);
                let message = socket.read_message();

                match message{
                    Ok(conteudo) => {
                        let msg = conteudo.to_string();
                        let info:Vec<&str> = msg.split(":").collect();
                        //let info2:Vec<&str> = info.split(";").collect();
                        if info[0] == "{\"vencedor\"" {
                            println!("Requisicoes realizadas pelo bot{}: {}", bot_id, count);
                            break
                        }
                    }
                    Err(Error::Io(ref e)) if e.kind() == io::ErrorKind::WouldBlock => {
                      continue;
                    },
                    Err(Error::Io(ref e)) if e.kind() == io::ErrorKind::ConnectionReset => {
                      println!("Connection reset");
                    },
                    Err(Error::AlreadyClosed) => {
                      println!("Conexão encerrada");
                      break;
                    },
                    Err(Error::ConnectionClosed) => {
                      println!("Conexão encerrada");
                      break;
                    },
                    Err(e) => panic!("encountered IO error: {e}")
                  }
            }
        });
    }
    thread::sleep(Duration::from_secs(10));
}

fn next_movement(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, bot_id: i64) {
    let x = rand::thread_rng().gen_range(0..5);
    let mut direction = "";
    match x {
        0 => {
            direction = "cima"
        },

        1 => {
            direction = "baixo"
        },

        2 => {
            direction = "esquerda"
        },

        3 => {
            direction = "direita"
        },

        _ => {
        },
    }
    //println!("{}", direction);
    socket.write_message(Message::Text(format!("atualiza;{};{}", bot_id, direction).into())).unwrap();
}

fn random_hex() -> String {
    let hex = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    let mut num = String::from("#");

    let mut x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    x = rand::thread_rng().gen_range(0..16);
    num.push(hex[x]);

    return num;
}