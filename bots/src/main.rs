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
    let mut number_of_bots = String::new();
    let mut port_number = String::new();
    println!("Quantos jogadores voce quer simular? ");
    std::io::stdin().read_line(&mut number_of_bots).unwrap();
    let number_of_bots = number_of_bots.trim().parse::<i32>().unwrap();

    let pool = ThreadPool::new(number_of_bots as usize);
    println!("Qual a porta desejada? ");
    std::io::stdin().read_line(&mut port_number).unwrap();

    //format!("ws://127.0.0.1:{}", port_number)
    for i in 0..number_of_bots {
        let (mut socket, response) = loop {
            //match connect("ws://127.0.0.1:3012") {
            match connect(format!("ws://127.0.0.1:{}", port_number.trim())) {
                Ok((mut socket, response)) => {
                    break (socket, response);
                },
                Err(_) => {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        };
        pool.execute(move || {
            //random hexadecimal
            socket.write_message(Message::Text(format!("conecta;{};{};0;0", -1, random_hex()))).unwrap();
            let msg = socket.read_message().unwrap();
            // let info:Vec<&str> = msg.to_string().split(";").collect();
            let botID = msg.to_string().trim().parse::<i32>().unwrap();
            println!("Bot id: {}", botID);
            loop {
                next_movement(&mut socket, botID);
                thread::sleep(Duration::from_millis(100));
            }
        });
        // id += 1;
    }
    thread::sleep(Duration::from_secs(10));
}

fn next_movement(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, id: i32) {
    let mut x = rand::thread_rng().gen_range(0..5);
    let mut direction = "";
    match x {
        0 => { //cima
            println!("Cima");
            direction = "cima"
        },

        1 => { //baixo
            println!("Baixo");
            direction = "baixo"
        },

        2 => { //esquerda
            println!("Esquerda");
            direction = "esquerda"
        },

        3 => { //direita
            println!("Direita");
            direction = "direita"
        },

        _ => { //parado
            println!("Cima");
            direction = "cima"
        },
    }
    socket.write_message(Message::Text(format!("atualiza;{};{}", id, direction).into())).unwrap();
    socket.write_message(Message::Text(format!("pinta;{}", id).into())).unwrap();
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