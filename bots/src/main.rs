//mod ThreadPool;

//mod ThreadPool;

use std::net::TcpStream;
use rand::Rng;
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use std::thread::Thread;
use std::time::Duration;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use bots::ThreadPool;

fn main() {
    let mut id = 0;
    let mut number_of_bots = "3".to_string();
    // let mut port_number = String::new();
    // println!("Quantos jogadores voce quer simular? ");
    // std::io::stdin().read_line(&mut number_of_bots).unwrap();
    let number_of_bots = number_of_bots.trim().parse::<i32>().unwrap();

    let pool = ThreadPool::new(number_of_bots as usize);
    // println!("Qual a porta desejada? ");
    // std::io::stdin().read_line(&mut port_number).unwrap();

    for i in 0..number_of_bots {
        let (mut socket, response) = loop {
            match connect("ws://127.0.0.1:3012") {
                Ok((mut socket, response)) => {
                    break (socket, response);
                },
                Err(_) => {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        };
        pool.execute(move || {
            socket.write_message(Message::Text(format!("conecta;{};yellow;0;0", -1))).unwrap();
            loop {
                let msg = match socket.read_message() {
                    Ok(x) => x,
                    _ => continue
                };
                println!("{}", msg);
                // let info:Vec<&str> = msg.to_string().split(";").collect();
                let botID = match msg.to_string().trim().parse::<i32>() {
                    Ok(x) => x,
                    _ => {
                        continue;
                    }
                };
                println!("Bot id: {}", botID);
                loop {
                    next_movement(&mut socket, botID);
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });
        // id += 1;
    }
    thread::sleep(Duration::from_secs(100));
}

fn next_movement(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, id: i32) {
    let mut x = rand::thread_rng().gen_range(0..5);
    let mut direction = "";
    if x == 0 { //cima
        println!("cima");
        direction = "cima";
    }
    if x == 1 { //baixo
        println!("baixo");
        direction = "baixo";
    }
    if x == 2 { //esquerda
        println!("esquerda");
        direction = "esquerda";
    }
    if x == 3 { //direita
        println!("direita");
        direction = "direita";
    }
    if x == 4 { //parado
        socket.write_message(Message::Text(format!("pinta;{};0;0", id).into())).unwrap();
        return;
    }
    socket.write_message(Message::Text(format!("atualiza;{};{}", id, direction).into())).unwrap();
    socket.write_message(Message::Text(format!("pinta;{}", id).into())).unwrap();
}